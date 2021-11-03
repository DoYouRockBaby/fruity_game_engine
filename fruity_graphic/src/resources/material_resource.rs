use crate::math::RED;
use crate::resources::shader_resource::ShaderResource;
use crate::resources::texture_resource::TextureResource;
use crate::GraphicsManager;
use fruity_any::*;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resources_manager::ResourceIdentifier;
use fruity_core::resource::resources_manager::ResourcesManager;
use fruity_core::service::service_guard::ServiceReadGuard;
use fruity_core::service::service_manager::ServiceManager;
use fruity_core::settings::build_settings_from_yaml;
use fruity_core::settings::Settings;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;
use std::sync::RwLock;
use wgpu::util::DeviceExt;
use yaml_rust::YamlLoader;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct BufferIdentifier(pub u32, pub u32);

#[derive(Debug, FruityAny)]
pub struct MaterialResource {
    pub shader: Arc<ShaderResource>,
    pub uniform_buffers: HashMap<BufferIdentifier, wgpu::Buffer>,
    pub bind_groups: Vec<(u32, wgpu::BindGroup)>,
    pub render_pipeline: wgpu::RenderPipeline,
}

struct MaterialParams {
    shader: Arc<ShaderResource>,
    binding_groups: Vec<MaterialParamsBindingGroup>,
}

struct MaterialParamsBindingGroup {
    index: u32,
    ty: MaterialParamsBindingGroupType,
}

struct MaterialParamsBinding {
    index: u32,
    ty: MaterialParamsBindingType,
}

enum MaterialParamsBindingType {
    Texture { texture: Arc<TextureResource> },
    Sampler { texture: Arc<TextureResource> },
    Uniform,
}

enum MaterialParamsBindingGroupType {
    Camera,
    Custom(Vec<MaterialParamsBinding>),
}

impl MaterialResource {
    fn new(
        label: &str,
        material_params: MaterialParams,
        graphics_manager: ServiceReadGuard<GraphicsManager>,
    ) -> MaterialResource {
        let surface_config = graphics_manager.get_config();
        let device = graphics_manager.get_device();
        let shader = material_params.shader.clone();
        let mut uniform_buffers: HashMap<BufferIdentifier, wgpu::Buffer> = HashMap::new();

        // Create the bind groups
        let bind_groups = material_params
            .binding_groups
            .into_iter()
            .map(|binding_group| {
                let bind_group = {
                    let bind_group_index = binding_group.index;
                    match binding_group.ty {
                        MaterialParamsBindingGroupType::Camera => {
                            let bind_group_layout = graphics_manager.get_camera_bind_group_layout();
                            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                                layout: &graphics_manager.get_camera_bind_group_layout(),
                                entries: &[wgpu::BindGroupEntry {
                                    binding: 0,
                                    resource: graphics_manager
                                        .get_camera_buffer()
                                        .as_entire_binding(),
                                }],
                                label: Some("camera_bind_group"),
                            });

                            (bind_group, bind_group_layout)
                        }
                        MaterialParamsBindingGroupType::Custom(bindings) => {
                            let bind_group_layout = &shader.bind_group_layout;
                            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                                layout: &shader.bind_group_layout,
                                entries: &bindings
                                    .into_iter()
                                    .map(|binding| {
                                        let buffer_identifier =
                                            BufferIdentifier(bind_group_index, binding.index);

                                        match binding.ty {
                                            MaterialParamsBindingType::Texture { texture } => {
                                                // TODO: Find a way to remove it
                                                let texture = Arc::as_ptr(&texture);
                                                let texture = unsafe { &*texture };

                                                wgpu::BindGroupEntry {
                                                    binding: binding.index,
                                                    resource: wgpu::BindingResource::TextureView(
                                                        &texture.view,
                                                    ),
                                                }
                                            }
                                            MaterialParamsBindingType::Sampler { texture } => {
                                                // TODO: Find a way to remove it
                                                let texture = Arc::as_ptr(&texture);
                                                let texture = unsafe { &*texture };

                                                wgpu::BindGroupEntry {
                                                    binding: binding.index,
                                                    resource: wgpu::BindingResource::Sampler(
                                                        &texture.sampler,
                                                    ),
                                                }
                                            }
                                            MaterialParamsBindingType::Uniform => {
                                                let color = RED;
                                                let color_buffer = device.create_buffer_init(
                                                    &wgpu::util::BufferInitDescriptor {
                                                        label: Some("Uniform Buffer"),
                                                        contents: bytemuck::cast_slice(&[
                                                            color.clone()
                                                        ]),
                                                        usage: wgpu::BufferUsages::UNIFORM
                                                            | wgpu::BufferUsages::COPY_DST,
                                                    },
                                                );

                                                uniform_buffers
                                                    .insert(buffer_identifier, color_buffer);
                                                let buffer = uniform_buffers
                                                    .get(&buffer_identifier)
                                                    .unwrap();

                                                let buffer = unsafe {
                                                    std::mem::transmute::<
                                                        &wgpu::Buffer,
                                                        &wgpu::Buffer,
                                                    >(
                                                        &buffer
                                                    )
                                                };

                                                let bind_group = wgpu::BindGroupEntry {
                                                    binding: 0,
                                                    resource: buffer.as_entire_binding(),
                                                };

                                                bind_group
                                            }
                                        }
                                    })
                                    .collect::<Vec<_>>(),
                                label: Some(label),
                            });

                            (bind_group, bind_group_layout)
                        }
                    }
                };

                (binding_group.index, bind_group)
            })
            .collect::<Vec<_>>();

        // Create the render pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(label),
                bind_group_layouts: &bind_groups
                    .iter()
                    .map(|bind_group| bind_group.1 .1)
                    .collect::<Vec<_>>(),
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &material_params.shader.shader,
                entry_point: "main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &material_params.shader.shader,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLAMPING
                clamp_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        MaterialResource {
            shader: material_params.shader,
            uniform_buffers,
            bind_groups: bind_groups
                .into_iter()
                .map(|bind_group| (bind_group.0, bind_group.1 .0))
                .collect(),
            render_pipeline,
        }
    }

    pub fn write_buffer(&self, buffer_id: &BufferIdentifier, queue: &wgpu::Queue, data: &[u8]) {
        if let Some(buffer) = self.uniform_buffers.get(buffer_id) {
            queue.write_buffer(buffer, 0, data);
        }
    }
}

