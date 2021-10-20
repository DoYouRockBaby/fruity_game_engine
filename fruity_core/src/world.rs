use crate::service::service_manager::ServiceManager;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

/// The main container of the ECS
#[derive(Debug)]
pub struct World {
    /// The services container
    pub service_manager: Arc<RwLock<ServiceManager>>,
}

impl<'s> World {
    /// Returns a World
    pub fn new() -> World {
        World {
            service_manager: Arc::new(RwLock::new(ServiceManager::new())),
        }
    }
}
