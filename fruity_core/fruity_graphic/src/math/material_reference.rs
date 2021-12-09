use crate::graphic_service::GraphicService;
use crate::math::Color;
use crate::resources::material_resource::MaterialResource;
use crate::Matrix4;
use crate::Vector2d;
use fruity_core::convert::FruityInto;
use fruity_core::convert::FruityTryFrom;
use fruity_core::introspect::IntrospectObject;
use fruity_core::resource::resource_reference::AnyResourceReference;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_core::serialize::serialized::Serialized;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::RwLock;

pub trait MaterialReference: IntrospectObject + SerializableObject + Debug {
    fn set_color(&self, entry_name: &str, color: Color);
    fn set_rect(&self, entry_name: &str, bottom_left: Vector2d, top_right: Vector2d);
    fn set_matrix4(&self, entry_name: &str, matrix: Matrix4);
    fn get_material(&self) -> ResourceReference<dyn MaterialResource>;
}

impl Clone for Box<dyn MaterialReference> {
    fn clone(&self) -> Self {
        let material = self.get_material();
        let graphic_service = material.resource_container.require::<dyn GraphicService>();
        let graphic_service = graphic_service.read();

        graphic_service.create_material_reference(material)
    }
}

impl FruityTryFrom<Serialized> for Box<dyn MaterialReference> {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => {
                if let Ok(value) = value
                    .clone()
                    .as_any_box()
                    .downcast::<Box<dyn MaterialReference>>()
                {
                    Ok(*value)
                } else if let Ok(value) = value
                    .clone()
                    .as_any_box()
                    .downcast::<ResourceReference<dyn MaterialResource>>()
                {
                    let graphic_service = value.resource_container.require::<dyn GraphicService>();
                    let graphic_service = graphic_service.read();

                    Ok(graphic_service.create_material_reference(*value))
                } else if let Ok(resource_reference) = value
                    .clone()
                    .as_any_box()
                    .downcast::<AnyResourceReference>()
                {
                    if let Ok(resource) = resource_reference
                        .resource
                        .as_any_arc()
                        .downcast::<RwLock<Box<dyn MaterialResource>>>()
                    {
                        let graphic_service = resource_reference
                            .resource_container
                            .require::<dyn GraphicService>();
                        let graphic_service = graphic_service.read();

                        Ok(
                            graphic_service.create_material_reference(ResourceReference::new(
                                &resource_reference.name,
                                resource,
                                resource_reference.resource_container.clone(),
                            )),
                        )
                    } else {
                        Err(format!("Couldn't convert a Serialized to native object"))
                    }
                } else {
                    Err(format!("Couldn't convert a Serialized to native object"))
                }
            }
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}

impl FruityInto<Serialized> for Box<dyn MaterialReference> {
    fn fruity_into(self) -> Serialized {
        Serialized::NativeObject(SerializableObject::duplicate(self.deref()))
    }
}
