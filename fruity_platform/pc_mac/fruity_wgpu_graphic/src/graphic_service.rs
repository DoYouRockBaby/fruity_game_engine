use crate::resources::material_resource::InstanceField;
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
use fruity_core::utils::slice::encode_into_bytes;
use fruity_core::RwLock;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic::graphic_service::MaterialParam;
use fruity_graphic::math::matrix4::Matrix4;
use fruity_graphic::math::vector2d::Vector2d;
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
use std::collections::HashMap;
use std::fmt::Debug;
use std::iter;
use std::ops::Deref;
use std::sync::Arc;
use tokio::runtime::Builder;
use wgpu::util::DeviceExt;
use winit::window::Window;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform(pub [[f32; 4]; 4]);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ViewportSizeUniform(pub [f32; 2]);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RenderSurfaceSizeUniform(pub [f32; 2]);

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
    pub viewport_size_buffer: wgpu::Buffer,
    pub viewport_size_bind_group: Arc<wgpu::BindGroup>,
    pub render_surface_size_buffer: wgpu::Buffer,
    pub render_surface_size_bind_group: Arc<wgpu::BindGroup>,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct RenderInstanceIdentifier {
    mesh_identifier: String,
    material_identifier: String,
    z_index: i32,
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
    window_service: ResourceReference<dyn WindowService>,
    current_output: Option<wgpu::SurfaceTexture>,
    render_instances: RwLock<BTreeMap<RenderInstanceIdentifier, RenderInstance>>,
    render_bundles: RwLock<Vec<wgpu::RenderBundle>>,
    current_encoder: Option<RwLock<wgpu::CommandEncoder>>,
    viewport_offset: RwLock<(u32, u32)>,
    viewport_size: RwLock<(u32, u32)>,
    pub on_before_draw_end: Signal<()>,
    pub on_after_draw_end: Signal<()>,
}

impl WgpuGraphicService {
    pub fn new(resource_container: Arc<ResourceContainer>) -> WgpuGraphicService {
        let window_service = resource_container.require::<dyn WindowService>();

        let state = {
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
            WgpuGraphicService::initialize(window_service.get_window())
        };

        // Dispatch initialized event
        let on_initialized = Signal::new();
        on_initialized.notify(());

        WgpuGraphicService {
            state,
            window_service,
            current_output: None,
            render_instances: RwLock::new(BTreeMap::new()),
            render_bundles: RwLock::new(Vec::new()),
            current_encoder: None,
            viewport_offset: Default::default(),
            viewport_size: Default::default(),
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
                    None,
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

            // Create camera bind group
            let (viewport_size_buffer, viewport_size_bind_group) =
                Self::initialize_viewport_size(&device);

            // Create camera bind group
            let (render_surface_size_buffer, render_surface_size_bind_group) =
                Self::initialize_render_surface_size(&device);

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
                viewport_size_buffer,
                viewport_size_bind_group,
                render_surface_size_buffer,
                render_surface_size_bind_group,
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
        z_index: i32,
    ) {
        puffin::profile_function!();

        let identifier = RenderInstanceIdentifier {
            mesh_identifier: mesh.get_name(),
            material_identifier: material.get_name(),
            z_index,
        };

        let mut render_instances = self.render_instances.write();
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
        let render_instances_reader = self.render_instances.read();
        if render_instances_reader.len() > 0 {
            let mut render_bundles = self.render_bundles.write();

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

            let mut render_instances = self.render_instances.write();
            render_instances.clear();
        } else {
        }
    }

    fn update_camera(&self, view_proj: Matrix4) {
        // Update camera viewproj bind group
        let mut camera_transform = self.state.camera_transform.write();
        *camera_transform = view_proj.clone();
        let camera_uniform = CameraUniform(view_proj.into());
        self.state.queue.write_buffer(
            &self.state.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );

        // Update viewport size bind group
        let screen_bottom_left = view_proj * Vector2d::new(-1.0, -1.0);
        let screen_top_right = view_proj * Vector2d::new(1.0, 1.0);
        let viewport_size = (screen_bottom_left - screen_top_right).abs();
        let viewport_size_uniform = ViewportSizeUniform([viewport_size.x, viewport_size.y]);
        self.state.queue.write_buffer(
            &self.state.viewport_size_buffer,
            0,
            bytemuck::cast_slice(&[viewport_size_uniform]),
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

    pub fn get_viewport_size_bind_group(&self) -> Arc<wgpu::BindGroup> {
        self.state.viewport_size_bind_group.clone()
    }

    pub fn get_render_surface_size_bind_group(&self) -> Arc<wgpu::BindGroup> {
        self.state.render_surface_size_bind_group.clone()
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

    fn initialize_viewport_size(device: &wgpu::Device) -> (wgpu::Buffer, Arc<wgpu::BindGroup>) {
        let viewport_size = Vector2d::default();

        let viewport_size_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Viewport Size Buffer"),
            contents: bytemuck::cast_slice(&[viewport_size]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let viewport_size_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
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
                label: Some("Viewport Size Buffer"),
            }),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: viewport_size_buffer.as_entire_binding(),
            }],
            label: Some("Viewport Size Buffer"),
        });

