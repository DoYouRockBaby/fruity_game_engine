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
    component_type: String,
    serialized: Serialized,
}

impl Component for SerializedComponent {
    fn get_component_type(&self) -> String {
        self.component_type.clone()
    }

    fn encode_size(&self) -> usize {
        std::mem::size_of::<Self>()
    }

    fn encode(self: Box<Self>, buffer: &mut [u8]) {
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
        if let Serialized::Object { fields, .. } = self.serialized.clone() {
            fields
                .iter()
                .map(|(key, _field)| {
                    let key1 = key.clone();
                    let key2 = key.clone();
                    
                    FieldInfo {
                        name: key.clone(),
                        ty: "".to_string(),
                        getter: Arc::new(move |this| {
                            let this = this.downcast_ref::<SerializedComponent>().unwrap();
                            if let Serialized::Object { fields, .. } = this.serialized.clone() {
                                return fields.get(&key1).unwrap().clone();
                            } else {
                                panic!("A getter try to access an inexistant property in serialized component, should never be reached");
                            }
                        }),
                        setter: Arc::new(move |this, value| {
                            let this = this.downcast_mut::<SerializedComponent>().unwrap();
                            if let Serialized::Object { fields, .. } = &mut this.serialized.clone() {
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
