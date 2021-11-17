use crate::GraphicService;
use crate::WgpuGraphicManager;
use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_graphic::resources::shader_resource::load_shader_settings;
use fruity_graphic::resources::shader_resource::ShaderBindingType;
use fruity_graphic::resources::shader_resource::ShaderBindingVisibility;
use fruity_graphic::resources::shader_resource::ShaderParams;
use fruity_graphic::resources::shader_resource::ShaderResource;
use std::io::Read;
use std::sync::Arc;

#[derive(Debug, FruityAny)]
pub struct WgpuShaderResource {
    pub shader: wgpu::ShaderModule,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl WgpuShaderResource {
    fn new(
        device: &wgpu::Device,
        buffer: &str,
        label: &str,
        params: ShaderParams,
    ) -> WgpuShaderResource {
        // Create the shader
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some(label),
            source: wgpu::ShaderSource::Wgsl(buffer.into()),
        });

        // Create the bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &params
                .bindings
                .iter()
                .map(|binding| wgpu::BindGroupLayoutEntry {
                    binding: binding.id,
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
                })
                .collect::<Vec<_>>(),
            label: Some(label),
        });

        WgpuShaderResource {
            shader,
            bind_group_layout,
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

    // read the whole file
    let mut buffer = String::new();
    if let Err(err) = reader.read_to_string(&mut buffer) {
        log::error!("{}", err.to_string());
        return;
    }

    // Parse settings
    let shader_params = load_shader_settings(&settings, resource_container.clone());

    // Build the resource
    let resource = WgpuShaderResource::new(device, &buffer, identifier, shader_params);

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
        vec![]
    }
}
