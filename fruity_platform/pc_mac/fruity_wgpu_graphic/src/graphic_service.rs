use crate::math::material_reference::WgpuMaterialReference;
use crate::resources::material_resource::WgpuMaterialResource;
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
use fruity_graphic::math::Color;
use fruity_graphic::resources::material_resource::MaterialResource;
use fruity_graphic::resources::material_resource::MaterialResourceSettings;
use fruity_graphic::resources::mesh_resource::MeshResource;
use fruity_graphic::resources::mesh_resource::MeshResourceSettings;
use fruity_graphic::resources::shader_resource::ShaderResource;
use fruity_graphic::resources::shader_resource::ShaderResourceSettings;
use fruity_graphic::resources::texture_resource::TextureResource;
use fruity_graphic::resources::texture_resource::TextureResourceSettings;
use fruity_windows::window_service::WindowService;
use fruity_winit_windows::window_service::WinitWindowService;
use image::load_from_memory;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::iter;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::RwLock;
use tokio::runtime::Builder;
use wgpu::util::DeviceExt;
use winit::window::Window;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform(pub [[f32; 4]; 4]);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DrawIndexedIndirectArgs {
    /// The number of indices to draw.
    pub index_count: u32,
    /// The number of instances to draw.
    pub instance_count: u32,
    /// Offset into the index buffer, in indices, begin drawing from.
    pub first_index: u32,
    /// Added to each index value before indexing into the vertex buffers.
    pub base_vertex: i32,
    /// First instance to draw.
    pub first_instance: u32,
}

#[derive(Debug)]
pub struct State {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub rendering_view: wgpu::TextureView,
    pub camera_transform: RwLock<Matrix4>,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group: Arc<wgpu::BindGroup>,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct RenderInstanceIdentifier {
    mesh_identifier: String,
    material_identifier: String,
    z_index: usize,
}

#[derive(Debug)]
struct RenderInstance {
    instance_count: usize,
    instance_buffers: BTreeMap<u64, Vec<u8>>,
    mesh: ResourceReference<dyn MeshResource>,
    material: ResourceReference<dyn MaterialResource>,
}

#[derive(Debug, FruityAny)]
pub struct WgpuGraphicService {
    state: State,
    current_output: Option<wgpu::SurfaceTexture>,
    render_instances: RwLock<BTreeMap<RenderInstanceIdentifier, RenderInstance>>,
    render_bundles: RwLock<Vec<wgpu::RenderBundle>>,
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
            puffin::profile_scope!("start_draw");

            let graphic_service = resource_container_2.require::<dyn GraphicService>();
            let mut graphic_service = graphic_service.write();
            let graphic_service = graphic_service.downcast_mut::<WgpuGraphicService>();

