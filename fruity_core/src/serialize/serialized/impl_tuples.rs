use crate::convert::FruityInto;
use crate::serialize::serialized::Serialized;

impl FruityInto<Serialized> for () {
    fn fruity_into(self) -> Serialized {
        Serialized::Array(vec![])
    }
}

impl<T: FruityInto<Serialized>, U: FruityInto<Serialized>> FruityInto<Serialized> for (T, U) {
    fn fruity_into(self) -> Serialized {
        Serialized::Array(vec![self.0.fruity_into(), self.1.fruity_into()])
    }
}
