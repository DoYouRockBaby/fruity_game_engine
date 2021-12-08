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
use fruity_core::utils::slice::encode_into_bytes;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic::math::material_reference::MaterialReference;
use fruity_graphic::math::matrix4::Matrix4;
use fruity_graphic::math::vector2d::Vector2d;
use fruity_graphic::math::Color;
use fruity_graphic::resources::material_resource::MaterialBinding;
use fruity_graphic::resources::material_resource::MaterialInstanceAttribute;
use fruity_graphic::resources::material_resource::MaterialResource;
use fruity_graphic::resources::shader_resource::ShaderInstanceAttributeType;
use std::collections::HashMap;
use std::mem::size_of;
use std::ops::Deref;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::RwLock;
use wgpu::util::DeviceExt;

#[derive(Debug, Clone)]
pub struct BufferLocation {
    offset: usize,
    size: usize,
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

#[derive(Debug)]
pub enum WgpuMaterialReferenceField {
    BindingEntry(wgpu::BindGroupEntry<'static>),
    Buffer(wgpu::Buffer),
    Instance(InstanceField),
}

#[derive(Debug, FruityAny)]
pub struct WgpuMaterialReference {
    material: ResourceReference<MaterialResource>,
    pub binding_groups: HashMap<u32, wgpu::BindGroup>,
    pub instance_buffer: RwLock<Vec<u8>>,
    pub fields: HashMap<String, Vec<WgpuMaterialReferenceField>>,
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
                    fields: HashMap::new(),
                    instance_buffer: RwLock::new(Vec::new()),
                    is_instantiable: AtomicBool::new(true),
                };
            };

        let shader = shader.downcast_ref::<WgpuShaderResource>();
        let mut entries_by_group = HashMap::<u32, Vec<wgpu::BindGroupEntry>>::new();
        let mut entry_names_by_group = HashMap::<u32, Vec<String>>::new();
        let mut fields = HashMap::<String, Vec<WgpuMaterialReferenceField>>::new();

