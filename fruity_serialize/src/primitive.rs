use crate::deserialize::Deserialize;
use crate::serialize::Serialize;
use crate::serialized::Serialized;

macro_rules! impl_serialize_deserialize {
  { $type:ident, $enum:ident } => {
    impl Deserialize for $type {
        type Value = $type;

        fn deserialize(
            serialized: Serialized,
        ) -> Option<Self::Value> {
            match serialized {
                Serialized::$enum(value) => Some(value),
                _ => None,
            }
        }
    }

    impl Serialize for $type {
        fn serialize<'a>(
            &self,
        ) -> Serialized {
            Serialized::$enum(self.clone())
        }
    }
  };
}

impl_serialize_deserialize! {i8, I8}
impl_serialize_deserialize! {i16, I16}
impl_serialize_deserialize! {i32, I32}
impl_serialize_deserialize! {i64, I64}
impl_serialize_deserialize! {isize, ISize}
impl_serialize_deserialize! {u8, U8}
impl_serialize_deserialize! {u16, U16}
impl_serialize_deserialize! {u32, U32}
impl_serialize_deserialize! {u64, U64}
impl_serialize_deserialize! {usize, USize}
impl_serialize_deserialize! {f32, F32}
impl_serialize_deserialize! {f64, F64}
impl_serialize_deserialize! {bool, Bool}
impl_serialize_deserialize! {String, String}
