use crate::component::component::Component;
use crate::component::component::ComponentDecoder;
use crate::component::component::ComponentDecoderMut;
use crate::utils::slice::copy;
use fruity_any::FruityAny;
use fruity_introspect::serialized::Serialized;
use fruity_introspect::Constructor;
use fruity_introspect::FieldInfo;
use fruity_introspect::InstantiableObject;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use fruity_introspect::SetterCaller;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;

/// A wrapper for components that come from scripting languages as serialized
#[derive(Debug, Clone, FruityAny)]
pub struct SerializedComponent {
    class_name: String,
    fields: HashMap<String, Serialized>,
}

impl SerializedComponent {
    /// Returns a SerializedComponent
    pub fn new(class_name: String, fields: HashMap<String, Serialized>) -> SerializedComponent {
        SerializedComponent { class_name, fields }
    }
}

impl Component for SerializedComponent {
    fn get_component_type(&self) -> String {
        self.class_name.clone()
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

    fn duplicate(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl IntrospectObject for SerializedComponent {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        self.fields
            .iter()
            .map(|(key, field)| {
                let key1 = key.clone();
                let key2 = key.clone();

                FieldInfo {
                    name: key.clone(),
                    // TODO: Complete that
                    ty: match field {
                        Serialized::U8(_) => TypeId::of::<u8>(),
                        Serialized::U16(_) => TypeId::of::<u16>(),
                        Serialized::U32(_) => TypeId::of::<u32>(),
                        Serialized::U64(_) => TypeId::of::<u64>(),
                        Serialized::USize(_) => TypeId::of::<usize>(),
                        Serialized::I8(_) => TypeId::of::<i8>(),
                        Serialized::I16(_) => TypeId::of::<i16>(),
                        Serialized::I32(_) => TypeId::of::<i32>(),
                        Serialized::I64(_) => TypeId::of::<i64>(),
                        Serialized::ISize(_) => TypeId::of::<isize>(),
                        Serialized::F32(_) => TypeId::of::<f32>(),
                        Serialized::F64(_) => TypeId::of::<f64>(),
                        Serialized::Bool(_) => TypeId::of::<bool>(),
                        Serialized::String(_) => TypeId::of::<String>(),
                        _ => TypeId::of::<u8>(),
                    },
                    getter: Arc::new(move |this| {
                        let this = this.downcast_ref::<SerializedComponent>().unwrap();
                        this.fields.get(&key1).unwrap().clone()
                    }),
                    setter: SetterCaller::Mut(Arc::new(move |this, value| {
                        let this = this.downcast_mut::<SerializedComponent>().unwrap();
                        this.fields.insert(key2.clone(), value);
                    })),
                }
            })
            .collect::<Vec<_>>()
    }
}
