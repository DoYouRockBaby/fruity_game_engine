use crate::math::Matrix4;
use fruity_any::*;
use fruity_core::service::service::Service;
use fruity_core::service::service_manager::ServiceManager;
use fruity_core::signal::Signal;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use fruity_windows::windows_manager::WindowsManager;
use std::fmt::Debug;
use std::iter;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use tokio::runtime::Builder;
use wgpu::util::DeviceExt;
use wgpu::RenderBundle;
use winit::window::Window;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform(pub [[f32; 4]; 4]);

#[derive(Debug)]
pub struct State {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub rendering_view: wgpu::TextureView,
    pub camera_transform: Matrix4,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group_layout: wgpu::BindGroupLayout,
}

#[derive(Debug)]
struct RenderBundleEntry {
    bundle: RenderBundle,
    z_index: usize,
}

#[derive(Debug, FruityAny)]
pub struct GraphicsManager {
    state: State,
    current_output: Option<wgpu::SurfaceTexture>,
    render_bundle_queue: Mutex<Vec<RenderBundleEntry>>,
    current_encoder: Option<RwLock<wgpu::CommandEncoder>>,
    pub on_before_draw_end: Signal<()>,
    pub on_after_draw_end: Signal<()>,
}

impl GraphicsManager {
    pub fn new(service_manager: &Arc<RwLock<ServiceManager>>) -> GraphicsManager {
        let service_manager_reader = service_manager.read().unwrap();
        let windows_manager = service_manager_reader.read::<WindowsManager>();

        // Subscribe to windows observer to proceed the graphics when it's neededs
        let service_manager_2 = service_manager.clone();
        windows_manager.on_start_update.add_observer(move |_| {
            let service_manager = service_manager_2.read().unwrap();
            let mut graphics_manager = service_manager.write::<GraphicsManager>();

            graphics_manager.start_draw();
        });

        let service_manager_2 = service_manager.clone();
        windows_manager.on_end_update.add_observer(move |_| {
            let service_manager = service_manager_2.read().unwrap();

            // Send the event that we will end to draw
            let graphics_manager = service_manager.read::<GraphicsManager>();
            graphics_manager.on_before_draw_end.notify(());
            std::mem::drop(graphics_manager);

            // End the drawing
            let mut graphics_manager = service_manager.write::<GraphicsManager>();
            graphics_manager.end_draw();
            std::mem::drop(graphics_manager);

            // Send the event that we finish to draw
            let graphics_manager = service_manager.read::<GraphicsManager>();
            graphics_manager.on_after_draw_end.notify(());
            std::mem::drop(graphics_manager);
        });

        let service_manager = service_manager.clone();
        windows_manager
            .on_resize
            .add_observer(move |(width, height)| {
                let service_manager = service_manager.read().unwrap();
                let mut graphics_manager = service_manager.write::<GraphicsManager>();

                graphics_manager.resize(*width, *height);
            });

        // Initialize the graphics
        let state = GraphicsManager::initialize(windows_manager.get_window());

        // Dispatch initialized event
        let on_initialized = Signal::new();
        on_initialized.notify(());

        GraphicsManager {
            state,
            current_output: None,
            render_bundle_queue: Mutex::new(Vec::new()),
            current_encoder: None,
            on_before_draw_end: Signal::new(),
            on_after_draw_end: Signal::new(),
        }
    }

    pub fn initialize(window: &Window) -> State {
        let future = async {
            // The instance is a handle to our GPU
            // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
            let instance = wgpu::Instance::new(wgpu::Backends::all());
            let surface = unsafe { instance.create_surface(window.deref()) };
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                })
                .await
                .unwrap();

