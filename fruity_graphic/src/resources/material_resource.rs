use crate::resources::shader_resource::ShaderResource;
use crate::resources::texture_resource::TextureResource;
use crate::GraphicsManager;
use fruity_any::*;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resources_manager::ResourceIdentifier;
use fruity_core::resource::resources_manager::ResourceLoaderParams;
use fruity_core::resource::resources_manager::ResourcesManager;
use fruity_core::serialize::serialized::Serialized;
use fruity_core::service::service_guard::ServiceReadGuard;
use fruity_core::service::service_manager::ServiceManager;
use fruity_core::settings::build_settings_serialized_from_yaml;
use std::io::Read;
use std::sync::Arc;
use std::sync::RwLock;
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

#[derive(Debug, FruityAnySyncSend)]
pub struct MaterialResource {
    pub shader: Arc<ShaderResource>,
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
        let surface_config = graphics_manager.get_config().unwrap();
        let device = graphics_manager.get_device().unwrap();
        let shader = material_params.shader.clone();

        // Create the bind groups
        let bind_groups = material_params
            .binding_groups
            .into_iter()
            .map(|binding_group| {
                let bind_group = match binding_group.ty {
                    MaterialParamsBindingGroupType::Camera => {
                        let bind_group_layout =
                            graphics_manager.get_camera_bind_group_layout().unwrap();
                        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                            layout: &graphics_manager.get_camera_bind_group_layout().unwrap(),
                            entries: &[wgpu::BindGroupEntry {
                                binding: 0,
                                resource: graphics_manager
                                    .get_camera_buffer()
                                    .unwrap()
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
                                .map(|binding| match binding.ty {
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
                                })
                                .collect::<Vec<_>>(),
                            label: Some(label),
                        });

                        (bind_group, bind_group_layout)
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
            bind_groups: bind_groups
                .into_iter()
                .map(|bind_group| (bind_group.0, bind_group.1 .0))
                .collect(),
            render_pipeline,
        }
    }
}

impl Resource for MaterialResource {}

pub fn material_loader(
    resources_manager: &mut ResourcesManager,
    identifier: ResourceIdentifier,
    reader: &mut dyn Read,
    _params: ResourceLoaderParams,
    service_manager: Arc<RwLock<ServiceManager>>,
) {
    // Get the graphic manager state
    let service_manager = service_manager.read().unwrap();
    let graphics_manager = service_manager.read::<GraphicsManager>();

    // read the whole file
    let mut buffer = String::new();
    if let Err(err) = reader.read_to_string(&mut buffer) {
        log::error!("{}", err.to_string());
        return;
    }
    let docs = YamlLoader::load_from_str(&buffer).unwrap();
    let root = &docs[0];
    let params = if let Some(params) = build_settings_serialized_from_yaml(root) {
        params
    } else {
        return;
    };
    let params = if let Serialized::SerializedObject { fields, .. } = params {
        ResourceLoaderParams(fields)
    } else {
        return;
    };

    // Parse settings
    let material_settings =
        if let Some(material_settings) = build_material_params(resources_manager, &params) {
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
    params: &ResourceLoaderParams,
) -> Option<MaterialParams> {
    let shader_identifier = params.get::<String>("shader", String::default());
    let shader =
        resources_manager.get_resource::<ShaderResource>(ResourceIdentifier(shader_identifier));
    let shader = if let Some(shader) = shader.0 {
        shader
    } else {
        return None;
    };

    let binding_groups = params.get::<Vec<ResourceLoaderParams>>("binding_groups", Vec::new());
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
    params: &ResourceLoaderParams,
) -> Option<MaterialParamsBindingGroup> {
    match &params.get::<String>("type", String::default()) as &str {
        "camera" => {
            let index = params.get::<u32>("index", 0);

            Some(MaterialParamsBindingGroup {
                index,
                ty: MaterialParamsBindingGroupType::Camera,
            })
        }
        "custom" => {
            let index = params.get::<u32>("index", 0);
            let bindings = params.get::<Vec<ResourceLoaderParams>>("bindings", Vec::new());
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
    params: &ResourceLoaderParams,
) -> Option<MaterialParamsBinding> {
    match &params.get::<String>("type", String::default()) as &str {
        "texture" => {
            let texture_identifier = params.get::<String>("texture", String::default());
            let texture = resources_manager
                .get_resource::<TextureResource>(ResourceIdentifier(texture_identifier));

            if let Some(texture) = texture.0 {
                Some(MaterialParamsBindingType::Texture { texture })
            } else {
                None
            }
        }
        "sampler" => {
            let texture_identifier = params.get::<String>("texture", String::default());
            let texture = resources_manager
                .get_resource::<TextureResource>(ResourceIdentifier(texture_identifier));

            if let Some(texture) = texture.0 {
                Some(MaterialParamsBindingType::Sampler { texture })
            } else {
                None
            }
        }
        _ => None,
    }
    .map(|ty| MaterialParamsBinding {
        index: params.get::<u32>("index", 0),
        ty,
    })
}
