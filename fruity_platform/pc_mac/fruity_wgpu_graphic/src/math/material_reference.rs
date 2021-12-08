use crate::graphic_service::WgpuGraphicService;
use crate::resources::shader_resource::WgpuShaderResource;
use crate::resources::texture_resource::WgpuTextureResource;
use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodCaller;
use fruity_core::introspect::MethodInfo;
use fruity_core::introspect::SetterCaller;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_core::utils::collection::insert_in_hashmap_vec;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic::math::material_reference::MaterialReference;
use fruity_graphic::math::Color;
use fruity_graphic::resources::material_resource::MaterialBinding;
use fruity_graphic::resources::material_resource::MaterialResource;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use wgpu::util::DeviceExt;

#[derive(Debug, FruityAny)]
pub struct WgpuMaterialReference {
    material: ResourceReference<MaterialResource>,
    pub binding_groups: HashMap<u32, wgpu::BindGroup>,
    pub binding_entries: HashMap<String, Vec<wgpu::BindGroupEntry<'static>>>,
    pub buffers: HashMap<String, Vec<wgpu::Buffer>>,
    is_instantiable: AtomicBool,
}

pub enum NewMaterialReferenceError {
    NoShader,
}

impl WgpuMaterialReference {
    pub fn new(
        graphic_service: &WgpuGraphicService,
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
                    binding_groups: HashMap::new(),
                    binding_entries: HashMap::new(),
                    buffers: HashMap::new(),
                    is_instantiable: AtomicBool::new(true),
                };
            };

        let shader = shader.downcast_ref::<WgpuShaderResource>();
        let mut entries_by_group = HashMap::<u32, Vec<wgpu::BindGroupEntry>>::new();
        let mut entry_names_by_group = HashMap::<u32, Vec<String>>::new();
        let mut buffers = HashMap::<String, Vec<wgpu::Buffer>>::new();

        // Build the binding entries from the configuration
        material_reader.bindings.iter().for_each(|(key, bindings)| {
            bindings.iter().for_each(|field| {
                match field {
                    MaterialBinding::Texture {
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

                        insert_in_hashmap_vec(&mut entries_by_group, *bind_group, bind_group_entry);

                        insert_in_hashmap_vec(&mut entry_names_by_group, *bind_group, key.clone());
                    }
                    MaterialBinding::Sampler {
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

                        insert_in_hashmap_vec(&mut entries_by_group, *bind_group, bind_group_entry);

                        insert_in_hashmap_vec(&mut entry_names_by_group, *bind_group, key.clone());
                    }
                    MaterialBinding::Camera { bind_group } => {
                        let bind_group_entry = wgpu::BindGroupEntry {
                            binding: 0,
                            resource: graphic_service.get_camera_buffer().as_entire_binding(),
                        };

                        insert_in_hashmap_vec(&mut entries_by_group, *bind_group, bind_group_entry);

                        insert_in_hashmap_vec(&mut entry_names_by_group, *bind_group, key.clone());
                    }
                    MaterialBinding::Color {
                        default,
                        bind_group,
                        bind,
                    } => {
                        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("Uniform Buffer"),
                            contents: bytemuck::cast_slice(&[default.clone()]),
                            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                        });

                        insert_in_hashmap_vec(&mut buffers, key.clone(), buffer);

                        // TODO: Find a way to remove it
                        let buffer = unsafe {
                            std::mem::transmute::<&wgpu::Buffer, &wgpu::Buffer>(
                                buffers.get(key).unwrap().last().unwrap(),
                            )
                        };

                        let bind_group_entry = wgpu::BindGroupEntry {
                            binding: *bind,
                            resource: buffer.as_entire_binding(),
                        };

                        insert_in_hashmap_vec(&mut entries_by_group, *bind_group, bind_group_entry);

                        insert_in_hashmap_vec(&mut entry_names_by_group, *bind_group, key.clone());
                    }
                }
            });
        });

        // Create the bind groups
        let mut binding_groups = HashMap::<u32, wgpu::BindGroup>::new();
        let mut binding_entries = HashMap::<String, Vec<wgpu::BindGroupEntry<'static>>>::new();
        entries_by_group
            .into_iter()
            .for_each(|(bind_group, entries)| {
                binding_groups.insert(
                    bind_group,
                    device.create_bind_group(&wgpu::BindGroupDescriptor {
                        layout: &shader
                            .binding_groups_layout
                            .get(bind_group as usize)
                            .unwrap(),
                        entries: &entries,
                        label: Some("camera_bind_group"),
                    }),
                );

                let group_names = entry_names_by_group.get(&bind_group).unwrap();
                entries.into_iter().enumerate().for_each(|(index, entry)| {
                    let name = group_names.get(index).unwrap();

                    // TODO: Find a way to remove it
                    let entry = unsafe {
                        std::mem::transmute::<wgpu::BindGroupEntry, wgpu::BindGroupEntry>(entry)
                    };

                    insert_in_hashmap_vec(&mut binding_entries, name.clone(), entry);
                });
            });

        WgpuMaterialReference {
            material,
            binding_groups,
            binding_entries,
            buffers,
            is_instantiable: AtomicBool::new(true),
        }
    }

    pub fn is_instantiable(&self) -> bool {
        self.is_instantiable.load(Ordering::Relaxed)
    }
}

