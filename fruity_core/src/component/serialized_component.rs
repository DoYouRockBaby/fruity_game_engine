use crate::utils::slice::copy;
use fruity_introspect::SetterCaller;
use fruity_introspect::MethodInfo;
use crate::component::component::Component;
use crate::component::component::ComponentDecoder;
use crate::component::component::ComponentDecoderMut;
use fruity_introspect::serialize::serialized::Serialized;
use fruity_any::FruityAny;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use std::sync::Arc;

/// A wrapper for components that come from scripting languages as serialized
#[derive(Debug, FruityAny)]
pub struct SerializedComponent {
    serialized: Serialized,
}

impl SerializedComponent {
    /// Returns a SerializedComponent
    pub fn new(serialized: Serialized) -> SerializedComponent {
        SerializedComponent {
            serialized
        }
    }
}

impl Component for SerializedComponent {
    fn get_component_type(&self) -> String {
        if let Serialized::SerializedObject { class_name, .. } = &self.serialized {
            class_name.clone()
        } else {
            "unknown".to_string()
        }
    }

    fn encode_size(&self) -> usize {
        std::mem::size_of::<Self>()
    }

    fn encode(&self, buffer: &mut [u8]) {
        let encoded = unsafe {
            std::slice::from_raw_parts(
                (&*self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
        };

        copy(buffer, encoded);
    }

    fn get_decoder(&self) -> ComponentDecoder {
        |data| {
            let (_head, body, _tail) = unsafe { data.align_to::<Self>() };
            &body[0]
        }
    }

    fn get_decoder_mut(&self) -> ComponentDecoderMut {
        |data| {
            let (_head, body, _tail) = unsafe { data.align_to_mut::<Self>() };
            &mut body[0]
        }
    }
}

impl IntrospectObject for SerializedComponent {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        if let Serialized::SerializedObject { fields, .. } = &self.serialized {
            fields
                .iter()
                .map(|(key, _field)| {
                    let key1 = key.clone();
                    let key2 = key.clone();
                    
                    FieldInfo {
                        name: key.clone(),
                        getter: Arc::new(move |this| {
                            let this = this.downcast_ref::<SerializedComponent>().unwrap();
                            if let Serialized::SerializedObject { fields, .. } = &this.serialized {
                                // TODO: Find a way to fix that, the problem is that now, serialized is not clonable
                                return Serialized::Bool(true);
                                //return fields.get(&key1).unwrap();
                            } else {
                                panic!("A getter try to access an inexistant property in serialized component, should never be reached");
                            }
                        }),
                        setter: SetterCaller::Mut(Arc::new(move |this, value| {
                            let this = this.downcast_mut::<SerializedComponent>().unwrap();
                            if let Serialized::SerializedObject { fields, .. } = &mut this.serialized {
                                fields.insert(key2.clone(), value);
                            } else {
                                panic!("A setter try to access an inexistant property in serialized component, should never be reached");
                            }
                        })),
                    }
                })
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    }
}
