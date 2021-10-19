use crate::resources::shader_resource::ShaderResource;
use crate::resources::texture_resource::TextureResource;
use crate::GraphicsManager;
use fruity_any::*;
use fruity_ecs::resource::resource::Resource;
use fruity_ecs::resource::resources_manager::ResourceIdentifier;
use fruity_ecs::resource::resources_manager::ResourceLoaderParams;
use fruity_ecs::resource::resources_manager::ResourcesManager;
use fruity_ecs::serialize::serialized::Serialized;
use fruity_ecs::service::service_manager::ServiceManager;
use fruity_ecs::settings::build_settings_serialized_from_yaml;
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
                    format: wgpu::VertexFormat::Float32x2, // NEW!
                },
            ],
        }
    }
}

#[derive(Debug, FruityAnySyncSend)]
pub struct MaterialResource {
    pub shader: Arc<ShaderResource>,
    pub bind_group: wgpu::BindGroup,
    pub render_pipeline: wgpu::RenderPipeline,
}

pub struct MaterialBinding<'s> {
    id: u32,
    ty: MaterialBindingType<'s>,
}

pub enum MaterialBindingType<'s> {
    Texture { texture: &'s TextureResource },
    Sampler { texture: &'s TextureResource },
}

impl MaterialResource {
    pub fn new(
        device: &wgpu::Device,
        label: &str,
        shader: Arc<ShaderResource>,
        bindings: Vec<MaterialBinding>,
        surface_config: &wgpu::SurfaceConfiguration,
    ) -> MaterialResource {
        // Create the bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &shader.bind_group_layout,
            entries: &bindings
                .into_iter()
                .map(|binding| match binding.ty {
                    MaterialBindingType::Texture { texture } => wgpu::BindGroupEntry {
                        binding: binding.id,
                        resource: wgpu::BindingResource::TextureView(&texture.view),
                    },
                    MaterialBindingType::Sampler { texture } => wgpu::BindGroupEntry {
                        binding: binding.id,
                        resource: wgpu::BindingResource::Sampler(&texture.sampler),
                    },
                })
                .collect::<Vec<_>>(),
            label: Some(label),
        });

        // Create the render pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(label),
                bind_group_layouts: &[&shader.bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader.shader,
                entry_point: "main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader.shader,
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
            shader,
            bind_group,
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
    let device = graphics_manager.get_device().unwrap();
    let surface_config = graphics_manager.get_config().unwrap();

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
    let params = if let Serialized::Object { fields, .. } = params {
        ResourceLoaderParams(fields)
    } else {
        return;
    };

    // Parse settings
    let shader_identifier = params.get::<String>("shader", String::default());
    let shader =
        resources_manager.get_resource::<ShaderResource>(ResourceIdentifier(shader_identifier));
    let shader = if let Some(shader) = shader.0 {
        shader
    } else {
        return;
    };

    let bindings = params.get::<Vec<ResourceLoaderParams>>("bindings", Vec::new());
    let bindings = bindings
        .iter()
        .filter_map(|params| {
            match &params.get::<String>("type", String::default()) as &str {
                "texture" => {
                    let texture_identifier = params.get::<String>("texture", String::default());
                    let texture = resources_manager
                        .get_resource::<TextureResource>(ResourceIdentifier(texture_identifier));

                    if let Some(texture) = texture.0 {
                        let texture = Arc::as_ptr(&texture);
                        let texture = unsafe { &*texture };
                        Some(MaterialBindingType::Texture { texture })
                    } else {
                        None
                    }
                }
                "sampler" => {
                    let texture_identifier = params.get::<String>("texture", String::default());
                    let texture = resources_manager
                        .get_resource::<TextureResource>(ResourceIdentifier(texture_identifier));

                    if let Some(texture) = texture.0 {
                        let texture = Arc::as_ptr(&texture);
                        let texture = unsafe { &*texture };
                        Some(MaterialBindingType::Sampler { texture })
                    } else {
                        None
                    }
                }
                _ => None,
            }
            .map(|ty| MaterialBinding {
                id: params.get::<u32>("id", 0),
                ty,
            })
        })
        .collect::<Vec<_>>();

    // Build the resource
    let resource = MaterialResource::new(device, &identifier.0, shader, bindings, surface_config);

    // Store the resource
    if let Err(_) = resources_manager.add_resource(identifier.clone(), resource) {
        log::error!(
            "Couldn't add a resource cause the identifier \"{}\" already exists",
            &identifier.0
        );
        return;
    }
}
