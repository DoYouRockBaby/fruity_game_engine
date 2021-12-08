use crate::math::material_reference::WgpuMaterialReference;
use crate::resources::mesh_resource::WgpuMeshResource;
use crate::resources::shader_resource::WgpuShaderResource;
use crate::resources::texture_resource::WgpuTextureResource;
use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::signal::Signal;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic::math::material_reference::MaterialReference;
use fruity_graphic::math::matrix4::Matrix4;
use fruity_graphic::resources::material_resource::MaterialResource;
use fruity_graphic::resources::mesh_resource::MeshResource;
use fruity_graphic::resources::mesh_resource::MeshResourceSettings;
use fruity_graphic::resources::shader_resource::ShaderResource;
use fruity_graphic::resources::shader_resource::ShaderResourceSettings;
use fruity_graphic::resources::texture_resource::TextureResource;
use fruity_graphic::resources::texture_resource::TextureResourceSettings;
use fruity_windows::window_service::WindowService;
use fruity_winit_windows::window_service::WinitWindowService;
use image::load_from_memory;
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
}

#[derive(Debug)]
struct RenderBundleEntry {
    bundle: RenderBundle,
    z_index: usize,
}

#[derive(Debug, FruityAny)]
pub struct WgpuGraphicService {
    state: State,
    current_output: Option<wgpu::SurfaceTexture>,
    render_bundle_queue: Mutex<Vec<RenderBundleEntry>>,
    current_encoder: Option<RwLock<wgpu::CommandEncoder>>,
    pub on_before_draw_end: Signal<()>,
    pub on_after_draw_end: Signal<()>,
}

impl WgpuGraphicService {
    pub fn new(resource_container: Arc<ResourceContainer>) -> WgpuGraphicService {
        let window_service = resource_container.require::<dyn WindowService>();
        let window_service = window_service.read();
        let window_service = window_service.downcast_ref::<WinitWindowService>();

        // Subscribe to windows observer to proceed the graphics when it's neededs
        let resource_container_2 = resource_container.clone();
        window_service.on_start_update().add_observer(move |_| {
            let graphic_service = resource_container_2.require::<dyn GraphicService>();
            let mut graphic_service = graphic_service.write();
            let graphic_service = graphic_service.downcast_mut::<WgpuGraphicService>();

            graphic_service.start_draw();
        });

        let resource_container_2 = resource_container.clone();
        window_service.on_end_update().add_observer(move |_| {
            let graphic_service = resource_container_2.require::<dyn GraphicService>();

            // Send the event that we will end to draw
            {
                let graphic_service = graphic_service.read();
                graphic_service.on_before_draw_end().notify(());
            }

            // End the drawing
            {
                let mut graphic_service = graphic_service.write();
                graphic_service.end_draw();
            }

            // Send the event that we finish to draw
            {
                let graphic_service = graphic_service.read();
                graphic_service.on_after_draw_end().notify(());
            }
        });

        let resource_container_2 = resource_container.clone();
        window_service
            .on_resize()
            .add_observer(move |(width, height)| {
                let graphic_service = resource_container_2.require::<dyn GraphicService>();
                let mut graphic_service = graphic_service.write();
                graphic_service.resize(*width, *height);
            });

        // Initialize the graphics
        let state = WgpuGraphicService::initialize(window_service.get_window());

        // Dispatch initialized event
        let on_initialized = Signal::new();
        on_initialized.notify(());

        WgpuGraphicService {
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
            let camera_buffer = Self::initialize_camera(&device);

            // Update state
            State {
                surface,
                device,
                queue,
                config,
                rendering_view,
                camera_transform: Matrix4::identity(),
                camera_buffer,
            }
        };

        Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(future)
    }

