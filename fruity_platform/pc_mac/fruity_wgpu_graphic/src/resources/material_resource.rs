use crate::resources::shader_resource::WgpuShaderResource;
use crate::resources::texture_resource::WgpuTextureResource;
use crate::GraphicManager;
use crate::WgpuGraphicsManager;
use fruity_any::*;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_manager::ResourceManager;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::settings::build_settings_from_yaml;
use fruity_core::settings::Settings;
use fruity_graphic::math::RED;
use fruity_graphic::resources::material_resource::build_material_params;
use fruity_graphic::resources::material_resource::MaterialParams;
use fruity_graphic::resources::material_resource::MaterialParamsBindingGroupType;
use fruity_graphic::resources::material_resource::MaterialParamsBindingType;
use fruity_graphic::resources::material_resource::MaterialResource;
use fruity_graphic::resources::shader_resource::ShaderResource;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;
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
pub struct WgpuMaterialResource {
    pub shader: ResourceReference<dyn ShaderResource>,
    pub uniform_buffers: HashMap<BufferIdentifier, wgpu::Buffer>,
    pub bind_groups: Vec<(u32, wgpu::BindGroup)>,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl WgpuMaterialResource {
    fn new(
        label: &str,
        material_params: MaterialParams,
        graphic_manager: &WgpuGraphicsManager,
    ) -> WgpuMaterialResource {
        let surface_config = graphic_manager.get_config();
        let device = graphic_manager.get_device();
        let mut uniform_buffers: HashMap<BufferIdentifier, wgpu::Buffer> = HashMap::new();
        let shader = material_params.shader.clone();

        let shader = shader.read();
        let shader = shader.downcast_ref::<WgpuShaderResource>();

        // Create the bind groups
        let bind_groups = material_params
            .binding_groups
            .into_iter()
            .map(|binding_group| {
                let bind_group = {
                    let bind_group_index = binding_group.index;
                    match binding_group.ty {
                        MaterialParamsBindingGroupType::Camera => {
                            let bind_group_layout = graphic_manager.get_camera_bind_group_layout();
                            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                                layout: &graphic_manager.get_camera_bind_group_layout(),
                                entries: &[wgpu::BindGroupEntry {
                                    binding: 0,
                                    resource: graphic_manager
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
                                                let texture = texture.read();
                                                let texture =
                                                    texture.downcast_ref::<WgpuTextureResource>();

                                                // TODO: Find a way to remove it
                                                let texture = unsafe {
                                                    std::mem::transmute::<
                                                        &WgpuTextureResource,
                                                        &WgpuTextureResource,
                                                    >(
                                                        texture
                                                    )
                                                };

                                                wgpu::BindGroupEntry {
                                                    binding: binding.index,
                                                    resource: wgpu::BindingResource::TextureView(
                                                        &texture.view,
                                                    ),
                                                }
                                            }
                                            MaterialParamsBindingType::Sampler { texture } => {
                                                let texture = texture.read();
                                                let texture =
                                                    texture.downcast_ref::<WgpuTextureResource>();

                                                // TODO: Find a way to remove it
                                                let texture = unsafe {
                                                    std::mem::transmute::<
                                                        &WgpuTextureResource,
                                                        &WgpuTextureResource,
                                                    >(
                                                        texture
                                                    )
                                                };

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

                                                // TODO: Find a way to remove it
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
                polygon_mode: wgpu::PolygonMode::Fill,
                clamp_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                ..Default::default()
            },
        });

        WgpuMaterialResource {
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

impl MaterialResource for WgpuMaterialResource {}

impl Resource for WgpuMaterialResource {}

pub fn load_material(
    identifier: &str,
    reader: &mut dyn Read,
    _settings: Settings,
    resource_manager: Arc<ResourceManager>,
) {
    // Get the dependencies
    let graphic_manager = resource_manager.require::<dyn GraphicManager>("graphic_manager");
    let graphic_manager = graphic_manager.read();
    let graphic_manager = graphic_manager.downcast_ref::<WgpuGraphicsManager>();

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
    let material_settings = if let Some(material_settings) =
        build_material_params(&settings, resource_manager.clone())
    {
        material_settings
    } else {
        return;
    };

    // Build the resource
    let resource = WgpuMaterialResource::new(identifier, material_settings, graphic_manager);

    // Store the resource
    if let Err(_) =
        resource_manager.add::<dyn MaterialResource>(identifier.clone(), Box::new(resource))
    {
        log::error!(
            "Couldn't add a resource cause the identifier \"{}\" already exists",
            identifier
        );
        return;
    }
}

impl IntrospectObject for WgpuMaterialResource {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}