impl Resource for MaterialResource {}

pub fn load_material(
    identifier: ResourceIdentifier,
    reader: &mut dyn Read,
    _settings: Settings,
    service_manager: Arc<RwLock<ServiceManager>>,
) {
    // Get the dependencies
    let service_manager = service_manager.read().unwrap();
    let graphics_manager = service_manager.read::<GraphicsManager>();
    let mut resources_manager = service_manager.write::<ResourcesManager>();

    // read the whole file
    let mut buffer = String::new();
    if let Err(err) = reader.read_to_string(&mut buffer) {
        log::error!("{}", err.to_string());
        return;
    }
    let docs = YamlLoader::load_from_str(&buffer).unwrap();
    let root = &docs[0];
    let settings = if let Some(settings) = build_settings_from_yaml(root) {
        settings
    } else {
        return;
    };

    // Parse settings
    let material_settings =
        if let Some(material_settings) = build_material_params(&resources_manager, &settings) {
            material_settings
        } else {
            return;
        };

    // Build the resource
    let resource = MaterialResource::new(&identifier.0, material_settings, graphics_manager);

    // Store the resource
    if let Err(_) = resources_manager.add_resource(identifier.clone(), resource) {
        log::error!(
            "Couldn't add a resource cause the identifier \"{}\" already exists",
            &identifier.0
        );
        return;
    }
}

fn build_material_params(
    resources_manager: &ResourcesManager,
    settings: &Settings,
) -> Option<MaterialParams> {
    let shader_identifier = settings.get::<String>("shader", String::default());
    let shader =
        resources_manager.get_resource::<ShaderResource>(ResourceIdentifier(shader_identifier));
    let shader = if let Some(shader) = shader {
        shader
    } else {
        return None;
    };

    let binding_groups = settings.get::<Vec<Settings>>("binding_groups", Vec::new());
    let binding_groups = binding_groups
        .iter()
        .filter_map(|params| build_material_bind_group_params(resources_manager, params))
        .collect::<Vec<_>>();

    Some(MaterialParams {
        shader,
        binding_groups,
    })
}

fn build_material_bind_group_params(
    resources_manager: &ResourcesManager,
    settings: &Settings,
) -> Option<MaterialParamsBindingGroup> {
    match &settings.get::<String>("type", String::default()) as &str {
        "camera" => {
            let index = settings.get::<u32>("index", 0);

            Some(MaterialParamsBindingGroup {
                index,
                ty: MaterialParamsBindingGroupType::Camera,
            })
        }
        "custom" => {
            let index = settings.get::<u32>("index", 0);
            let bindings = settings.get::<Vec<Settings>>("bindings", Vec::new());
            let bindings = bindings
                .iter()
                .filter_map(|params| build_material_bind_params(resources_manager, params))
                .collect::<Vec<_>>();

            Some(MaterialParamsBindingGroup {
                index,
                ty: MaterialParamsBindingGroupType::Custom(bindings),
            })
        }
        _ => None,
    }
}

fn build_material_bind_params(
    resources_manager: &ResourcesManager,
    params: &Settings,
) -> Option<MaterialParamsBinding> {
    match &params.get::<String>("type", String::default()) as &str {
        "texture" => {
            let texture_identifier = params.get::<String>("texture", String::default());
            let texture = resources_manager
                .get_resource::<TextureResource>(ResourceIdentifier(texture_identifier));

            if let Some(texture) = texture {
                Some(MaterialParamsBindingType::Texture { texture })
            } else {
                None
            }
        }
        "sampler" => {
            let texture_identifier = params.get::<String>("texture", String::default());
            let texture = resources_manager
                .get_resource::<TextureResource>(ResourceIdentifier(texture_identifier));

            if let Some(texture) = texture {
                Some(MaterialParamsBindingType::Sampler { texture })
            } else {
                None
            }
        }
        "uniform" => Some(MaterialParamsBindingType::Uniform),
        _ => None,
    }
    .map(|ty| MaterialParamsBinding {
        index: params.get::<u32>("index", 0),
        ty,
    })
}

impl IntrospectObject for MaterialResource {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}
