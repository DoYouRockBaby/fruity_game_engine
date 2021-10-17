use crate::error::ResourcesManagerError;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

pub mod error;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ResourceIdentifier(String);

pub struct ResourcesManager {
    resources: HashMap<ResourceIdentifier, Arc<dyn Any + Sync + Send>>,
}

impl ResourcesManager {
    pub fn new() -> ResourcesManager {
        ResourcesManager {
            resources: HashMap::new(),
        }
    }

    pub fn get_resource<T: Any + Sync + Send>(
        &self,
        identifier: ResourceIdentifier,
    ) -> Option<Arc<RwLock<dyn Any>>> {
        match self
            .resources
            .get(&identifier)
            .map(|resource| resource.clone())
        {
            Some(resource) => match resource.downcast::<RwLock<T>>() {
                Ok(resource) => Some(resource),
                Err(_) => None,
            },
            None => None,
        }
    }

    pub fn contains(&self, identifier: ResourceIdentifier) -> bool {
        self.resources.contains_key(&identifier)
    }

    pub fn add_resource<T: Any + Sync + Send>(
        &mut self,
        identifier: ResourceIdentifier,
        resource: T,
    ) -> Result<(), ResourcesManagerError> {
        if self.resources.contains_key(&identifier) {
            Err(ResourcesManagerError::ResourceAlreadyExists(identifier))
        } else {
            self.resources
                .insert(identifier, Arc::new(RwLock::new(resource)));
            Ok(())
        }
    }

    pub fn remove_resource(
        &mut self,
        identifier: ResourceIdentifier,
    ) -> Result<(), ResourcesManagerError> {
        if self.resources.contains_key(&identifier) {
            self.resources.remove(&identifier);
            Ok(())
        } else {
            Err(ResourcesManagerError::ResourceAlreadyExists(identifier))
        }
    }
}
