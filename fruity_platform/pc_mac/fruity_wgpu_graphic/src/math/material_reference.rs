use crate::resources::material_resource::InstanceField;
use crate::resources::material_resource::WgpuMaterialResource;
use crate::resources::shader_resource::WgpuShaderResource;
use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodCaller;
use fruity_core::introspect::MethodInfo;
use fruity_core::introspect::SetterCaller;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_core::utils::slice::encode_into_bytes;
use fruity_graphic::math::material_reference::MaterialReference;
use fruity_graphic::math::matrix4::Matrix4;
use fruity_graphic::math::vector2d::Vector2d;
use fruity_graphic::math::Color;
use fruity_graphic::resources::material_resource::MaterialResource;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Debug, FruityAny)]
pub struct WgpuMaterialReference {
    pub material: ResourceReference<dyn MaterialResource>,
    pub instance_buffer: RwLock<Vec<u8>>,
}

impl WgpuMaterialReference {
    pub fn new(material: ResourceReference<dyn MaterialResource>) -> Self {
        let material_reader = material.read();

        let shader = if let Some(shader) = material_reader
            .get_shader()
            .as_ref()
            .map(|shader| shader.read())
        {
            shader
        } else {
            return Self {
                material,
                instance_buffer: RwLock::new(Vec::new()),
            };
        };

        let shader = shader.downcast_ref::<WgpuShaderResource>();

        // Build instance buffer
        let mut instance_buffer = Vec::new();
        instance_buffer.resize(shader.instance_size, 0);

        WgpuMaterialReference {
            material,
            instance_buffer: RwLock::new(instance_buffer),
        }
    }
}

impl MaterialReference for WgpuMaterialReference {
    fn set_color(&self, entry_name: &str, color: Color) {
        let material = self.material.read();
        let material = material.downcast_ref::<WgpuMaterialResource>();

        if let Some(fields) = material.fields.get(entry_name) {
            fields.iter().for_each(|field| {
                match field {
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
                };
            });
        }
    }

    fn set_rect(&self, entry_name: &str, bottom_left: Vector2d, top_right: Vector2d) {
        let material = self.material.read();
        let material = material.downcast_ref::<WgpuMaterialResource>();

        if let Some(fields) = material.fields.get(entry_name) {
            fields.iter().for_each(|field| {
                match field {
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
                };
            });
        }
    }

    fn set_matrix4(&self, entry_name: &str, matrix: Matrix4) {
        let material = self.material.read();
        let material = material.downcast_ref::<WgpuMaterialResource>();

        if let Some(fields) = material.fields.get(entry_name) {
            fields.iter().for_each(|field| {
                match field {
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
                };
            });
        }
    }

    fn get_material(&self) -> ResourceReference<dyn MaterialResource> {
        self.material.clone()
    }
}

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
    type Target = ResourceReference<dyn MaterialResource>;

    fn deref(&self) -> &Self::Target {
        &self.material
    }
}
