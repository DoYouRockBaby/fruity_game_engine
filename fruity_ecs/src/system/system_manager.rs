use rayon::prelude::*;
use std::fmt::Debug;
use crate::entity::entity_manager::EntityManager;
use crate::service::service_manager::ServiceManager;

type System = dyn Fn(&EntityManager, &ServiceManager) + Sync + Send + 'static;

/// A systems collection
pub struct SystemManager {
    systems: Vec<Box<System>>,
}

impl Debug for SystemManager {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'s> SystemManager {
    /// Returns a SystemManager
    pub fn new() -> SystemManager {
        SystemManager {
            systems: Vec::new(),
        }
    }

    /// Add a system to the collection
    ///
    /// # Arguments
    /// * `system` - A function that will compute the world
    ///
    pub fn add_system<T: Fn(&EntityManager, &ServiceManager) + Sync + Send + 'static>(&mut self, system: T) {
        self.systems.push(Box::new(system))
    }

    /// Run all the stored systems
    ///
    /// # Arguments
    /// * `entity_manager` - Entities collection
    /// * `service_manager` - Services collection
    ///
    pub fn run(&self, entity_manager: &EntityManager, service_manager: &ServiceManager) {
        self.systems
            .iter()
            .par_bridge()
            .for_each(|system| system(entity_manager, service_manager));
    }
}