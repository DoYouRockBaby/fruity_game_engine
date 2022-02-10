use crate::resources::shader_resource::WgpuShaderResource;
use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodCaller;
use fruity_core::introspect::MethodInfo;
use fruity_core::introspect::SetterCaller;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_graphic::math::material_reference::MaterialReference;
use fruity_graphic::resources::material_resource::MaterialResource;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, FruityAny)]
pub struct WgpuMaterialReference {
    pub material: ResourceReference<dyn MaterialResource>,
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
            return Self { material };
        };

        let shader = shader.downcast_ref::<WgpuShaderResource>();

        // Build instance buffer
        let mut instance_buffer = Vec::new();
        instance_buffer.resize(shader.instance_size, 0);

        WgpuMaterialReference { material }
    }
}

impl MaterialReference for WgpuMaterialReference {
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
