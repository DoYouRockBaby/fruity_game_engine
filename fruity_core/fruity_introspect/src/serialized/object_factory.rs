use crate::serialized::Serialized;
use crate::Constructor;
use crate::InstantiableObject;
use fruity_any::*;
use std::collections::HashMap;
use std::fmt::Debug;

/// Provides a factory for the introspect types
/// This will be used by the scripting language to expose object creation
#[derive(FruityAny)]
pub struct ObjectFactory {
    factories: HashMap<String, Constructor>,
}

impl Debug for ObjectFactory {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl ObjectFactory {
    /// Returns an ObjectFactory
    pub fn new() -> ObjectFactory {
        ObjectFactory {
            factories: HashMap::new(),
        }
    }

    /// Register a new object factory
    ///
    /// # Arguments
    /// * `object_type` - The object type identifier
    ///
    /// # Generic Arguments
    /// * `T` - The type of the object
    ///
    pub fn register<T>(&mut self, object_type: &str)
    where
        T: InstantiableObject,
    {
        self.factories
            .insert(object_type.to_string(), T::get_constructor());
    }

    /// Instantiate an object from it's factory
    ///
    /// # Arguments
    /// * `object_type` - The object type identifier
    /// * `serialized` - A serialized value that will populate the new component
    ///
    pub fn instantiate(&self, object_type: &str, args: Vec<Serialized>) -> Option<Serialized> {
        let factory = self.factories.get(object_type)?;
        let instantied = factory(args).ok()?;
        Some(instantied)
    }

    /// Iterate over all object factories
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Constructor)> {
        self.factories.iter()
    }
}
