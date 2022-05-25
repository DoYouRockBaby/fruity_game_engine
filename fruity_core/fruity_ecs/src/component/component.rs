use crate::component::serialized_component::SerializedComponent;
use crate::entity::archetype::component_collection::ComponentCollection;
use fruity_any::*;
use fruity_core::convert::FruityTryFrom;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodCaller;
use fruity_core::introspect::MethodInfo;
use fruity_core::introspect::SetterCaller;
use fruity_core::object_factory_service::ObjectFactoryService;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_core::serialize::serialized::Serialized;
use fruity_core::serialize::Deserialize;
use fruity_core::serialize::Serialize;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;

/// An abstraction over a component, should be implemented for every component
pub trait StaticComponent {
    /// Return the class type name
    fn get_component_name() -> &'static str;
}

/// An abstraction over a component, should be implemented for every component
pub trait Component: IntrospectObject + Debug {
    /// Get a collection to store this component in the archetype
    fn get_collection(&self) -> Box<dyn ComponentCollection>;

    /// Create a new component that is a clone of self
    fn duplicate(&self) -> Box<dyn Component>;
}

impl Serialize for &dyn Component {
    fn serialize(&self) -> Option<Serialized> {
        let native_serialized =
            Serialized::NativeObject(Box::new(AnyComponent::from_box(self.duplicate())));
        let serialized = native_serialized.serialize_native_objects();
        Some(serialized)
    }
}

/// An container for a component without knowing the instancied type
#[derive(FruityAny, Debug)]
pub struct AnyComponent {
    component: Box<dyn Component>,
}

impl AnyComponent {
    /// Returns an AnyComponent
    pub fn new(component: impl Component) -> AnyComponent {
        AnyComponent {
            component: Box::new(component),
        }
    }

    /// Returns an AnyComponent
    pub fn from_box(component: Box<dyn Component>) -> AnyComponent {
        AnyComponent { component }
    }

    /// Returns an AnyComponent
    pub fn into_box(self) -> Box<dyn Component> {
        self.component
    }
}

impl Deref for AnyComponent {
    type Target = dyn Component;

    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        self.component.as_ref()
    }
}

impl FruityTryFrom<Serialized> for AnyComponent {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => {
                match value.as_any_box().downcast::<AnyComponent>() {
                    Ok(value) => Ok(*value),
                    Err(_) => Err(format!("Couldn't convert An AnyComponent to native object")),
                }
            }
            Serialized::SerializedObject { class_name, fields } => {
                let serialized_component = SerializedComponent::new(class_name, fields);
                Ok(AnyComponent::new(serialized_component))
            }
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}

impl IntrospectObject for AnyComponent {
    fn get_class_name(&self) -> String {
        self.component.get_class_name()
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        self.component
            .as_ref()
            .get_field_infos()
            .into_iter()
            .map(|field_info| FieldInfo {
                name: field_info.name,
                serializable: field_info.serializable,
                getter: Arc::new(move |this| {
                    let this = this.downcast_ref::<AnyComponent>().unwrap();
                    (field_info.getter)(this.component.as_ref().as_any_ref())
                }),
                setter: match field_info.setter {
                    SetterCaller::Const(call) => {
                        SetterCaller::Const(Arc::new(move |this, args| {
                            let this = this.downcast_ref::<AnyComponent>().unwrap();
                            call(this.component.as_ref().as_any_ref(), args)
                        }))
                    }
                    SetterCaller::Mut(call) => SetterCaller::Mut(Arc::new(move |this, args| {
                        let this = this.downcast_mut::<AnyComponent>().unwrap();
                        call(this.component.as_mut().as_any_mut(), args)
                    })),
                    SetterCaller::None => SetterCaller::None,
                },
            })
            .collect::<Vec<_>>()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        self.component
            .as_ref()
            .get_method_infos()
            .into_iter()
            .map(|method_info| MethodInfo {
                name: method_info.name,
                call: match method_info.call {
                    MethodCaller::Const(call) => {
                        MethodCaller::Const(Arc::new(move |this, args| {
                            let this = this.downcast_ref::<AnyComponent>().unwrap();
                            call(this.component.as_ref().as_any_ref(), args)
                        }))
                    }
                    MethodCaller::Mut(call) => MethodCaller::Mut(Arc::new(move |this, args| {
                        let this = this.downcast_mut::<AnyComponent>().unwrap();
                        call(this.component.as_mut().as_any_mut(), args)
                    })),
                },
            })
            .collect::<Vec<_>>()
    }
}

impl Deserialize for AnyComponent {
    type Output = Self;

    fn deserialize(serialized: &Serialized, object_factory: &ObjectFactoryService) -> Option<Self> {
        let native_serialized = serialized.deserialize_native_objects(object_factory);
        if let Serialized::NativeObject(native_object) = native_serialized {
            native_object
                .as_any_box()
                .downcast::<AnyComponent>()
                .ok()
                .map(|component| *component)
        } else if let Serialized::SerializedObject { class_name, fields } = native_serialized {
            Some(AnyComponent::new(SerializedComponent::new(
                class_name, fields,
            )))
        } else {
            None
        }
    }
}

impl SerializableObject for AnyComponent {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        let component = self.component.duplicate();
        Box::new(AnyComponent { component })
    }
}
