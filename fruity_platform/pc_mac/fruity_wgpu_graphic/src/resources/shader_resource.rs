use crate::wgpu_bridge::INSTANCE_DESC;
use crate::wgpu_bridge::VERTEX_DESC;
use fruity_any::*;
use fruity_core::convert::FruityInto;
use fruity_core::convert::FruityTryFrom;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::introspect::SetterCaller;
use fruity_core::resource::resource::Resource;
use fruity_graphic::resources::shader_resource::ShaderBinding;
use fruity_graphic::resources::shader_resource::ShaderBindingGroup;
use fruity_graphic::resources::shader_resource::ShaderBindingType;
use fruity_graphic::resources::shader_resource::ShaderBindingVisibility;
use fruity_graphic::resources::shader_resource::ShaderResource;
use fruity_graphic::resources::shader_resource::ShaderResourceSettings;
use std::sync::Arc;

#[derive(Debug, FruityAny)]
pub struct WgpuShaderResource {
    pub params: ShaderResourceSettings,
    pub shader_module: wgpu::ShaderModule,
    pub render_pipeline: wgpu::RenderPipeline,
    pub binding_groups_layout: Vec<wgpu::BindGroupLayout>,
}

impl WgpuShaderResource {
    pub fn new(
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
        buffer: &str,
        label: &str,
        params: &ShaderResourceSettings,
    ) -> WgpuShaderResource {
        // Create the shader
        let shader_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
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
            &shader_module,
            label,
            device,
            surface_config,
        );

        WgpuShaderResource {
            params: params.clone(),
            shader_module,
            render_pipeline,
            binding_groups_layout,
        }
    }

    fn build_render_pipeline(
        binding_groups_layout: &[wgpu::BindGroupLayout],
        shader_module: &wgpu::ShaderModule,
        label: &str,
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::RenderPipeline {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(label),
                bind_group_layouts: &binding_groups_layout.iter().collect::<Vec<_>>(),
                push_constant_ranges: &[],
            });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: "main",
                buffers: &[VERTEX_DESC.clone(), INSTANCE_DESC.clone()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
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

                match ShaderResourceSettings::fruity_try_from(value) {
                    Ok(value) => this.params = value,
                    Err(_) => {
                        log::error!("Expected a ShaderParams for property params");
                    }
                }
            })),
        }]
    }
}
