use crate::math::Matrix4;
use crate::resources::texture_resource::TextureResource;
use fruity_any::*;
use fruity_core::service::service::Service;
use fruity_core::service::service_manager::ServiceManager;
use fruity_core::signal::Signal;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use fruity_windows::windows_manager::WindowsManager;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::thread::JoinHandle;
use tokio::runtime::Builder;
use wgpu::util::DeviceExt;
use winit::window::Window;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform(pub [[f32; 4]; 4]);

#[derive(Debug)]
pub struct State {
    pub surface: wgpu::Surface,
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub config: wgpu::SurfaceConfiguration,
    pub rendering_view: Arc<wgpu::TextureView>,
    pub camera_transform: Matrix4,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group_layout: wgpu::BindGroupLayout,
    pub depth_texture: Arc<RwLock<TextureResource>>,
}

#[derive(Debug, FruityAny)]
pub struct GraphicsManager {
    state: State,
    current_output: Option<wgpu::SurfaceTexture>,
    current_render_pass: Option<RenderPassService>,
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
            current_render_pass: None,
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
            let rendering_view = Arc::new(
                output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default()),
            );

            // Create camera bind group
            let (camera_buffer, camera_bind_group_layout) = Self::initialize_camera(&device);

            // Create the depth texture
            let depth_texture = Arc::new(RwLock::new(TextureResource::new_depth_texture(
                &device,
                &config,
                "depth_texture",
            )));

            // Update state
            State {
                surface,
                device: Arc::new(device),
                queue: Arc::new(queue),
                config,
                rendering_view,
                camera_transform: Matrix4::identity(),
                camera_buffer,
                camera_bind_group_layout,
                depth_texture,
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
        self.state.rendering_view = Arc::new(rendering_view);
    }

    pub fn start_pass(&mut self) {
        let device = self.state.device.clone();
        let rendering_view = self.state.rendering_view.clone();
        let depth_view = self.state.rendering_view.clone();
        let queue = self.state.queue.clone();

        self.current_render_pass = Some(RenderPassService::new(
            device,
            rendering_view,
            depth_view,
            queue,
        ));
    }

    pub fn execute_with_pass<F>(&mut self, callback: F)
    where
        F: FnOnce(&mut wgpu::RenderPass) + Send + Sync + 'static,
    {
        if let Some(render_pass) = self.current_render_pass.as_ref() {
            render_pass.call(callback);
        }
    }

    pub fn end_draw(&mut self) {
        let output = if let Some(output) = self.current_output.take() {
            output
        } else {
            return;
        };

        if let Some(render_pass) = self.current_render_pass.take() {
            render_pass.end_pass();
        }

        output.present();
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.state.config.width = width as u32;
        self.state.config.height = height as u32;

        self.state
            .surface
            .configure(&self.state.device, &self.state.config);

        self.state.depth_texture = Arc::new(RwLock::new(TextureResource::new_depth_texture(
            &self.state.device,
            &self.state.config,
            "depth_texture",
        )));
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

    pub fn get_camera_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.state.camera_bind_group_layout
    }

    pub fn with_render_pass<F>(&self, callback: F)
    where
        F: FnOnce(&mut wgpu::RenderPass) + Send + Sync + 'static,
    {
        if let Some(render_pass) = self.current_render_pass.as_ref() {
            render_pass.call(callback);
        }
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

type RenderPassCallback = dyn FnOnce(&mut wgpu::RenderPass) + Send + Sync + 'static;

struct RenderPassInstruction {
    callback: Box<RenderPassCallback>,
    notify_done_sender: mpsc::Sender<()>,
}

#[derive(Debug)]
struct RenderPassService {
    channel_sender: mpsc::SyncSender<RenderPassInstruction>,
    join_handle: JoinHandle<()>,
}

impl RenderPassService {
    pub fn new(
        device: Arc<wgpu::Device>,
        rendering_view: Arc<wgpu::TextureView>,
        depth_view: Arc<wgpu::TextureView>,
        queue: Arc<wgpu::Queue>,
    ) -> Self {
        // TODO: think about a good number for sync channel
        let (sender, receiver) = mpsc::sync_channel::<RenderPassInstruction>(10);
        let (loading_sender, loading_receiver) = mpsc::channel::<()>();

        // Create a thread that will be dedicated to the inner service
        // An event channel will be used to send instruction to the service
        let join_handle = thread::spawn(move || {
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: &rendering_view,
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
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &depth_view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: true,
                        }),
                        stencil_ops: None,
                    }),
                });
                loading_sender.send(()).unwrap();

                for received in receiver {
                    (received.callback)(&mut render_pass);
                    (received.notify_done_sender).send(()).unwrap();
                }
            }

            queue.submit(std::iter::once(encoder.finish()));
        });

        loading_receiver.recv().unwrap();

        Self {
            channel_sender: sender,
            join_handle,
        }
    }

    pub fn call<F>(&self, callback: F)
    where
        F: FnOnce(&mut wgpu::RenderPass) + Send + Sync + 'static,
    {
        let (notify_done_sender, notify_done_receiver) = mpsc::channel::<()>();

        self.channel_sender
            .send(RenderPassInstruction {
                callback: Box::new(callback),
                notify_done_sender,
            })
            .unwrap();

        notify_done_receiver.recv().unwrap()
    }

    pub fn end_pass(self) {
        std::mem::drop(self.channel_sender);
        self.join_handle.join().unwrap();
    }
}
