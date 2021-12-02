use crate::math::vertex::Vertex;
use crate::GraphicService;
use crate::WgpuGraphicManager;
use fruity_any::*;
use fruity_core::convert::FruityInto;
use fruity_core::convert::FruityTryFrom;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::introspect::SetterCaller;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_graphic::resources::shader_resource::load_shader_settings;
use fruity_graphic::resources::shader_resource::ShaderBinding;
use fruity_graphic::resources::shader_resource::ShaderBindingGroup;
use fruity_graphic::resources::shader_resource::ShaderBindingType;
use fruity_graphic::resources::shader_resource::ShaderBindingVisibility;
use fruity_graphic::resources::shader_resource::ShaderParams;
use fruity_graphic::resources::shader_resource::ShaderResource;
use std::io::Read;
use std::sync::Arc;

#[derive(Debug, FruityAny)]
pub struct WgpuShaderResource {
    pub params: ShaderParams,
    pub render_pipeline: wgpu::RenderPipeline,
    pub binding_groups_layout: Vec<wgpu::BindGroupLayout>,
}

impl WgpuShaderResource {
    fn new(
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
        buffer: &str,
        label: &str,
        params: &ShaderParams,
    ) -> WgpuShaderResource {
        // Create the shader
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some(label),
            source: wgpu::ShaderSource::Wgsl(buffer.into()),
        });

        let binding_groups_layout = params
            .binding_groups
            .iter()
            .map(|binding_group| Self::build_binding_group_layout(binding_group, label, device))
            .collect::<Vec<_>>();

        let render_pipeline = Self::build_render_pipeline(
            &binding_groups_layout,
            &shader,
            label,
            device,
            surface_config,
        );

        WgpuShaderResource {
            params: params.clone(),
            render_pipeline,
            binding_groups_layout,
        }
    }

    fn build_render_pipeline(
        binding_groups_layout: &[wgpu::BindGroupLayout],
        shader: &wgpu::ShaderModule,
        label: &str,
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::RenderPipeline {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &binding_groups_layout.iter().collect::<Vec<_>>(),
                push_constant_ranges: &[],
            });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
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
        })
    }

    fn build_binding_group_layout(
        binding_group: &ShaderBindingGroup,
        label: &str,
        device: &wgpu::Device,
    ) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &binding_group
                .bindings
                .iter()
                .enumerate()
                .map(|(index, binding)| Self::build_binding_group_layout_entry(index, binding))
                .collect::<Vec<_>>(),
            label: Some(label),
        })
    }

    fn build_binding_group_layout_entry(
        index: usize,
        binding: &ShaderBinding,
    ) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: index as u32,
            visibility: match binding.visibility {
                ShaderBindingVisibility::Vertex => wgpu::ShaderStages::VERTEX,
                ShaderBindingVisibility::Fragment => wgpu::ShaderStages::FRAGMENT,
            },
            ty: match binding.ty {
                ShaderBindingType::Texture => wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                ShaderBindingType::Sampler => wgpu::BindingType::Sampler {
                    comparison: false,
                    filtering: true,
                },
                ShaderBindingType::Uniform => wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            },
            count: None,
        }
    }
}

impl ShaderResource for WgpuShaderResource {}

impl Resource for WgpuShaderResource {}

pub fn load_shader(
    identifier: &str,
    reader: &mut dyn Read,
    settings: Settings,
    resource_container: Arc<ResourceContainer>,
) {
    // Get the graphic manager state
    let graphic_service = resource_container.require::<dyn GraphicService>();
    let graphic_service = graphic_service.read();
    let graphic_service = graphic_service.downcast_ref::<WgpuGraphicManager>();

    let device = graphic_service.get_device();
    let surface_config = graphic_service.get_config();

    // read the whole file
    let mut buffer = String::new();
    if let Err(err) = reader.read_to_string(&mut buffer) {
        log::error!("{}", err.to_string());
        return;
    }

    // Parse settings
    let shader_params = load_shader_settings(&settings, resource_container.clone());

    // Build the resource
    let resource =
        WgpuShaderResource::new(device, surface_config, &buffer, identifier, &shader_params);

    // Store the resource
    if let Err(_) = resource_container.add::<dyn ShaderResource>(identifier, Box::new(resource)) {
        log::error!(
            "Couldn't add a resource cause the identifier \"{}\" already exists",
            identifier
        );
        return;
    }
}

impl IntrospectObject for WgpuShaderResource {
    fn get_class_name(&self) -> String {
        "ShaderResource".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![FieldInfo {
            name: "params".to_string(),
            serializable: true,
            getter: Arc::new(|this| {
                this.downcast_ref::<WgpuShaderResource>()
                    .unwrap()
                    .params
                    .clone()
                    .fruity_into()
            }),
            setter: SetterCaller::Mut(std::sync::Arc::new(|this, value| {
                let this = this.downcast_mut::<WgpuShaderResource>().unwrap();

                match ShaderParams::fruity_try_from(value) {
                    Ok(value) => this.params = value,
                    Err(_) => {
                        log::error!("Expected a ShaderParams for property params");
                    }
                }
            })),
        }]
    }
}
