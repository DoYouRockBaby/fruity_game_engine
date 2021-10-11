use fruity_any::FruityAny;
use fruity_introspect::IntrospectFields;
use std::any::type_name;
use std::any::Any;
use std::fmt::Debug;

/// A function to decode an object from byte array to an any reference
pub type ComponentDecoder = fn(buffer: &[u8]) -> &dyn Component;

/// A function to decode an object from byte array to an any mutable reference
pub type ComponentDecoderMut = fn(buffer: &mut [u8]) -> &mut dyn Component;

/// An abstraction over a component, should be implemented for every component
pub trait Component: IntrospectFields + Debug + Send + Sync + FruityAny {
    /// Return the component type identifier
    fn get_component_type(&self) -> String;

    /// Return the size that is required to encode the object
    fn encode_size(&self) -> usize;

    /// Encode the object to a byte array
    ///
    /// # Arguments
    /// * `buffer` - The buffer where the encoder will write, should match the result of encode_size function
    ///
    fn encode(self: Box<Self>, buffer: &mut [u8]);

    /// Return a function to decode an object from byte array to an any reference
    fn get_decoder(&self) -> ComponentDecoder;

    /// Return a function to decode an object from byte array to an any mutable reference
    fn get_decoder_mut(&self) -> ComponentDecoderMut;
}

impl dyn Component {
    /// Get one of the component field value
    ///
    /// # Arguments
    /// * `property` - The field name
    ///
    /// # Generic Arguments
    /// * `T` - The field type
    ///
    pub fn get_field<T: Any>(&self, property: &str) -> Option<&T> {
        let fields = self.get_field_infos();
        match fields.iter().find(|field| field.name == property) {
            Some(field) => match (field.getter)(self.as_any_ref()).downcast_ref::<T>() {
                Some(field) => Some(field),
                None => {
                    log::error!(
                        "Try to get a {:?} from property {:?}, got {:?}",
                        type_name::<T>(),
                        property,
                        field.name
                    );
                    None
                }
            },
            None => None,
        }
    }

    /// Set one of the component field
    ///
    /// # Arguments
    /// * `property` - The field name
    /// * `value` - The new field value
    ///
    /// # Generic Arguments
    /// * `T` - The field type
    ///
    pub fn set_field<T: Any>(&mut self, property: &str, value: T) {
        let fields = self.get_field_infos();
        match fields.iter().find(|field| field.name == property) {
            Some(field) => (field.setter)(self.as_any_mut(), &value),
            None => {
                log::error!(
                    "Try to set a {:?} on property {:?}, property not exists",
                    type_name::<T>(),
                    property,
                );
            }
        };
    }
}