impl MaterialReference for WgpuMaterialReference {
    fn set_color(&self, entry_name: &str, color: Color) {
        if let Some(buffers) = self.buffers.get(entry_name) {
            let graphic_service = self
                .material
                .resource_container
                .require::<dyn GraphicService>();
            let graphic_service = graphic_service.read();
            let graphic_service = graphic_service.downcast_ref::<WgpuGraphicService>();

            buffers.iter().for_each(|buffer| {
                graphic_service
                    .get_queue()
                    .write_buffer(buffer, 0, bytemuck::cast_slice(&[color]));
            });

            self.is_instantiable.store(false, Ordering::Relaxed)
        }
    }

    fn get_material(&self) -> ResourceReference<MaterialResource> {
        self.material.clone()
    }
}

// TODO: Complete that
impl IntrospectObject for WgpuMaterialReference {
    fn get_class_name(&self) -> String {
        "MaterialReference".to_string()
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        self.material
            .get_field_infos()
            .into_iter()
            .map(|field_info| FieldInfo {
                name: field_info.name,
                serializable: field_info.serializable,
                getter: Arc::new(move |this| {
                    let this = this.downcast_ref::<WgpuMaterialReference>().unwrap();

                    (field_info.getter)(this.material.as_any_ref())
                }),
                setter: match field_info.setter {
                    SetterCaller::Const(call) => {
                        SetterCaller::Const(Arc::new(move |this, args| {
                            let this = this.downcast_ref::<WgpuMaterialReference>().unwrap();

                            call(this.material.as_any_ref(), args)
                        }))
                    }
                    SetterCaller::Mut(call) => SetterCaller::Mut(Arc::new(move |this, args| {
                        let this = this.downcast_mut::<WgpuMaterialReference>().unwrap();

                        call(this.material.as_any_mut(), args)
                    })),
                    SetterCaller::None => SetterCaller::None,
                },
            })
            .collect::<Vec<_>>()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        self.material
            .get_method_infos()
            .into_iter()
            .map(|method_info| MethodInfo {
                name: method_info.name,
                call: match method_info.call {
                    MethodCaller::Const(call) => {
                        MethodCaller::Const(Arc::new(move |this, args| {
                            let this = this.downcast_ref::<WgpuMaterialReference>().unwrap();

                            call(this.material.as_any_ref(), args)
                        }))
                    }
                    MethodCaller::Mut(call) => MethodCaller::Mut(Arc::new(move |this, args| {
                        let this = this.downcast_mut::<WgpuMaterialReference>().unwrap();

                        call(this.material.as_any_mut(), args)
                    })),
                },
            })
            .collect::<Vec<_>>()
    }
}

impl SerializableObject for WgpuMaterialReference {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.material.clone())
    }
}

impl Deref for WgpuMaterialReference {
    type Target = ResourceReference<MaterialResource>;

    fn deref(&self) -> &Self::Target {
        &self.material
    }
}
