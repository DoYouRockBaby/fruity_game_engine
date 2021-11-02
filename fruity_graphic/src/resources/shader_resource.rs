use crate::GraphicsManager;
use fruity_any::*;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resources_manager::ResourceIdentifier;
use fruity_core::resource::resources_manager::ResourcesManager;
use fruity_core::service::service_manager::ServiceManager;
use fruity_core::settings::Settings;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use std::io::Read;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Debug, FruityAny)]
pub struct ShaderResource {
    pub shader: wgpu::ShaderModule,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

pub enum ShaderBindingVisibility {
    Vertex,
    Fragment,
}

pub enum ShaderBindingType {
    Texture,
    Sampler,
}

pub struct ShaderBinding {
    id: u32,
    visibility: ShaderBindingVisibility,
    ty: ShaderBindingType,
}

impl ShaderResource {
    fn new(
        device: &wgpu::Device,
        buffer: &str,
        label: &str,
        bindings: Vec<ShaderBinding>,
    ) -> ShaderResource {
        // Create the shader
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some(label),
            source: wgpu::ShaderSource::Wgsl(buffer.into()),
        });

        // Create the bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &bindings
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
                    },
                    count: None,
                })
                .collect::<Vec<_>>(),
            label: Some(label),
        });

        ShaderResource {
            shader,
            bind_group_layout,
        }
    }
}

impl Resource for ShaderResource {}

pub fn load_shader(
    identifier: ResourceIdentifier,
    reader: &mut dyn Read,
    settings: Settings,
    service_manager: Arc<RwLock<ServiceManager>>,
) {
    // Get the graphic manager state
    let service_manager = service_manager.read().unwrap();
    let graphics_manager = service_manager.read::<GraphicsManager>();
    let mut resources_manager = service_manager.write::<ResourcesManager>();
    let device = graphics_manager.get_device();

    // read the whole file
    let mut buffer = String::new();
    if let Err(err) = reader.read_to_string(&mut buffer) {
        log::error!("{}", err.to_string());
        return;
    }

    // Parse settings
    let bindings = settings.get::<Vec<Settings>>("bindings", Vec::new());
    let bindings = bindings
        .iter()
        .map(|params| ShaderBinding {
            id: params.get::<u32>("id", 0),
            visibility: match &params.get::<String>("visibility", String::default()) as &str {
                "vertex" => ShaderBindingVisibility::Vertex,
                "fragment" => ShaderBindingVisibility::Fragment,
                _ => ShaderBindingVisibility::Vertex,
            },
            ty: match &params.get::<String>("type", String::default()) as &str {
                "texture" => ShaderBindingType::Texture,
                "sampler" => ShaderBindingType::Sampler,
                _ => ShaderBindingType::Texture,
            },
        })
        .collect::<Vec<_>>();

    // Build the resource
    let resource = ShaderResource::new(device, &buffer, &identifier.0, bindings);

    // Store the resource
    if let Err(_) = resources_manager.add_resource(identifier.clone(), resource) {
        log::error!(
            "Couldn't add a resource cause the identifier \"{}\" already exists",
            &identifier.0
        );
        return;
    }
}

impl IntrospectObject for ShaderResource {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}