            graphic_service.start_draw();
        });

        let resource_container_2 = resource_container.clone();
        window_service.on_end_update().add_observer(move |_| {
            puffin::profile_scope!("end_draw");

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
            render_instances: RwLock::new(BTreeMap::new()),
            render_bundles: RwLock::new(Vec::new()),
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
            let (camera_buffer, camera_bind_group) = Self::initialize_camera(&device);

            // Update state
            State {
                surface,
                device,
                queue,
                config,
                rendering_view,
                camera_transform: RwLock::new(Matrix4::identity()),
                camera_buffer,
                camera_bind_group,
            }
        };

        Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(future)
    }

    pub fn push_render_instance(
        &self,
        instance_identifier: u64,
        mut instance_buffer: Vec<u8>,
        mesh: ResourceReference<dyn MeshResource>,
        material: ResourceReference<dyn MaterialResource>,
        z_index: usize,
    ) {
        puffin::profile_function!();

        let identifier = RenderInstanceIdentifier {
            mesh_identifier: mesh.get_name(),
            material_identifier: material.get_name(),
            z_index,
        };

        let mut render_instances = self.render_instances.write().unwrap();
        if let Some(render_instance) = render_instances.get_mut(&identifier) {
            render_instance.instance_count += 1;

            if let Some(existing_instance_buffer) = render_instance
                .instance_buffers
                .get_mut(&instance_identifier)
            {
                existing_instance_buffer.append(&mut instance_buffer);
            } else {
                render_instance
                    .instance_buffers
                    .insert(instance_identifier, instance_buffer);
            }
        } else {
            let mut instance_buffers = BTreeMap::new();
            instance_buffers.insert(instance_identifier, instance_buffer);

            render_instances.insert(
                identifier,
                RenderInstance {
                    instance_count: 1,
                    instance_buffers,
                    mesh,
                    material,
                },
            );
        }
    }

    pub fn update_render_bundles(&self) {
        // We update the bundles only once per frame and not per camera per frame
        let render_instances_reader = self.render_instances.read().unwrap();
        if render_instances_reader.len() > 0 {
            let mut render_bundles = self.render_bundles.write().unwrap();

            *render_bundles = render_instances_reader
                .iter()
                .filter_map(|(_test, render_instance)| {
                    let device = self.get_device();
                    let config = self.get_config();

                    // Get resources
                    let material = render_instance.material.read();
                    let material = material.downcast_ref::<WgpuMaterialResource>();

                    let shader = if let Some(shader) = material.get_shader() {
                        shader
                    } else {
                        return None;
                    };

                    let shader = shader.read();
                    let shader = shader.downcast_ref::<WgpuShaderResource>();

                    let mesh = render_instance.mesh.read();
                    let mesh = mesh.downcast_ref::<WgpuMeshResource>();

                    // Create the instance buffer
                    // TODO: Don't do it every frame by implementing a cache system
                    let instance_buffer = render_instance
                        .instance_buffers
                        .values()
                        .flatten()
                        .map(|elem| *elem)
                        .collect::<Vec<_>>();

                    let instance_buffer =
                        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("Instance Buffer"),
                            contents: &instance_buffer,
                            usage: wgpu::BufferUsages::VERTEX,
                        });
                    let instance_count = render_instance.instance_count as u32;

                    // Render the instances
                    let mut encoder =
                        device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
                            label: Some("draw_mesh"),
                            color_formats: &[config.format],
                            depth_stencil: None,
                            sample_count: 1,
                        });
                    encoder.set_pipeline(&shader.render_pipeline);
                    material
                        .binding_groups
                        .iter()
                        .for_each(|(index, bind_group)| {
                            encoder.set_bind_group(*index, &bind_group, &[]);
                        });
                    encoder.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                    encoder.set_vertex_buffer(1, instance_buffer.slice(..));
                    encoder
                        .set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                    encoder.draw_indexed(0..mesh.index_count as u32, 0, 0..instance_count);

                    let bundle = encoder.finish(&wgpu::RenderBundleDescriptor {
                        label: Some("main"),
                    });

                    Some(bundle)
                })
                .collect::<Vec<wgpu::RenderBundle>>();
            std::mem::drop(render_instances_reader);

            let mut render_instances = self.render_instances.write().unwrap();
            render_instances.clear();
        } else {
        }
    }

    fn update_camera(&self, view_proj: Matrix4) {
        let mut camera_transform = self.state.camera_transform.write().unwrap();
        *camera_transform = view_proj.clone();
        let camera_uniform = CameraUniform(view_proj.into());
        self.state.queue.write_buffer(
            &self.state.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );
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

    pub fn get_camera_bind_group(&self) -> Arc<wgpu::BindGroup> {
        self.state.camera_bind_group.clone()
    }

    pub fn get_encoder(&self) -> Option<&RwLock<wgpu::CommandEncoder>> {
        self.current_encoder.as_ref()
    }

    fn initialize_camera(device: &wgpu::Device) -> (wgpu::Buffer, Arc<wgpu::BindGroup>) {
        let camera_uniform = CameraUniform(Matrix4::identity().into());

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                label: Some("Camera Buffer"),
            }),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("Camera Buffer"),
        });

        (camera_buffer, Arc::new(camera_bind_group))
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

        let mut render_instances = self.render_instances.write().unwrap();
        render_instances.clear();

        let mut render_bundles = self.render_bundles.write().unwrap();
        render_bundles.clear();
    }

    fn render_scene(
        &self,
        view_proj: Matrix4,
        background_color: Color,
        target: Option<ResourceReference<dyn TextureResource>>,
    ) {
        puffin::profile_function!();

        self.update_camera(view_proj);

        let mut encoder = if let Some(encoder) = self.current_encoder.as_ref() {
            encoder.write().unwrap()
        } else {
            return;
        };

        let rendering_view = target
            .as_ref()
            .map(|texture| {
                let texture = texture.read();
                let texture = texture.downcast_ref::<WgpuTextureResource>();

                // TODO: Try to find a way to remove that
                let value = unsafe {
                    std::mem::transmute::<&wgpu::TextureView, &wgpu::TextureView>(&texture.view)
                };

                value
            })
            .unwrap_or(&self.state.rendering_view);

        let mut render_pass = {
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: rendering_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: background_color.r as f64,
                            g: background_color.g as f64,
                            b: background_color.b as f64,
                            a: background_color.a as f64,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            })
        };

        // Render the instances bundles
        self.update_render_bundles();

        let render_bundles = self.render_bundles.read().unwrap();
        render_bundles.iter().for_each(move |bundle| {
            render_pass.execute_bundles(iter::once(bundle));
        });
    }

    fn get_camera_transform(&self) -> Matrix4 {
        let camera_transform = self.state.camera_transform.read().unwrap();
        camera_transform.clone()
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

    fn draw_mesh(
        &self,
        identifier: u64,
        mesh: ResourceReference<dyn MeshResource>,
        material: &dyn MaterialReference,
        z_index: usize,
    ) {
        let material = if let Some(material) = material
            .as_any_ref()
            .downcast_ref::<WgpuMaterialReference>()
        {
            material
        } else {
            return;
        };

        let instance_buffer = material.instance_buffer.read().unwrap();

        self.push_render_instance(
            identifier,
            instance_buffer.clone(),
            mesh,
            material.get_material(),
            z_index,
        )
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

    fn create_material_resource(
        &self,
        _identifier: &str,
        params: MaterialResourceSettings,
    ) -> Result<Box<dyn MaterialResource>, String> {
        let resource = WgpuMaterialResource::new(self, &params);

        Ok(Box::new(resource))
    }

    fn create_material_reference(
        &self,
        resource_reference: ResourceReference<dyn MaterialResource>,
    ) -> Box<dyn MaterialReference> {
        Box::new(WgpuMaterialReference::new(resource_reference))
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
