use crate::resource::error::AddResourceError;
use crate::resource::error::LoadResourceError;
use crate::resource::error::RemoveResourceError;
use crate::resource::resource::Resource;
use crate::service::service::Service;
use crate::service::utils::cast_service;
use crate::service::utils::ArgumentCaster;
use crate::settings::Settings;
use crate::ServiceManager;
use fruity_any::*;
use fruity_introspect::serialized::Serialized;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;

/// A a function that is used to load a resource
pub type ResourceLoader = fn(
    &mut ResourcesManager,
    ResourceIdentifier,
    &mut dyn Read,
    Settings,
    Arc<RwLock<ServiceManager>>,
);

/// A unique resource identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceIdentifier(pub String);

/// The resource manager
#[derive(FruityAny)]
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
    pub fn new(service_manager: &Arc<RwLock<ServiceManager>>) -> ResourcesManager {
        ResourcesManager {
            resources: HashMap::new(),
            resource_loaders: HashMap::new(),
            service_manager: service_manager.clone(),
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
    pub fn get_resource<T: Resource>(&self, identifier: ResourceIdentifier) -> Option<Arc<T>> {
        match self
            .resources
            .get(&identifier)
            .map(|resource| resource.clone())
        {
            Some(resource) => match resource.as_any_arc().downcast::<T>() {
                Ok(resource) => Some(resource),
                Err(_) => None,
            },
            None => None,
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

    /// Load an any resource file
    ///
    /// # Arguments
    /// * `path` - The path of the file
    /// * `resource_type` - The resource type
    ///
    pub fn load_resource_file(
        &mut self,
        path: &str,
        resource_type: &str,
    ) -> Result<(), LoadResourceError> {
        let mut file = File::open(path).unwrap();
        self.load_resource(
            ResourceIdentifier(path.to_string()),
            resource_type,
            &mut file,
            Settings::new(),
        )?;

        Ok(())
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
        settings: Settings,
    ) -> Result<(), LoadResourceError> {
        if let Some(resource_loader) = self.resource_loaders.get(resource_type) {
            resource_loader(
                self,
                identifier,
                reader,
                settings,
                self.service_manager.clone(),
            );
            Ok(())
        } else {
            Err(LoadResourceError::ResourceTypeNotKnown(
                resource_type.to_string(),
            ))
        }
    }

    /// Load many resources for settings
    ///
    /// # Arguments
    /// * `settings` - The settings of resources
    ///
    pub fn load_resources_settings(&mut self, settings: Vec<Settings>) {
        settings.into_iter().for_each(|settings| {
            self.load_resource_settings(settings);
        })
    }

    /// Load resources for settings
    ///
    /// # Arguments
    /// * `settings` - The settings of resources
    ///
    pub fn load_resource_settings(&mut self, settings: Settings) -> Option<()> {
        // Parse settings
        let fields = if let Settings::Object(fields) = settings {
            fields
        } else {
            return None;
        };

        // Get the resource path
        let path = {
            if let Settings::String(path) = fields.get("path")? {
                path.clone()
            } else {
                return None;
            }
        };

        // Deduce informations about the resource from the path
        let resource_type = Path::new(&path).extension()?;
        let resource_type = resource_type.to_str()?;
        let resource_identifier = ResourceIdentifier(path.clone());
        let mut resource_file = File::open(&path).ok()?;

        // Load the resource
        self.load_resource(
            resource_identifier,
            resource_type,
            &mut resource_file,
            Settings::Object(fields.clone()),
        )
        .ok()?;

        Some(())
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
    pub fn add_resource_loader(&mut self, resource_type: &str, loader: ResourceLoader) {
        self.resource_loaders
            .insert(resource_type.to_string(), loader);
    }
}

impl IntrospectObject for ResourcesManager {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![MethodInfo {
            name: "get_resource".to_string(),
            call: MethodCaller::Const(Arc::new(|this, args| {
                let this = cast_service::<ResourcesManager>(this);

                let mut caster = ArgumentCaster::new("get_resource", args);
                let arg1 = caster.cast_next::<String>()?;

                let result = this.get_untyped_resource(ResourceIdentifier(arg1));

                Ok(result.map(|result| Serialized::NativeObject(Box::new(result))))
            })),
        }]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Service for ResourcesManager {}