    pub fn push_render_bundle(&self, bundle: RenderBundle, z_index: usize) {
        let mut render_bundle_queue = self.render_bundle_queue.lock().unwrap();
        render_bundle_queue.push(RenderBundleEntry { bundle, z_index });
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

    pub fn get_camera_buffer(&self) -> &wgpu::Buffer {
        &self.state.camera_buffer
    }

    pub fn get_encoder(&self) -> Option<&RwLock<wgpu::CommandEncoder>> {
        self.current_encoder.as_ref()
    }

    fn initialize_camera(device: &wgpu::Device) -> wgpu::Buffer {
        let camera_uniform = CameraUniform(Matrix4::identity().into());

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        camera_buffer
    }
}

impl GraphicService for WgpuGraphicService {
    fn start_draw(&mut self) {
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

    fn end_draw(&mut self) {
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

    fn start_pass(&self) {
        let mut render_bundle_queue = self.render_bundle_queue.lock().unwrap();
        render_bundle_queue.clear();
    }

    fn end_pass(&self) {
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

        render_bundle_queue.iter().for_each(move |bundle| {
            render_pass.execute_bundles(iter::once(&bundle.bundle));
        });
    }

    fn update_camera(&mut self, view_proj: Matrix4) {
        self.state.camera_transform = view_proj.clone();
        let camera_uniform = CameraUniform(view_proj.into());
        self.state.queue.write_buffer(
            &self.state.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );
    }

    fn get_camera_transform(&self) -> &Matrix4 {
        &self.state.camera_transform
    }

    fn resize(&mut self, width: usize, height: usize) {
        self.state.config.width = width as u32;
        self.state.config.height = height as u32;

        self.state
            .surface
            .configure(&self.state.device, &self.state.config);
    }

    fn on_before_draw_end(&self) -> &Signal<()> {
        &self.on_before_draw_end
    }

    fn on_after_draw_end(&self) -> &Signal<()> {
        &self.on_after_draw_end
    }

    fn create_material_reference(
        &self,
        resource_reference: ResourceReference<MaterialResource>,
    ) -> Box<dyn MaterialReference> {
        Box::new(WgpuMaterialReference::new(self, resource_reference))
    }

    fn create_mesh_resource(
        &self,
        identifier: &str,
        params: MeshResourceSettings,
    ) -> Result<Box<dyn MeshResource>, String> {
        let device = self.get_device();

        let resource = WgpuMeshResource::new(device, identifier, &params);

        Ok(Box::new(resource))
    }

    fn create_shader_resource(
        &self,
        identifier: &str,
        contents: String,
        params: ShaderResourceSettings,
    ) -> Result<Box<dyn ShaderResource>, String> {
        let device = self.get_device();
        let surface_config = self.get_config();

        let resource =
            WgpuShaderResource::new(device, surface_config, &contents, identifier, &params);

        Ok(Box::new(resource))
    }

    fn create_texture_resource(
        &self,
        identifier: &str,
        contents: &[u8],
        _params: TextureResourceSettings,
    ) -> Result<Box<dyn TextureResource>, String> {
        let device = self.get_device();
        let queue = self.get_queue();

        let image = load_from_memory(contents).unwrap();
        let resource = WgpuTextureResource::from_image(device, queue, &image, Some(&identifier))?;

        Ok(Box::new(resource))
    }

    fn draw_mesh(&self, z_index: usize, mesh: &dyn MeshResource, material: &dyn MaterialReference) {
        let device = self.get_device();
        let config = self.get_config();

        // Get resources
        let (material_reference, material) = if let Some(material) = material
            .as_any_ref()
            .downcast_ref::<WgpuMaterialReference>(
        ) {
            (material, material.read())
        } else {
            return;
        };

        let shader = if let Some(shader) = &material.shader {
            shader
        } else {
            return;
        };

        let shader_reader = shader.read();
        let shader_reader = shader_reader.downcast_ref::<WgpuShaderResource>();

        // Create the main render pipeline
        let mesh = mesh
            .as_any_ref()
            .downcast_ref::<WgpuMeshResource>()
            .unwrap();

        let mut encoder =
            device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
                label: Some("draw_square_bundle"),
                color_formats: &[config.format],
                depth_stencil: None,
                sample_count: 1,
            });

        // TODO: Don't do it every frame (AKA: implements the instancied rendering)
        let instance_buffer = material_reference.instance_buffer.read().unwrap();
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: &instance_buffer,
            usage: wgpu::BufferUsages::VERTEX,
        });

        encoder.set_pipeline(&shader_reader.render_pipeline);
        material_reference
            .binding_groups
            .iter()
            .for_each(|(index, bind_group)| {
                encoder.set_bind_group(*index, &bind_group, &[]);
            });
        encoder.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        encoder.set_vertex_buffer(1, instance_buffer.slice(..));
        encoder.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        encoder.draw_indexed(0..mesh.index_count as u32, 0, 0..1);
        let bundle = encoder.finish(&wgpu::RenderBundleDescriptor {
            label: Some("main"),
        });

        self.push_render_bundle(bundle, z_index);
    }
}

impl IntrospectObject for WgpuGraphicService {
    fn get_class_name(&self) -> String {
        "GraphicService".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for WgpuGraphicService {}
