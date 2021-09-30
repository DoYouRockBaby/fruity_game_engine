use std::sync::RwLock;
use std::sync::Arc;
use std::fmt::Debug;
use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;

/// A services collection
pub struct ServiceManager {
    services: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl Debug for ServiceManager {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'s> ServiceManager {
    /// Returns a ServiceManager
    pub fn new() -> ServiceManager {
        ServiceManager {
            services: HashMap::new(),
        }
    }

    /// Add a service to the collection
    ///
    /// # Generic Arguments
    /// * `T` - The service type
    ///
    pub fn register<T: Any + Send + Sync>(&mut self, service: T) {
        self.services.insert(TypeId::of::<T>(), Arc::new(RwLock::new(service)));
    }

    /// Get an existing service
    ///
    /// # Generic Arguments
    /// * `T` - The service type
    ///
    pub fn get<T: Any + Send + Sync>(&self) -> Option<Arc<RwLock<T>>> {
        match self.services.get(&TypeId::of::<T>()) {
            Some(service) => match service.clone().downcast::<RwLock<T>>() {
                Ok(service) => Some(service),
                Err(_) => None,
            },
            None => None,
        }
    }
}