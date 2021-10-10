use crate::service::service::Service;
use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

/// A services collection
pub struct ServiceManager {
    services: HashMap<TypeId, Arc<RwLock<dyn Service>>>,
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
    pub fn register<T: Service>(&mut self, service: T) {
        self.services
            .insert(TypeId::of::<T>(), Arc::new(RwLock::new(service)));
    }

    /// Get an existing service
    ///
    /// # Generic Arguments
    /// * `T` - The service type
    ///
    pub fn get<T: Service>(&self) -> Option<Arc<RwLock<T>>> {
        match self.services.get(&TypeId::of::<T>()) {
            Some(service) => match service.clone().downcast::<RwLock<T>>() {
                Ok(service) => Some(service),
                Err(_) => None,
            },
            None => None,
        }
    }
}
