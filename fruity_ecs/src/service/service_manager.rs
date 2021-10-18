use crate::service::service::Service;
use crate::service::service_rwlock::ServiceRwLock;
use fruity_any::*;
use std::any::TypeId;
use std::collections::hash_map::Iter as HashMapIter;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

/// A services collection
#[derive(Debug, FruityAny)]
pub struct ServiceManager {
    services: HashMap<TypeId, Arc<RwLock<Box<dyn Service>>>>,
    services_by_name: HashMap<String, Arc<RwLock<Box<dyn Service>>>>,
}

impl<'s> ServiceManager {
    /// Returns a ServiceManager
    pub fn new() -> ServiceManager {
        ServiceManager {
            services: HashMap::new(),
            services_by_name: HashMap::new(),
        }
    }

    /// Add a service to the collection
    ///
    /// # Generic Arguments
    /// * `T` - The service type
    ///
    pub fn register<T: Service>(&mut self, name: &str, service: T) {
        let service = Arc::new(RwLock::new(Box::new(service) as Box<dyn Service>));
        self.register_arc(name, service);
    }

    /// Add a service to the collection
    ///
    /// # Generic Arguments
    /// * `T` - The service type
    ///
    pub fn register_arc(&mut self, name: &str, service: Arc<RwLock<Box<dyn Service>>>) {
        let type_id = {
            let reader = service.read().unwrap();
            reader.type_id()
        };

        self.services.insert(type_id, service.clone());
        self.services_by_name.insert(name.to_string(), service);
    }

    /// Get an existing service
    ///
    /// # Generic Arguments
    /// * `T` - The service type
    ///
    pub fn get<T: Service>(&self) -> Option<ServiceRwLock<T>> {
        match self.get_by_type_id(&TypeId::of::<T>()) {
            Some(service) => Some(ServiceRwLock::new(service)),
            None => None,
        }
    }

    /// Get an existing service
    ///
    /// # Arguments
    /// * `type_id` - The type id of the service
    ///
    pub fn get_by_type_id(&self, type_id: &TypeId) -> Option<Arc<RwLock<Box<dyn Service>>>> {
        self.services.get(type_id).map(|service| service.clone())
    }

    /// Get an existing service
    ///
    /// # Arguments
    /// * `name` - The name of the service
    ///
    pub fn get_by_name(&self, name: &str) -> Option<Arc<RwLock<Box<dyn Service>>>> {
        self.services_by_name
            .get(name)
            .map(|service| service.clone())
    }

    /// Iter over all services
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            itern_iter: self.services.iter(),
        }
    }
}

/// Iterator over entities of an archetype
pub struct Iter<'s> {
    /// The targeted archetype
    itern_iter: HashMapIter<'s, TypeId, Arc<RwLock<Box<dyn Service>>>>,
}

impl<'s> Iterator for Iter<'s> {
    type Item = Arc<RwLock<Box<dyn Service>>>;

    fn next(&mut self) -> Option<Arc<RwLock<Box<dyn Service>>>> {
        self.itern_iter.next().map(|(_, service)| service.clone())
    }
}
