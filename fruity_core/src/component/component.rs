use crate::component::serialized_component::SerializedComponent;
use fruity_any::*;
use fruity_introspect::serializable_object::SerializableObject;
use fruity_introspect::serialize::serialized::Serialized;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use fruity_introspect::SetterCaller;
use std::any::Any;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::RwLock;

/// A function to decode an object from byte array to an any reference
pub type ComponentDecoder = fn(buffer: &[u8]) -> &dyn Component;

/// A function to decode an object from byte array to an any mutable reference
pub type ComponentDecoderMut = fn(buffer: &mut [u8]) -> &mut dyn Component;

/// An abstraction over a component, should be implemented for every component
pub trait Component: IntrospectObject + Debug {
    /// Return the component type identifier
    fn get_component_type(&self) -> String;

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

    /// Return a function to decode an object from byte array to an any mutable reference
    fn get_decoder_mut(&self) -> ComponentDecoderMut;
}

/// An container for a component without knowing the instancied type
#[derive(FruityAny, Clone, Debug)]
pub struct AnyComponent {
    component: Arc<RwLock<dyn Component>>,
}

impl AnyComponent {
    /// Returns an AnyComponent
    pub fn new(component: impl Component) -> AnyComponent {
        AnyComponent {
            component: Arc::new(RwLock::new(component)),
        }
    }
}

impl Deref for AnyComponent {
    type Target = RwLock<dyn Component>;

    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        &self.component
    }
}

impl TryFrom<Serialized> for AnyComponent {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
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
    fn get_field_infos(&self) -> Vec<FieldInfo> {
        let reader = self.read().unwrap();
        reader
            .get_field_infos()
            .into_iter()
            .map(|field_info| {
                let getter = field_info.getter.clone();
                let setter = field_info.setter.clone();

                FieldInfo {
                    name: field_info.name,
                    getter: Arc::new(move |this| {
                        let this = this.downcast_ref::<AnyComponent>().unwrap();
                        let reader = this.read().unwrap();

                        getter(reader.as_any_ref())
                    }),
                    setter: match setter {
                        SetterCaller::Const(call) => {
                            SetterCaller::Const(Arc::new(move |this, args| {
                                let this = this.downcast_ref::<AnyComponent>().unwrap();
                                let reader = this.read().unwrap();

                                call(reader.as_any_ref(), args)
                            }))
                        }
                        SetterCaller::Mut(call) => {
                            SetterCaller::Mut(Arc::new(move |this, args| {
                                let this = this.downcast_mut::<AnyComponent>().unwrap();
                                let mut writer = this.write().unwrap();

                                call(writer.as_any_mut(), args)
                            }))
                        }
                    },
                }
            })
            .collect::<Vec<_>>()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        let reader = self.read().unwrap();
        reader
            .get_method_infos()
            .into_iter()
            .map(|method_info| MethodInfo {
                name: method_info.name,
                call: match method_info.call {
                    MethodCaller::Const(call) => {
                        MethodCaller::Const(Arc::new(move |this, args| {
                            let this = this.downcast_ref::<AnyComponent>().unwrap();
                            let reader = this.read().unwrap();

                            call(reader.as_any_ref(), args)
                        }))
                    }
                    MethodCaller::Mut(call) => MethodCaller::Mut(Arc::new(move |this, args| {
                        let this = this.downcast_mut::<AnyComponent>().unwrap();
                        let mut writer = this.write().unwrap();

                        call(writer.as_any_mut(), args)
                    })),
                },
            })
            .collect::<Vec<_>>()
    }
}

impl SerializableObject for AnyComponent {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}