        (viewport_size_buffer, Arc::new(viewport_size_bind_group))
    }

    fn initialize_render_surface_size(
        device: &wgpu::Device,
    ) -> (wgpu::Buffer, Arc<wgpu::BindGroup>) {
        let render_surface_size = (u32::default(), u32::default());

        let render_surface_size_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Render Surface Size Buffer"),
                contents: bytemuck::cast_slice(&[render_surface_size.0, render_surface_size.1]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let render_surface_size_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
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
                label: Some("Render Surface Size Buffer"),
            }),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: render_surface_size_buffer.as_entire_binding(),
            }],
            label: Some("Render Surface Size Buffer"),
        });

        (
            render_surface_size_buffer,
            Arc::new(render_surface_size_bind_group),
        )
    }

    fn build_instance_buffer(
        material: &ResourceReference<dyn MaterialResource>,
        params: HashMap<String, MaterialParam>,
    ) -> Vec<u8> {
        // Get references
        let material = material.read();
        let material = material.downcast_ref::<WgpuMaterialResource>();

        // Allocate instance buffer
        let mut instance_buffer = Vec::with_capacity(material.instance_size);
        instance_buffer.resize(material.instance_size, 0);

        // Inject the values into the instance buffer
        params.into_iter().for_each(|(param_name, param)| {
            let material_fields = if let Some(material_fields) = material.fields.get(&param_name) {
                material_fields
            } else {
                return;
            };

            material_fields
                .iter()
                .for_each(|material_field| match param {
                    MaterialParam::UInt(value) => {
                        if let InstanceField::UInt { location } = material_field {
                            encode_into_bytes(
                                &mut instance_buffer,
                                location.offset,
                                location.size,
                                value,
                            );
                        }
                    }
                    MaterialParam::Int(value) => {
                        if let InstanceField::Int { location } = material_field {
                            encode_into_bytes(
                                &mut instance_buffer,
                                location.offset,
                                location.size,
                                value,
                            );
                        }
                    }
                    MaterialParam::Float(value) => {
                        if let InstanceField::Float { location } = material_field {
                            encode_into_bytes(
                                &mut instance_buffer,
                                location.offset,
                                location.size,
                                value,
                            );
                        }
                    }
                    MaterialParam::Vector2(value) => {
                        if let InstanceField::Vector2 { location } = material_field {
                            encode_into_bytes(
                                &mut instance_buffer,
                                location.offset,
                                location.size,
                                value,
                            );
                        }
                    }
                    MaterialParam::Color(value) => {
                        if let InstanceField::Vector4 { location } = material_field {
                            encode_into_bytes(
                                &mut instance_buffer,
                                location.offset,
                                location.size,
                                value,
                            );
                        }
                    }
                    MaterialParam::Rect {
                        bottom_left,
                        top_right,
                    } => {
                        if let InstanceField::Rect {
                            vec0_location,
                            vec1_location,
                        } = material_field
                        {
                            encode_into_bytes(
                                &mut instance_buffer,
                                vec0_location.offset,
                                vec0_location.size,
                                bottom_left,
                            );
                            encode_into_bytes(
                                &mut instance_buffer,
                                vec1_location.offset,
                                vec1_location.size,
                                top_right,
                            );
                        }
                    }
                    MaterialParam::Matrix4(value) => {
                        if let InstanceField::Matrix4 {
                            vec0_location,
                            vec1_location,
                            vec2_location,
                            vec3_location,
                        } = material_field
                        {
                            encode_into_bytes(
                                &mut instance_buffer,
                                vec0_location.offset,
                                vec0_location.size,
                                value.0[0],
                            );
                            encode_into_bytes(
                                &mut instance_buffer,
                                vec1_location.offset,
                                vec1_location.size,
                                value.0[1],
                            );
                            encode_into_bytes(
                                &mut instance_buffer,
                                vec2_location.offset,
                                vec2_location.size,
                                value.0[2],
                            );
                            encode_into_bytes(
                                &mut instance_buffer,
                                vec3_location.offset,
                                vec3_location.size,
                                value.0[3],
                            );
                        }
                    }
                });
        });

        instance_buffer
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
            encoder.into_inner()
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

        let mut render_instances = self.render_instances.write();
        render_instances.clear();

        let mut render_bundles = self.render_bundles.write();
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
            encoder.write()
        } else {
            return;
        };

        let (rendering_view, render_surface_size) = target
            .as_ref()
            .map(|texture| {
                let texture = texture.read();
                let texture = texture.downcast_ref::<WgpuTextureResource>();

                // TODO: Try to find a way to remove that
                let value = unsafe {
                    std::mem::transmute::<&wgpu::TextureView, &wgpu::TextureView>(&texture.view)
                };

                (value, texture.get_size())
            })
            .unwrap_or_else(|| (&self.state.rendering_view, self.get_viewport_size()));

        // Update viewport size bind group
        let render_surface_size_uniform =
            RenderSurfaceSizeUniform([render_surface_size.0 as f32, render_surface_size.1 as f32]);
        self.state.queue.write_buffer(
            &self.state.render_surface_size_buffer,
            0,
            bytemuck::cast_slice(&[render_surface_size_uniform]),
        );

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

        let render_bundles = self.render_bundles.read();
        render_bundles.iter().for_each(move |bundle| {
            render_pass.execute_bundles(iter::once(bundle));
        });
    }

    fn get_camera_transform(&self) -> Matrix4 {
        let camera_transform = self.state.camera_transform.read();
        camera_transform.clone()
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.state.config.width = width;
        self.state.config.height = height;

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
        material: ResourceReference<dyn MaterialResource>,
        params: HashMap<String, MaterialParam>,
        z_index: i32,
    ) {
        let instance_buffer = Self::build_instance_buffer(&material, params);
        self.push_render_instance(identifier, instance_buffer, mesh, material, z_index)
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

    /// Convert a position in 2d world to a position in viewport
    fn world_position_to_viewport_position(&self, pos: Vector2d) -> (u32, u32) {
        let viewport_offset = self.get_viewport_offset();
        let viewport_size = self.get_viewport_size();
        let camera_transform = self.get_camera_transform().clone();

        let viewport_pos = camera_transform * pos;

        (
            ((viewport_pos.x + 1.0) / 2.0 * viewport_size.0 as f32 + viewport_offset.0 as f32)
                as u32,
            ((viewport_pos.y - 1.0) / -2.0 * viewport_size.1 as f32 + viewport_offset.1 as f32)
                as u32,
        )
    }

    /// Convert a position in the viewport to a position in 2d world
    fn viewport_position_to_world_position(&self, x: u32, y: u32) -> Vector2d {
        let viewport_offset = self.get_viewport_offset();
        let viewport_size = self.get_viewport_size();
        let camera_transform = self.get_camera_transform().clone();

        // Transform the cursor in the engine world (especialy taking care of camera)
        let cursor_pos = Vector2d::new(
            ((x as f32 - viewport_offset.0 as f32) / viewport_size.0 as f32) * 2.0 - 1.0,
            ((y as f32 - viewport_offset.1 as f32) / viewport_size.1 as f32) * -2.0 + 1.0,
        );

        camera_transform.invert() * cursor_pos
    }

    /// Get the cursor position in the viewport, take in care the camera transform
    fn get_cursor_position(&self) -> Vector2d {
        // Get informations from the resource dependencies
        let cursor_position = {
            let window_service = self.window_service.read();
            window_service.get_cursor_position()
        };

        self.viewport_position_to_world_position(cursor_position.0, cursor_position.1)
    }

    /// Check the cursor position is in the viewport
    fn is_cursor_hover_scene(&self) -> bool {
        // Get informations from the resource dependencies
        let cursor_position = {
            let window_service = self.window_service.read();
            window_service.get_cursor_position()
        };

        let viewport_offset = self.get_viewport_offset();
        let viewport_size = self.get_viewport_size();

        let cursor_pos = Vector2d::new(
            (cursor_position.0 as f32 - viewport_offset.0 as f32) / viewport_size.0 as f32,
            (cursor_position.1 as f32 - viewport_offset.1 as f32) / viewport_size.1 as f32,
        );

        cursor_pos.x >= 0.0 && cursor_pos.x < 1.0 && cursor_pos.y >= 0.0 && cursor_pos.y < 1.0
    }

    fn get_viewport_offset(&self) -> (u32, u32) {
        let viewport_offset = self.viewport_offset.read();
        viewport_offset.clone()
    }

    fn set_viewport_offset(&self, x: u32, y: u32) {
        let mut viewport_offset = self.viewport_offset.write();
        *viewport_offset = (x, y);
    }

    fn get_viewport_size(&self) -> (u32, u32) {
        let viewport_size = self.viewport_size.read();
        viewport_size.clone()
    }

    fn set_viewport_size(&self, x: u32, y: u32) {
        let mut viewport_size = self.viewport_size.write();
        *viewport_size = (x, y);
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
