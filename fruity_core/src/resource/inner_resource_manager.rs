use crate::resource::error::AddResourceError;
use crate::resource::error::LoadResourceError;
use crate::resource::error::RemoveResourceError;
use crate::resource::resource::Resource;
use crate::resource::resource_manager::ResourceLoader;
use crate::resource::resource_reference::ResourceReference;
use crate::settings::Settings;
use crate::ResourceManager;
use fruity_any::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(FruityAny)]
pub struct InnerResourceManager {
    resources: HashMap<String, Arc<dyn Resource>>,
    resource_loaders: HashMap<String, ResourceLoader>,
}

impl Debug for InnerResourceManager {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl InnerResourceManager {
    pub fn new() -> InnerResourceManager {
        InnerResourceManager {
            resources: HashMap::new(),
            resource_loaders: HashMap::new(),
        }
    }

    pub fn get<T: Resource + ?Sized>(&self, identifier: &str) -> Option<ResourceReference<T>> {
        match self
            .resources
            .get(identifier)
            .map(|resource| resource.clone())
        {
            Some(resource) => match resource.as_any_arc().downcast::<RwLock<Box<T>>>() {
                Ok(resource) => Some(ResourceReference::new(resource)),
                Err(_) => None,
            },
            None => None,
        }
    }

    pub fn get_untyped(&self, identifier: &str) -> Option<Arc<dyn Resource>> {
        self.resources
            .get(identifier)
            .map(|resource| resource.clone())
    }

    pub fn contains(&self, identifier: &str) -> bool {
        self.resources.contains_key(identifier)
    }

    pub fn add<T: Resource + ?Sized>(
        &mut self,
        identifier: &str,
        resource: Box<T>,
    ) -> Result<(), AddResourceError> {
        if self.resources.contains_key(identifier) {
            Err(AddResourceError::ResourceAlreadyExists(
                identifier.to_string(),
            ))
        } else {
            self.resources
                .insert(identifier.to_string(), Arc::new(RwLock::new(resource)));
            Ok(())
        }
    }

    pub fn remove(&mut self, identifier: &str) -> Result<(), RemoveResourceError> {
        if self.resources.contains_key(identifier) {
            self.resources.remove(identifier);
            Ok(())
        } else {
            Err(RemoveResourceError::ResourceNotFound(
                identifier.to_string(),
            ))
        }
    }

    pub fn add_resource_loader(&mut self, resource_type: &str, loader: ResourceLoader) {
        self.resource_loaders
            .insert(resource_type.to_string(), loader);
    }

    pub fn load_resource_file(
        this: Arc<ResourceManager>,
        path: &str,
        resource_type: &str,
    ) -> Result<(), LoadResourceError> {
        let mut file = File::open(path).unwrap();
        Self::load_resource(this, path, resource_type, &mut file, Settings::new())?;

        Ok(())
    }

    pub fn load_resource(
        this: Arc<ResourceManager>,
        identifier: &str,
        resource_type: &str,
        reader: &mut dyn Read,
        settings: Settings,
    ) -> Result<(), LoadResourceError> {
        let resource_loader = {
            let inner_reader = this.inner.read().unwrap();

            if let Some(resource_loader) = inner_reader.resource_loaders.get(resource_type) {
                Ok(resource_loader.clone())
            } else {
                Err(LoadResourceError::ResourceTypeNotKnown(
                    resource_type.to_string(),
                ))
            }?
        };

        resource_loader(identifier, reader, settings, this);
        Ok(())
    }

    pub fn load_resources_settings(this: Arc<ResourceManager>, settings: Vec<Settings>) {
        settings.into_iter().for_each(|settings| {
            Self::load_resource_settings(this.clone(), settings);
        })
    }

    pub fn load_resource_settings(this: Arc<ResourceManager>, settings: Settings) -> Option<()> {
        // Parse settings
        let fields = if let Settings::Object(fields) = settings {
            fields
        } else {
            return None;
        };

        // Get the resource name
        let name = {
            if let Settings::String(name) = fields.get("name")? {
                name.clone()
            } else {
                return None;
            }
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
        let mut resource_file = File::open(&path).ok()?;

        // Load the resource
        Self::load_resource(
            this,
            &name,
            resource_type,
            &mut resource_file,
            Settings::Object(fields.clone()),
        )
        .ok()?;

        Some(())
    }
}
