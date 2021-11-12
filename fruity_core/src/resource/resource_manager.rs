use crate::resource::error::AddResourceError;
use crate::resource::error::LoadResourceError;
use crate::resource::error::RemoveResourceError;
use crate::resource::inner_resource_manager::InnerResourceManager;
use crate::resource::resource::Resource;
use crate::resource::resource_reference::ResourceReference;
use crate::resource::serialized_resource::SerializedResource;
use crate::settings::Settings;
use fruity_any::*;
use fruity_introspect::serialized::Serialized;
use fruity_introspect::utils::cast_introspect_ref;
use fruity_introspect::utils::ArgumentCaster;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use std::fmt::Debug;
use std::io::Read;
use std::sync::Arc;
use std::sync::RwLock;

/// A a function that is used to load a resource
pub type ResourceLoader = fn(&str, &mut dyn Read, Settings, Arc<ResourceManager>);

/// The resource manager
#[derive(FruityAny)]
pub struct ResourceManager {
    pub(crate) inner: RwLock<InnerResourceManager>,
}

impl Debug for ResourceManager {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl ResourceManager {
    /// Returns a ResourceManager
    pub fn new() -> ResourceManager {
        ResourceManager {
            inner: RwLock::new(InnerResourceManager::new()),
        }
    }

    /// Get a required resource by it's identifier
    /// Panic if the resource is not known
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    ///
    /// # Generic Arguments
    /// * `T` - The resource type
    ///
    pub fn require<T: Resource + ?Sized>(&self, identifier: &str) -> ResourceReference<T> {
        // TODO: Add a beautifull error message
        self.get(identifier).unwrap()
    }

    /// Get a resource by it's identifier
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    ///
    /// # Generic Arguments
    /// * `T` - The resource type
    ///
    pub fn get<T: Resource + ?Sized>(&self, identifier: &str) -> Option<ResourceReference<T>> {
        let inner = self.inner.read().unwrap();
        inner.get(identifier)
    }

    /// Get a resource by it's identifier without casting it
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    ///
    pub fn get_untyped(&self, identifier: &str) -> Option<Arc<dyn Resource>> {
        let inner = self.inner.read().unwrap();
        inner.get_untyped(identifier)
    }

    /// Check if a resource identifier has already been registered
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    ///
    pub fn contains(&self, identifier: &str) -> bool {
        let inner = self.inner.read().unwrap();
        inner.contains(identifier)
    }

    /// Add a resource into the collection
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    /// * `resource` - The resource object
    ///
    pub fn add<T: Resource + ?Sized>(
        &self,
        identifier: &str,
        resource: Box<T>,
    ) -> Result<(), AddResourceError> {
        let mut inner = self.inner.write().unwrap();
        inner.add(identifier, resource)
    }

    /// Remove a resource of the collection
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    ///
    pub fn remove(&self, identifier: &str) -> Result<(), RemoveResourceError> {
        let mut inner = self.inner.write().unwrap();
        inner.remove(identifier)
    }

    /// Add a resource loader that will be used to load resources
    ///
    /// # Arguments
    /// * `resource_type` - The resource loader type
    /// * `loader` - The resource loader
    ///
    pub fn add_resource_loader(&self, resource_type: &str, loader: ResourceLoader) {
        let mut inner = self.inner.write().unwrap();
        inner.add_resource_loader(resource_type, loader)
    }

    /// Load an any resource file
    ///
    /// # Arguments
    /// * `path` - The path of the file
    /// * `resource_type` - The resource type
    ///
    pub fn load_resource_file(
        self: Arc<Self>,
        path: &str,
        resource_type: &str,
    ) -> Result<(), LoadResourceError> {
        InnerResourceManager::load_resource_file(self, path, resource_type)
    }

    /// Load and add a resource into the collection
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    /// * `resource_type` - The resource type
    /// * `read` - The reader, generaly a file reader
    ///
    pub fn load_resource(
        self: Arc<Self>,
        identifier: &str,
        resource_type: &str,
        reader: &mut dyn Read,
        settings: Settings,
    ) -> Result<(), LoadResourceError> {
        InnerResourceManager::load_resource(self, identifier, resource_type, reader, settings)
    }

    /// Load many resources for settings
    ///
    /// # Arguments
    /// * `settings` - The settings of resources
    ///
    pub fn load_resources_settings(self: Arc<Self>, settings: Vec<Settings>) {
        InnerResourceManager::load_resources_settings(self, settings)
    }

    /// Load resources for settings
    ///
    /// # Arguments
    /// * `settings` - The settings of resources
    ///
    pub fn load_resource_settings(self: Arc<Self>, settings: Settings) -> Option<()> {
        InnerResourceManager::load_resource_settings(self, settings)
    }
}

impl IntrospectObject for ResourceManager {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![
            MethodInfo {
                name: "get".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_introspect_ref::<ResourceManager>(this);

                    let mut caster = ArgumentCaster::new("get", args);
                    let arg1 = caster.cast_next::<String>()?;

                    let result = this.get_untyped(&arg1);

                    Ok(result.map(|result| Serialized::NativeObject(Box::new(result))))
                })),
            },
            MethodInfo {
                name: "add".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_introspect_ref::<ResourceManager>(this);

                    let mut caster = ArgumentCaster::new("add", args);
                    let arg1 = caster.cast_next::<String>()?;
                    let arg2 = caster.next()?;

                    let result = this
                        .add::<SerializedResource>(&arg1, Box::new(SerializedResource::new(arg2)));

                    if let Err(err) = result {
                        log::error!("{}", err.to_string());
                    }

                    Ok(None)
                })),
            },
            MethodInfo {
                name: "remove".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_introspect_ref::<ResourceManager>(this);

                    let mut caster = ArgumentCaster::new("remove", args);
                    let arg1 = caster.cast_next::<String>()?;

                    let result = this.get_untyped(&arg1);

                    Ok(result.map(|result| Serialized::NativeObject(Box::new(result))))
                })),
            },
        ]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}
