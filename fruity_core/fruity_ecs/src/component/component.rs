use crate::component::serialized_component::SerializedComponent;
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

/// A function to decode an object from byte array to an any reference
pub type ComponentDecoder = fn(buffer: &[u8]) -> &dyn Component;

/// An abstraction over a component, should be implemented for every component
pub trait StaticComponent {
    /// Return the class type name
    fn get_component_name() -> String;
}

/// An abstraction over a component, should be implemented for every component
pub trait Component: IntrospectObject + Debug {
    /// Return the size that is required to encode the object
    fn encode_size(&self) -> usize;

    /// Encode the object to a byte array
    ///
    /// # Arguments
    /// * `buffer` - The buffer where the encoder will write, should match the result of encode_size function
    ///
    fn encode(&self, buffer: &mut [u8]);

    /// Return a function to decode an object from byte array to an any reference
    fn get_decoder(&self) -> ComponentDecoder;

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

/// An container for a component read reference to inject it into the scripting language
#[derive(FruityAny, Clone, Debug)]
pub struct ReadComponent {
    component: &'static dyn Component,
}

impl ReadComponent {
    /// Returns a ReadComponent
    pub fn new(component: &dyn Component) -> Self {
        // TODO: Try to find a way to remove that
        let component = unsafe { std::mem::transmute::<&dyn Component, &dyn Component>(component) };

        Self { component }
    }
}

impl IntrospectObject for ReadComponent {
    fn get_class_name(&self) -> String {
        "ReadComponent".to_string()
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        self.component
            .get_field_infos()
            .into_iter()
            .map(|field_info| FieldInfo {
                name: field_info.name,
                serializable: field_info.serializable,
                getter: Arc::new(move |this| {
                    let this = this.downcast_ref::<ReadComponent>().unwrap();
                    (field_info.getter)(this.component.as_any_ref())
                }),
                setter: match field_info.setter {
                    SetterCaller::Const(call) => {
                        SetterCaller::Const(Arc::new(move |this, args| {
                            let this = this.downcast_ref::<ReadComponent>().unwrap();
                            call(this.component.as_any_ref(), args)
                        }))
                    }
                    SetterCaller::Mut(_) => SetterCaller::None,
                    SetterCaller::None => SetterCaller::None,
                },
            })
            .collect::<Vec<_>>()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        self.component
            .get_method_infos()
            .into_iter()
            .filter_map(|method_info| match method_info.call {
                MethodCaller::Const(call) => Some(MethodInfo {
                    name: method_info.name,
                    call: MethodCaller::Const(Arc::new(move |this, args| {
                        let this = this.downcast_ref::<ReadComponent>().unwrap();
                        call(this.component.as_any_ref(), args)
                    })),
                }),
                MethodCaller::Mut(_) => None,
            })
            .collect::<Vec<_>>()
    }
}

impl SerializableObject for ReadComponent {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

/// An container for a component read reference to inject it into the scripting language
#[derive(FruityAny, Debug)]
pub struct WriteComponent {
    component: &'static mut dyn Component,
}

impl WriteComponent {
    /// Returns a WriteComponent
    pub fn new(component: &mut dyn Component) -> Self {
        // TODO: Try to find a way to remove that
        let component =
            unsafe { std::mem::transmute::<&mut dyn Component, &mut dyn Component>(component) };

        Self { component }
    }
}

impl IntrospectObject for WriteComponent {
    fn get_class_name(&self) -> String {
        "WriteComponent".to_string()
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        self.component
            .get_field_infos()
            .into_iter()
            .map(|field_info| FieldInfo {
                name: field_info.name,
                serializable: field_info.serializable,
                getter: Arc::new(move |this| {
                    let this = this.downcast_ref::<WriteComponent>().unwrap();
                    (field_info.getter)(this.component.as_any_ref())
                }),
                setter: match field_info.setter {
                    SetterCaller::Const(call) => {
                        SetterCaller::Const(Arc::new(move |this, args| {
                            let this = this.downcast_ref::<WriteComponent>().unwrap();
                            call(this.component.as_any_ref(), args)
                        }))
                    }
                    SetterCaller::Mut(call) => SetterCaller::Mut(Arc::new(move |this, args| {
                        let this = this.downcast_mut::<WriteComponent>().unwrap();
                        call(this.component.as_any_mut(), args)
                    })),
                    SetterCaller::None => SetterCaller::None,
                },
            })
            .collect::<Vec<_>>()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        self.component
            .get_method_infos()
            .into_iter()
            .map(|method_info| MethodInfo {
                name: method_info.name,
                call: match method_info.call {
                    MethodCaller::Const(call) => {
                        MethodCaller::Const(Arc::new(move |this, args| {
                            let this = this.downcast_ref::<WriteComponent>().unwrap();
                            call(this.component.as_any_ref(), args)
                        }))
                    }
                    MethodCaller::Mut(call) => MethodCaller::Mut(Arc::new(move |this, args| {
                        let this = this.downcast_mut::<WriteComponent>().unwrap();
                        call(this.component.as_any_mut(), args)
                    })),
                },
            })
            .collect::<Vec<_>>()
    }
}

impl SerializableObject for WriteComponent {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        panic!()
    }
}
