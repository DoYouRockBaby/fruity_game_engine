use crate::graphic_service::WgpuGraphicManager;
use crate::resources::shader_resource::WgpuShaderResource;
use crate::resources::texture_resource::WgpuTextureResource;
use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::introspect::SetterCaller;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_graphic::math::material_reference::MaterialReference;
use fruity_graphic::resources::material_resource::MaterialField;
use fruity_graphic::resources::material_resource::MaterialResource;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, Clone, FruityAny)]
pub struct WgpuMaterialReference {
    material: ResourceReference<MaterialResource>,
    binding_names: HashMap<String, Vec<(u32, u32)>>,
    pub binding_groups: Arc<Vec<wgpu::BindGroup>>,
}

pub enum NewMaterialReferenceError {
    NoShader,
}

impl WgpuMaterialReference {
    pub fn new(
        graphic_service: &WgpuGraphicManager,
        material: ResourceReference<MaterialResource>,
    ) -> Self {
        let material_reader = material.read();

        let device = graphic_service.get_device();
        let shader =
            if let Some(shader) = material_reader.shader.as_ref().map(|shader| shader.read()) {
                shader
            } else {
                return Self {
                    material,
                    binding_names: HashMap::new(),
                    binding_groups: Arc::new(Vec::new()),
                };
            };

        let shader = shader.downcast_ref::<WgpuShaderResource>();
        let mut binding_names = HashMap::<String, Vec<(u32, u32)>>::new();
        let mut binding_groups = HashMap::<u32, Vec<wgpu::BindGroupEntry>>::new();

        material_reader.fields.iter().for_each(|(key, fields)| {
            let bindings = fields
                .iter()
                .filter_map(|field| {
                    match field {
                        MaterialField::Texture {
                            default,
                            bind_group,
                            bind,
                        } => {
                            let default = default.read();
                            let default = default.downcast_ref::<WgpuTextureResource>();

                            // TODO: Find a way to remove it
                            let default = unsafe {
                                std::mem::transmute::<&WgpuTextureResource, &WgpuTextureResource>(
                                    default,
                                )
                            };

                            let bind_group_entry = wgpu::BindGroupEntry {
                                binding: *bind,
                                resource: wgpu::BindingResource::TextureView(&default.view),
                            };

                            if let Some(binding_group) = binding_groups.get_mut(bind_group) {
                                binding_group.push(bind_group_entry);
                            } else {
                                binding_groups.insert(*bind_group, vec![bind_group_entry]);
                            }

                            Some((*bind_group, *bind))
                        }
                        MaterialField::Sampler {
                            default,
                            bind_group,
                            bind,
                        } => {
                            let default = default.read();
                            let default = default.downcast_ref::<WgpuTextureResource>();

                            // TODO: Find a way to remove it
                            let default = unsafe {
                                std::mem::transmute::<&WgpuTextureResource, &WgpuTextureResource>(
                                    default,
                                )
                            };

                            let bind_group_entry = wgpu::BindGroupEntry {
                                binding: *bind,
                                resource: wgpu::BindingResource::Sampler(&default.sampler),
                            };

                            if let Some(binding_group) = binding_groups.get_mut(bind_group) {
                                binding_group.push(bind_group_entry);
                            } else {
                                binding_groups.insert(*bind_group, vec![bind_group_entry]);
                            }

                            Some((*bind_group, *bind))
                        }
                        MaterialField::Camera { bind_group } => {
                            let bind_group_entry = wgpu::BindGroupEntry {
                                binding: 0,
                                resource: graphic_service.get_camera_buffer().as_entire_binding(),
                            };

                            if let Some(binding_group) = binding_groups.get_mut(bind_group) {
                                binding_group.push(bind_group_entry);
                            } else {
                                binding_groups.insert(*bind_group, vec![bind_group_entry]);
                            }

                            None
                        }
                    }
                })
                .collect::<Vec<_>>();

            binding_names.insert(key.clone(), bindings);
        });

        let binding_groups = binding_groups
            .into_iter()
            .map(|(bind_group, entries)| {
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &shader
                        .binding_groups_layout
                        .get(bind_group as usize)
                        .unwrap(),
                    entries: &entries,
                    label: Some("camera_bind_group"),
                })
            })
            .collect::<Vec<_>>();

        WgpuMaterialReference {
            material,
            binding_names,
            binding_groups: Arc::new(binding_groups),
        }
    }
}

impl IntrospectObject for WgpuMaterialReference {
    fn get_class_name(&self) -> String {
        let component = self.read();
        component.get_class_name()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        let component = self.read();
        component
            .get_field_infos()
            .into_iter()
            .map(|field_info| {
                let getter = field_info.getter.clone();
                let setter = field_info.setter.clone();

                FieldInfo {
                    name: field_info.name,
                    serializable: field_info.serializable,
                    getter: Arc::new(move |this| {
                        let this = this.downcast_ref::<WgpuMaterialReference>().unwrap();
                        let reader = this.read();

                        getter(reader.as_any_ref())
                    }),
                    setter: match setter {
                        SetterCaller::Const(call) => {
                            SetterCaller::Const(Arc::new(move |this, args| {
                                let this = this.downcast_ref::<WgpuMaterialReference>().unwrap();
                                let reader = this.read();

                                call(reader.as_any_ref(), args)
                            }))
                        }
                        SetterCaller::Mut(call) => {
                            SetterCaller::Const(Arc::new(move |this, args| {
                                let this = this.downcast_ref::<WgpuMaterialReference>().unwrap();
                                let mut writer = this.write();

                                call(writer.as_any_mut(), args)
                            }))
                        }
                        SetterCaller::None => SetterCaller::None,
                    },
                }
            })
            .collect::<Vec<_>>()
    }
}

impl MaterialReference for WgpuMaterialReference {
    fn duplicate(&self) -> Box<dyn MaterialReference> {
        Box::new(self.clone())
    }
}

impl SerializableObject for WgpuMaterialReference {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

impl Deref for WgpuMaterialReference {
    type Target = ResourceReference<MaterialResource>;

    fn deref(&self) -> &Self::Target {
        &self.material
    }
}
