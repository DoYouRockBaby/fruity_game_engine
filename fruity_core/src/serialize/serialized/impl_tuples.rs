use crate::convert::FruityInto;
use crate::convert::FruityTryFrom;
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

impl<T: FruityTryFrom<Serialized>, U: FruityTryFrom<Serialized>> FruityTryFrom<Serialized>
    for (T, U)
{
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::Array(mut value) => {
                if value.len() < 2 {
                    return Err(format!("Couldn't convert {:?} to tuple", value));
                };

                let value1 = if let Ok(value1) = T::fruity_try_from(value.remove(0)) {
                    value1
                } else {
                    return Err(format!("Couldn't convert {:?} to tuple", value));
                };

                let value2 = if let Ok(value2) = U::fruity_try_from(value.remove(0)) {
                    value2
                } else {
                    return Err(format!("Couldn't convert {:?} to tuple", value));
                };

                Ok((value1, value2))
            }
            _ => Err(format!("Couldn't convert {:?} to tuple", value)),
        }
    }
}
