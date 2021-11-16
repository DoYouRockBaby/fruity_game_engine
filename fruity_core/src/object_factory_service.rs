use crate::resource::resource::Resource;
use crate::ResourceContainer;
use fruity_any::*;
use fruity_introspect::serialized::object_factory::ObjectFactory;
use fruity_introspect::serialized::Serialized;
use fruity_introspect::utils::cast_introspect_ref;
use fruity_introspect::utils::ArgumentCaster;
use fruity_introspect::Constructor;
use fruity_introspect::FieldInfo;
use fruity_introspect::InstantiableObject;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use std::fmt::Debug;
use std::sync::Arc;

/// Provides a factory for the introspect types
/// This will be used by the scripting language to expose object creation
#[derive(FruityAny)]
pub struct ObjectFactoryService {
    object_factory: ObjectFactory,
}

impl Debug for ObjectFactoryService {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl ObjectFactoryService {
    /// Returns an ObjectFactoryService
    pub fn new(_resource_container: Arc<ResourceContainer>) -> ObjectFactoryService {
        ObjectFactoryService {
            object_factory: ObjectFactory::new(),
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
        self.object_factory.register::<T>(object_type);
    }

    /// Instantiate an object from it's factory
    ///
    /// # Arguments
    /// * `object_type` - The object type identifier
    /// * `serialized` - A serialized value that will populate the new component
    ///
    pub fn instantiate(&self, object_type: &str, args: Vec<Serialized>) -> Option<Serialized> {
        self.object_factory.instantiate(object_type, args)
    }

    /// Iterate over all object factories
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Constructor)> {
        self.object_factory.iter()
    }

    /// Iterate over all object factories
    pub fn get_object_factory(&self) -> &ObjectFactory {
        &self.object_factory
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