            // Create the device and queue
            let (device, queue) = adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        features: wgpu::Features::empty(),
                        limits: wgpu::Limits::default(),
                        label: None,
                    },
                    None, // Trace path
                )
                .await
                .unwrap();

            // Base configuration for the surface
            let size = window.inner_size();
            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface.get_preferred_format(&adapter).unwrap(),
                width: size.width,
                height: size.height,
                present_mode: wgpu::PresentMode::Mailbox,
            };

            surface.configure(&device, &config);

            // Get the texture view where the scene will be rendered
            let output = surface.get_current_texture().unwrap();
            let rendering_view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            // Create camera bind group
            let (camera_buffer, camera_bind_group_layout) = Self::initialize_camera(&device);

            // Update state
            State {
                surface,
                device,
                queue,
                config,
                rendering_view,
                camera_transform: Matrix4::identity(),
                camera_buffer,
                camera_bind_group_layout,
            }
        };

        Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(future)
    }

    pub fn start_draw(&mut self) {
        // Get the texture view where the scene will be rendered
        let output = self.state.surface.get_current_texture().unwrap();
        let rendering_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.current_output = Some(output);
        self.state.rendering_view = rendering_view;

        let encoder = self
            .state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Store the handles about this frame
        self.current_encoder = Some(RwLock::new(encoder));
    }

    pub fn start_pass(&self) {
        let mut render_bundle_queue = self.render_bundle_queue.lock().unwrap();
        render_bundle_queue.clear();
    }

    pub fn push_render_bundle(&self, bundle: RenderBundle, z_index: usize) {
        let mut render_bundle_queue = self.render_bundle_queue.lock().unwrap();
        render_bundle_queue.push(RenderBundleEntry { bundle, z_index });
    }

    pub fn end_pass(&self) {
        let mut encoder = if let Some(encoder) = self.current_encoder.as_ref() {
            encoder.write().unwrap()
        } else {
            return;
        };

        let mut render_pass = {
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &self.state.rendering_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            })
        };

        let mut render_bundle_queue = self.render_bundle_queue.lock().unwrap();

        // TODO: There is probably a way to optimize that with an ordered list
        render_bundle_queue.sort_by(|a, b| a.z_index.cmp(&b.z_index));

        // TODO: Try to remove that
        let render_bundle_queue = render_bundle_queue.deref();
        let render_bundle_queue =
            unsafe { &*(render_bundle_queue as *const _) } as &Vec<RenderBundleEntry>;

        render_bundle_queue.iter().for_each(|bundle| {
            render_pass.execute_bundles(iter::once(&bundle.bundle));
        });
    }

    pub fn end_draw(&mut self) {
        let encoder = if let Some(encoder) = self.current_encoder.take() {
            encoder.into_inner().unwrap()
        } else {
            return;
        };

        let output = if let Some(output) = self.current_output.take() {
            output
        } else {
            return;
        };

        self.get_queue().submit(std::iter::once(encoder.finish()));
        output.present();
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.state.config.width = width as u32;
        self.state.config.height = height as u32;

        self.state
            .surface
            .configure(&self.state.device, &self.state.config);
    }

    pub fn get_device(&self) -> &wgpu::Device {
        &self.state.device
    }

    pub fn get_queue(&self) -> &wgpu::Queue {
        &self.state.queue
    }

    pub fn get_surface(&self) -> &wgpu::Surface {
        &self.state.surface
    }

    pub fn get_config(&self) -> &wgpu::SurfaceConfiguration {
        &self.state.config
    }

    pub fn get_rendering_view(&self) -> &wgpu::TextureView {
        &self.state.rendering_view
    }

    pub fn get_camera_transform(&self) -> &Matrix4 {
        &self.state.camera_transform
    }

    pub fn get_camera_buffer(&self) -> &wgpu::Buffer {
        &self.state.camera_buffer
    }

    pub fn get_encoder(&self) -> Option<&RwLock<wgpu::CommandEncoder>> {
        self.current_encoder.as_ref()
    }

    pub fn get_camera_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.state.camera_bind_group_layout
    }

    pub fn update_camera(&mut self, view_proj: Matrix4) {
        self.state.camera_transform = view_proj.clone();
        let camera_uniform = CameraUniform(view_proj.into());
        self.state.queue.write_buffer(
            &self.state.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );
    }

    fn initialize_camera(device: &wgpu::Device) -> (wgpu::Buffer, wgpu::BindGroupLayout) {
        let camera_uniform = CameraUniform(Matrix4::identity().into());

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        (camera_buffer, camera_bind_group_layout)
    }
}

impl IntrospectObject for GraphicsManager {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Service for GraphicsManager {}
