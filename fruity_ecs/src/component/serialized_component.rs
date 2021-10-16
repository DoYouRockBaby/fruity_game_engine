use crate::component::component::Component;
use crate::component::component::ComponentDecoder;
use crate::component::component::ComponentDecoderMut;
use crate::serialize::serialized::Serialized;
use fruity_any_derive::*;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectFields;
use std::sync::Arc;

/// A wrapper for components that come from scripting languages as serialized
#[derive(Debug, Clone, FruityAny)]
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
        if let Serialized::Object { class_name, .. } = &self.serialized {
            class_name.clone()
        } else {
            "unknown".to_string()
        }
    }

    fn encode_size(&self) -> usize {
        std::mem::size_of::<Self>()
    }

    fn duplicate(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }

    fn encode(&self, buffer: &mut [u8]) {
        let encoded = unsafe {
            std::slice::from_raw_parts(
                (&*self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
        };

        fruity_collections::slice::copy(buffer, encoded);
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

impl IntrospectFields<Serialized> for SerializedComponent {
    fn get_field_infos(&self) -> Vec<FieldInfo<Serialized>> {
        if let Serialized::Object { fields, .. } = &self.serialized {
            fields
                .iter()
                .map(|(key, _field)| {
                    let key1 = key.clone();
                    let key2 = key.clone();
                    
                    FieldInfo::<Serialized> {
                        name: key.clone(),
                        ty: "".to_string(),
                        getter: Arc::new(move |this| {
                            let this = this.downcast_ref::<SerializedComponent>().unwrap();
                            if let Serialized::Object { fields, .. } = &this.serialized {
                                return fields.get(&key1).unwrap().clone();
                            } else {
                                panic!("A getter try to access an inexistant property in serialized component, should never be reached");
                            }
                        }),
                        setter: Arc::new(move |this, value| {
                            let this = this.downcast_mut::<SerializedComponent>().unwrap();
                            if let Serialized::Object { fields, .. } = &mut this.serialized {
                                fields.insert(key2.clone(), value);
                            } else {
                                panic!("A setter try to access an inexistant property in serialized component, should never be reached");
                            }
                        }),
                    }
                })
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    }
}
