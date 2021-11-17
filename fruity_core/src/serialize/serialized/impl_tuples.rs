use crate::serialize::serialized::Serialized;

impl Into<Serialized> for () {
    fn into(self) -> Serialized {
        Serialized::Array(vec![])
    }
}

impl<T: Into<Serialized>, U: Into<Serialized>> Into<Serialized> for (T, U) {
    fn into(self) -> Serialized {
        Serialized::Array(vec![self.0.into(), self.1.into()])
    }
}
