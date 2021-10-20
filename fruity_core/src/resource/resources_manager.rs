use crate::resource::error::AddResourceError;
use crate::resource::error::AddResourceLoaderError;
use crate::resource::error::LoadResourceError;
use crate::resource::error::RemoveResourceError;
use crate::resource::resource::Resource;
use crate::serialize::serialized::ResourceReference;
use crate::serialize::serialized::Serialized;
use crate::service::service::Service;
use crate::service::utils::cast_service;
use crate::service::utils::cast_service_mut;
use crate::service::utils::ArgumentCaster;
use crate::ServiceManager;
use crate::World;
use fruity_any::*;
use fruity_introspect::IntrospectMethods;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::sync::RwLock;

/// A unique resource identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceIdentifier(pub String);

/// A unique resource identifier
#[derive(Debug, Clone)]
pub struct ResourceLoaderParams(pub HashMap<String, Serialized>);

/// A resource loader, it is a function that is intended to parse a resource and add some resource in the resource manager
pub type ResourceLoader = fn(
    resources_manager: &mut ResourcesManager,
    identifier: ResourceIdentifier,
    reader: &mut dyn Read,
    params: ResourceLoaderParams,
    service_manager: Arc<RwLock<ServiceManager>>,
);

/// The resource manager
#[derive(FruityAnySyncSend)]
pub struct ResourcesManager {
    resources: HashMap<ResourceIdentifier, Arc<dyn Resource>>,
    resource_loaders: HashMap<String, ResourceLoader>,
    service_manager: Arc<RwLock<ServiceManager>>,
}

impl Debug for ResourcesManager {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl ResourcesManager {
    /// Returns a ResourcesManager
    pub fn new(world: &World) -> ResourcesManager {
        ResourcesManager {
            resources: HashMap::new(),
            resource_loaders: HashMap::new(),
            service_manager: world.service_manager.clone(),
        }
    }

    /// Get a resource by it's identifier
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    ///
    /// # Generic Arguments
    /// * `T` - The resource type
    ///
    pub fn get_resource<T: Resource>(
        &self,
        identifier: ResourceIdentifier,
    ) -> ResourceReference<T> {
        match self
            .resources
            .get(&identifier)
            .map(|resource| resource.clone())
        {
            Some(resource) => match resource.as_any_arc_send_sync().downcast::<T>() {
                Ok(resource) => ResourceReference::from_resource(resource),
                Err(_) => ResourceReference::new(),
            },
            None => ResourceReference::new(),
        }
    }

    /// Get a resource by it's identifier without casting it
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    ///
    pub fn get_untyped_resource(
        &self,
        identifier: ResourceIdentifier,
    ) -> Option<Arc<dyn Resource>> {
        self.resources
            .get(&identifier)
            .map(|resource| resource.clone())
    }

    /// Check if a resource identifier has already been registered
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    ///
    pub fn contains(&self, identifier: ResourceIdentifier) -> bool {
        self.resources.contains_key(&identifier)
    }

    /// Load and add a resource into the collection
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    /// * `resource_type` - The resource type
    /// * `read` - The reader, generaly a file reader
    ///
    pub fn load_resource(
        &mut self,
        identifier: ResourceIdentifier,
        resource_type: &str,
        reader: &mut dyn Read,
        params: ResourceLoaderParams,
    ) -> Result<(), LoadResourceError> {
        if let Some(resource_loader) = self.resource_loaders.get(resource_type) {
            resource_loader(
                self,
                identifier,
                reader,
                params,
                self.service_manager.clone(),
            );
            Ok(())
        } else {
            Err(LoadResourceError::ResourceTypeNotKnown(
                resource_type.to_string(),
            ))
        }
    }

    /// Load a resource configuration file
    ///
    /// # Arguments
    /// * `path` - The path of the file
    /// * `resource_type` - The resource type
    /// * `read` - The reader, generaly a file reader
    ///
    pub fn read_resource_settings(&mut self, path: &str) -> Result<(), LoadResourceError> {
        let mut file = File::open(path).unwrap();
        self.load_resource(
            ResourceIdentifier(path.to_string()),
            "resource_settings",
            &mut file,
            ResourceLoaderParams::new(),
        )?;

        Ok(())
    }

    /// Add a resource into the collection
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    /// * `resource` - The resource object
    ///
    pub fn add_resource<T: Resource>(
        &mut self,
        identifier: ResourceIdentifier,
        resource: T,
    ) -> Result<(), AddResourceError> {
        if self.resources.contains_key(&identifier) {
            Err(AddResourceError::ResourceAlreadyExists(identifier))
        } else {
            self.resources.insert(identifier, Arc::new(resource));
            Ok(())
        }
    }

    /// Remove a resource of the collection
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    ///
    pub fn remove_resource(
        &mut self,
        identifier: ResourceIdentifier,
    ) -> Result<(), RemoveResourceError> {
        if self.resources.contains_key(&identifier) {
            self.resources.remove(&identifier);
            Ok(())
        } else {
            Err(RemoveResourceError::ResourceNotFound(identifier))
        }
    }

    /// Add a resource loader that will be used to load resources
    ///
    /// # Arguments
    /// * `resource_type` - The resource loader type
    /// * `loader` - The resource loader
    ///
    pub fn add_resource_loader(
        &mut self,
        resource_type: &str,
        loader: ResourceLoader,
    ) -> Result<(), AddResourceLoaderError> {
        if self.resource_loaders.contains_key(resource_type) {
            Err(AddResourceLoaderError::ResourceTypeAlreadyExists(
                resource_type.to_string(),
            ))
        } else {
            self.resource_loaders
                .insert(resource_type.to_string(), loader);
            Ok(())
        }
    }
}

impl IntrospectMethods<Serialized> for ResourcesManager {
    fn get_method_infos(&self) -> Vec<MethodInfo<Serialized>> {
        vec![
            MethodInfo {
                name: "get_resource".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_service::<ResourcesManager>(this);

                    let mut caster = ArgumentCaster::new("get_resource", args);
                    let arg1 = caster.cast_next::<String>()?;

                    let result = this.get_untyped_resource(ResourceIdentifier(arg1));

                    Ok(result.map(|result| Serialized::Resource(result)))
                })),
            },
            MethodInfo {
                name: "read_resource_settings".to_string(),
                call: MethodCaller::Mut(Arc::new(|this, args| {
                    let this = cast_service_mut::<ResourcesManager>(this);

                    let mut caster = ArgumentCaster::new("read_resource_settings", args);
                    let arg1 = caster.cast_next::<String>()?;

                    this.read_resource_settings(&arg1).unwrap();

                    Ok(None)
                })),
            },
        ]
    }
}

impl ResourceLoaderParams {
    /// Returns a ResourceLoaderParams
    pub fn new() -> ResourceLoaderParams {
        ResourceLoaderParams(HashMap::new())
    }

    /// Get a field into the params
    ///
    /// # Arguments
    /// * `key` - The field identifier
    /// * `default` - The default value, if not found or couldn't serialize
    ///
    /// # Generic Arguments
    /// * `T` - The type to cast the value
    ///
    pub fn get<T: TryFrom<Serialized>>(&self, key: &str, default: T) -> T {
        match self.0.get(key) {
            Some(value) => T::try_from(value.clone()).unwrap_or(default),
            None => default,
        }
    }
}

impl Service for ResourcesManager {}