        // Build the binding entries from the configuration
        material_reader.bindings.iter().for_each(|(key, bindings)| {
            bindings.iter().for_each(|binding| {
                match binding {
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

                        insert_in_hashmap_vec(
                            &mut fields,
                            key.clone(),
                            WgpuMaterialReferenceField::Buffer(buffer),
                        );

                        // TODO: Find a way to remove it
                        let buffer = if let Some(WgpuMaterialReferenceField::Buffer(buffer)) =
                            fields.get(key).unwrap().last()
                        {
                            Some(buffer)
                        } else {
                            None
                        }
                        .unwrap();

                        let buffer =
                            unsafe { std::mem::transmute::<&wgpu::Buffer, &wgpu::Buffer>(buffer) };

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

        // Build instance buffer
        let mut instance_buffer = Vec::new();
        instance_buffer.resize(shader.instance_size, 0);

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
        material_reader
            .instance_attributes
            .iter()
            .for_each(|instance_attribute| match instance_attribute.1 {
                MaterialInstanceAttribute::Vector4 { location } => {
                    insert_in_hashmap_vec(
                        &mut fields,
                        instance_attribute.0.clone(),
                        WgpuMaterialReferenceField::Instance(InstanceField::Vector4 {
                            location: fields_by_locations.get(location).unwrap().clone(),
                        }),
                    );
                }
                MaterialInstanceAttribute::Rect {
                    vec0_location,
                    vec1_location,
                } => {
                    insert_in_hashmap_vec(
                        &mut fields,
                        instance_attribute.0.clone(),
                        WgpuMaterialReferenceField::Instance(InstanceField::Rect {
                            vec0_location: fields_by_locations.get(vec0_location).unwrap().clone(),
                            vec1_location: fields_by_locations.get(vec1_location).unwrap().clone(),
                        }),
                    );
                }
                MaterialInstanceAttribute::Matrix4 {
                    vec0_location,
                    vec1_location,
                    vec2_location,
                    vec3_location,
                } => {
                    insert_in_hashmap_vec(
                        &mut fields,
                        instance_attribute.0.clone(),
                        WgpuMaterialReferenceField::Instance(InstanceField::Matrix4 {
                            vec0_location: fields_by_locations.get(vec0_location).unwrap().clone(),
                            vec1_location: fields_by_locations.get(vec1_location).unwrap().clone(),
                            vec2_location: fields_by_locations.get(vec2_location).unwrap().clone(),
                            vec3_location: fields_by_locations.get(vec3_location).unwrap().clone(),
                        }),
                    );
                }
            });

        WgpuMaterialReference {
            material,
            binding_groups,
            fields,
            instance_buffer: RwLock::new(instance_buffer),
            is_instantiable: AtomicBool::new(true),
        }
    }

    pub fn is_instantiable(&self) -> bool {
        self.is_instantiable.load(Ordering::Relaxed)
    }
}

impl MaterialReference for WgpuMaterialReference {
    fn set_color(&self, entry_name: &str, color: Color) {
        if let Some(fields) = self.fields.get(entry_name) {
            fields.iter().for_each(|field| {
                match field {
                    WgpuMaterialReferenceField::Buffer(buffer) => {
                        let graphic_service = self
                            .material
                            .resource_container
                            .require::<dyn GraphicService>();
                        let graphic_service = graphic_service.read();
                        let graphic_service = graphic_service.downcast_ref::<WgpuGraphicService>();

                        graphic_service.get_queue().write_buffer(
                            buffer,
                            0,
                            bytemuck::cast_slice(&[color]),
                        );

                        self.is_instantiable.store(false, Ordering::Relaxed)
                    }
                    WgpuMaterialReferenceField::Instance(field) => match field {
                        InstanceField::Vector4 { location } => {
                            let mut instance_buffer_writer = self.instance_buffer.write().unwrap();

                            encode_into_bytes(
                                &mut instance_buffer_writer,
                                location.offset,
                                location.size,
                                color,
                            );
                        }
                        _ => (),
                    },
                    _ => (),
                };
            });
        }
    }

    fn set_rect(&self, entry_name: &str, bottom_left: Vector2d, top_right: Vector2d) {
        if let Some(fields) = self.fields.get(entry_name) {
            fields.iter().for_each(|field| {
                match field {
                    // TODO: Implements for uniform
                    WgpuMaterialReferenceField::Instance(field) => match field {
                        InstanceField::Rect {
                            vec0_location,
                            vec1_location,
                        } => {
                            let mut instance_buffer_writer = self.instance_buffer.write().unwrap();

                            encode_into_bytes(
                                &mut instance_buffer_writer,
                                vec0_location.offset,
                                vec0_location.size,
                                [bottom_left.x, bottom_left.y],
                            );

                            encode_into_bytes(
                                &mut instance_buffer_writer,
                                vec1_location.offset,
                                vec1_location.size,
                                [top_right.x, top_right.y],
                            );
                        }
                        _ => (),
                    },
                    _ => (),
                };
            });
        }
    }

    fn set_matrix4(&self, entry_name: &str, matrix: Matrix4) {
        if let Some(fields) = self.fields.get(entry_name) {
            fields.iter().for_each(|field| {
                match field {
                    WgpuMaterialReferenceField::Buffer(buffer) => {
                        let graphic_service = self
                            .material
                            .resource_container
                            .require::<dyn GraphicService>();
                        let graphic_service = graphic_service.read();
                        let graphic_service = graphic_service.downcast_ref::<WgpuGraphicService>();

                        graphic_service.get_queue().write_buffer(
                            buffer,
                            0,
                            bytemuck::cast_slice(&[matrix.0]),
                        );

                        self.is_instantiable.store(false, Ordering::Relaxed)
                    }
                    WgpuMaterialReferenceField::Instance(field) => match field {
                        InstanceField::Matrix4 {
                            vec0_location,
                            vec1_location,
                            vec2_location,
                            vec3_location,
                        } => {
                            let mut instance_buffer_writer = self.instance_buffer.write().unwrap();

                            encode_into_bytes(
                                &mut instance_buffer_writer,
                                vec0_location.offset,
                                vec0_location.size,
                                matrix.0[0],
                            );

                            encode_into_bytes(
                                &mut instance_buffer_writer,
                                vec1_location.offset,
                                vec1_location.size,
                                matrix.0[1],
                            );

                            encode_into_bytes(
                                &mut instance_buffer_writer,
                                vec2_location.offset,
                                vec2_location.size,
                                matrix.0[2],
                            );

                            encode_into_bytes(
                                &mut instance_buffer_writer,
                                vec3_location.offset,
                                vec3_location.size,
                                matrix.0[3],
                            );
                        }
                        _ => (),
                    },
                    _ => (),
                };
            });
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
