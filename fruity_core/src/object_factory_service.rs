use crate::introspect::Constructor;
use crate::introspect::FieldInfo;
use crate::introspect::InstantiableObject;
use crate::introspect::IntrospectError;
use crate::introspect::IntrospectObject;
use crate::introspect::MethodCaller;
use crate::introspect::MethodInfo;
use crate::resource::resource::Resource;
use crate::serialize::serialized::Serialized;
use crate::utils::introspect::cast_introspect_ref;
use crate::utils::introspect::ArgumentCaster;
use crate::ResourceContainer;
use fruity_any::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

/// Provides a factory for the introspect types
/// This will be used by the scripting language to expose object creation
#[derive(FruityAny)]
pub struct ObjectFactoryService {
    resource_container: Arc<ResourceContainer>,
    factories: HashMap<String, Constructor>,
}

impl Debug for ObjectFactoryService {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl ObjectFactoryService {
    /// Returns an ObjectFactoryService
    pub fn new(resource_container: Arc<ResourceContainer>) -> ObjectFactoryService {
        ObjectFactoryService {
            resource_container,
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

    /// Register a new object factory from a function constructor
    ///
    /// # Arguments
    /// * `object_type` - The object type identifier
    /// * `constructor` - The constructor
    ///
    pub fn register_func(
        &mut self,
        object_type: &str,
        constructor: impl Fn(Arc<ResourceContainer>, Vec<Serialized>) -> Result<Serialized, IntrospectError>
            + Send
            + Sync
            + 'static,
    ) {
        self.factories
            .insert(object_type.to_string(), Arc::new(constructor));
    }

    /// Instantiate an object from it's factory
    ///
    /// # Arguments
    /// * `object_type` - The object type identifier
    /// * `serialized` - A serialized value that will populate the new component
    ///
    pub fn instantiate(&self, object_type: &str, args: Vec<Serialized>) -> Option<Serialized> {
        let factory = self.factories.get(object_type)?;
        let instantied = factory(self.resource_container.clone(), args).ok()?;
        Some(instantied)
    }

    /// Iterate over all object factories
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Constructor)> {
        self.factories.iter()
    }
}

impl IntrospectObject for ObjectFactoryService {
    fn get_class_name(&self) -> String {
        "ObjectFactoryService".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![MethodInfo {
            name: "instantiate".to_string(),
            call: MethodCaller::Const(Arc::new(move |this, args| {
                let this = cast_introspect_ref::<ObjectFactoryService>(this);

                let mut caster = ArgumentCaster::new("instantiate", args);
                let arg1 = caster.cast_next::<String>()?;
                let rest = caster.rest();

                let new_object = this.instantiate(&arg1, rest);
                Ok(new_object)
            })),
        }]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for ObjectFactoryService {}
