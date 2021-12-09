use crate::resources::shader_resource::WgpuShaderResource;
use crate::resources::texture_resource::WgpuTextureResource;
use crate::WgpuGraphicService;
use fruity_any::*;
use fruity_core::convert::FruityInto;
use fruity_core::convert::FruityTryFrom;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::introspect::SetterCaller;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::utils::collection::insert_in_hashmap_vec;
use fruity_graphic::resources::material_resource::MaterialResource;
use fruity_graphic::resources::material_resource::MaterialResourceSettings;
use fruity_graphic::resources::material_resource::MaterialSettingsBinding;
use fruity_graphic::resources::material_resource::MaterialSettingsInstanceAttribute;
use fruity_graphic::resources::shader_resource::ShaderInstanceAttributeType;
use fruity_graphic::resources::shader_resource::ShaderResource;
use std::collections::HashMap;
use std::mem::size_of;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct BufferLocation {
    pub offset: usize,
    pub size: usize,
}

#[derive(Debug)]
pub enum InstanceField {
    Vector4 {
        location: BufferLocation,
    },
    Rect {
        vec0_location: BufferLocation,
        vec1_location: BufferLocation,
    },
    Matrix4 {
        vec0_location: BufferLocation,
        vec1_location: BufferLocation,
        vec2_location: BufferLocation,
        vec3_location: BufferLocation,
    },
}

#[derive(Debug, FruityAny)]
pub struct WgpuMaterialResource {
    pub params: MaterialResourceSettings,
    pub binding_groups: Vec<(u32, Arc<wgpu::BindGroup>)>,
    pub fields: HashMap<String, Vec<InstanceField>>,
}

impl WgpuMaterialResource {
    pub fn new(graphic_service: &WgpuGraphicService, params: &MaterialResourceSettings) -> Self {
        let shader = if let Some(shader) = params.shader.as_ref().map(|shader| shader.read()) {
            shader
        } else {
            return Self {
                params: params.clone(),
                binding_groups: Vec::new(),
                fields: HashMap::new(),
            };
        };

        let shader = shader.downcast_ref::<WgpuShaderResource>();

        // Get the binding group
        let binding_groups = params
            .bindings
            .iter()
            .map(|binding| match binding {
                MaterialSettingsBinding::Texture { value, bind_group } => {
                    let value = value.read();
                    let value = value.downcast_ref::<WgpuTextureResource>();
                    (*bind_group, value.bind_group.clone())
                }
                MaterialSettingsBinding::Camera { bind_group } => {
                    (*bind_group, graphic_service.get_camera_bind_group())
                }
            })
            .collect::<Vec<_>>();

        // Build an association beween location and the position of datas in the buffer
        let mut current_offset = 0;
        let mut fields_by_locations = HashMap::<u32, BufferLocation>::new();
        shader
            .params
            .instance_attributes
            .iter()
            .for_each(|instance_attribute| {
                let size = match instance_attribute.ty {
                    ShaderInstanceAttributeType::Float => size_of::<f32>(),
                    ShaderInstanceAttributeType::Vector2 => size_of::<[f32; 2]>(),
                    ShaderInstanceAttributeType::Vector4 => size_of::<[f32; 4]>(),
                };

                fields_by_locations.insert(
                    instance_attribute.location,
                    BufferLocation {
                        offset: current_offset,
                        size: size,
                    },
                );

                current_offset += size;
            });

        // Insert the instance fields
        let mut fields = HashMap::<String, Vec<InstanceField>>::new();
        params
            .instance_attributes
            .iter()
            .for_each(|instance_attribute| match instance_attribute.1 {
                MaterialSettingsInstanceAttribute::Vector4 { location } => {
                    insert_in_hashmap_vec(
                        &mut fields,
                        instance_attribute.0.clone(),
                        InstanceField::Vector4 {
                            location: fields_by_locations.get(location).unwrap().clone(),
                        },
                    );
                }
                MaterialSettingsInstanceAttribute::Rect {
                    vec0_location,
                    vec1_location,
                } => {
                    insert_in_hashmap_vec(
                        &mut fields,
                        instance_attribute.0.clone(),
                        InstanceField::Rect {
                            vec0_location: fields_by_locations.get(vec0_location).unwrap().clone(),
                            vec1_location: fields_by_locations.get(vec1_location).unwrap().clone(),
                        },
                    );
                }
                MaterialSettingsInstanceAttribute::Matrix4 {
                    vec0_location,
                    vec1_location,
                    vec2_location,
                    vec3_location,
                } => {
                    insert_in_hashmap_vec(
                        &mut fields,
                        instance_attribute.0.clone(),
                        InstanceField::Matrix4 {
                            vec0_location: fields_by_locations.get(vec0_location).unwrap().clone(),
                            vec1_location: fields_by_locations.get(vec1_location).unwrap().clone(),
                            vec2_location: fields_by_locations.get(vec2_location).unwrap().clone(),
                            vec3_location: fields_by_locations.get(vec3_location).unwrap().clone(),
                        },
                    );
                }
            });

        Self {
            params: params.clone(),
            binding_groups,
            fields,
        }
    }
}

impl MaterialResource for WgpuMaterialResource {
    fn get_shader(&self) -> Option<ResourceReference<dyn ShaderResource>> {
        self.params.shader.clone()
    }
}

impl Resource for WgpuMaterialResource {}

impl IntrospectObject for WgpuMaterialResource {
    fn get_class_name(&self) -> String {
        "MaterialResource".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![FieldInfo {
            name: "params".to_string(),
            serializable: true,
            getter: Arc::new(|this| {
                this.downcast_ref::<WgpuMaterialResource>()
                    .unwrap()
                    .params
                    .clone()
                    .fruity_into()
            }),
            setter: SetterCaller::Mut(std::sync::Arc::new(|this, value| {
                let this = this.downcast_mut::<WgpuMaterialResource>().unwrap();

                match MaterialResourceSettings::fruity_try_from(value) {
                    Ok(value) => this.params = value,
                    Err(_) => {
                        log::error!("Expected a MaterialResourceSettings for property params");
                    }
                }
            })),
        }]
    }
}
