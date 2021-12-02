use crate::resources::shader_resource::ShaderResource;
use crate::resources::texture_resource::TextureResource;
use fruity_any::*;
use fruity_core::convert::FruityInto;
use fruity_core::convert::FruityTryFrom;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectError;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_core::serialize::serialized::Serialized;
use fruity_core::utils::introspect::ArgumentCaster;
use fruity_ecs::*;
use std::sync::Arc;

#[derive(Debug, Clone, Default, FruityAny, IntrospectObject, InstantiableObject)]
pub struct Material {
    pub shader: Option<ResourceReference<dyn ShaderResource>>,
    pub binding_groups: Vec<BindingGroup>,
}

impl FruityTryFrom<Serialized> for Material {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => match value.as_any_box().downcast::<Material>() {
                Ok(value) => Ok(*value),
                Err(_) => Err(format!("Couldn't convert a Material to native object")),
            },
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}

impl FruityInto<Serialized> for Material {
    fn fruity_into(self) -> Serialized {
        Serialized::NativeObject(Box::new(self))
    }
}

impl SerializableObject for Material {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone, FruityAny)]
pub enum BindingGroup {
    Camera,
    Custom(Vec<Binding>),
}

impl Default for BindingGroup {
    fn default() -> Self {
        Self::Custom(Vec::new())
    }
}

pub fn camera_binding_group_constructor(
    _: Arc<ResourceContainer>,
    _: Vec<Serialized>,
) -> Result<Serialized, IntrospectError> {
    Ok(Serialized::NativeObject(Box::new(BindingGroup::Camera)))
}

pub fn custom_binding_group_constructor(
    _: Arc<ResourceContainer>,
    args: Vec<Serialized>,
) -> Result<Serialized, IntrospectError> {
    let mut caster = ArgumentCaster::new("CustomBindingGroup", args);
    let arg1 = caster.cast_next::<Vec<Binding>>()?;

    Ok(Serialized::NativeObject(Box::new(BindingGroup::Custom(
        arg1,
    ))))
}

// TODO: Complete that
impl IntrospectObject for BindingGroup {
    fn get_class_name(&self) -> String {
        "BindingGroup".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl SerializableObject for BindingGroup {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

impl FruityTryFrom<Serialized> for BindingGroup {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => {
                match value.as_any_box().downcast::<BindingGroup>() {
                    Ok(value) => Ok(*value),
                    Err(_) => Err(format!("Couldn't convert a BindingGroup to native object")),
                }
            }
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}

impl FruityInto<Serialized> for BindingGroup {
    fn fruity_into(self) -> Serialized {
        Serialized::NativeObject(Box::new(self))
    }
}

#[derive(Debug, Clone, FruityAny)]
pub enum Binding {
    Texture(ResourceReference<dyn TextureResource>),
    Sampler(ResourceReference<dyn TextureResource>),
    None,
}

impl Default for Binding {
    fn default() -> Self {
        Self::None
    }
}

pub fn texture_binding_constructor(
    _: Arc<ResourceContainer>,
    args: Vec<Serialized>,
) -> Result<Serialized, IntrospectError> {
    let mut caster = ArgumentCaster::new("TextureBinding", args);
    let arg1 = caster.cast_next::<ResourceReference<dyn TextureResource>>()?;

    Ok(Serialized::NativeObject(Box::new(Binding::Texture(arg1))))
}

pub fn sampler_binding_constructor(
    _: Arc<ResourceContainer>,
    args: Vec<Serialized>,
) -> Result<Serialized, IntrospectError> {
    let mut caster = ArgumentCaster::new("SamplerBinding", args);
    let arg1 = caster.cast_next::<ResourceReference<dyn TextureResource>>()?;

    Ok(Serialized::NativeObject(Box::new(Binding::Sampler(arg1))))
}

// TODO: Complete that
impl IntrospectObject for Binding {
    fn get_class_name(&self) -> String {
        "Binding".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl SerializableObject for Binding {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

impl FruityTryFrom<Serialized> for Binding {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => match value.as_any_box().downcast::<Binding>() {
                Ok(value) => Ok(*value),
                Err(_) => Err(format!("Couldn't convert a Binding to native object")),
            },
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}

impl FruityInto<Serialized> for Binding {
    fn fruity_into(self) -> Serialized {
        Serialized::NativeObject(Box::new(self))
    }
}
