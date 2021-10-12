use fruity_any_derive::*;
use fruity_core::service::Service;
use fruity_core::service_manager::ServiceManager;
use fruity_introspect::IntrospectMethods;
use fruity_introspect::MethodInfo;
use rayon::prelude::*;
use std::fmt::Debug;

type System = dyn Fn(&ServiceManager) + Sync + Send + 'static;

/// A systems collection
#[derive(FruityAny)]
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
    pub fn add_system<T: Fn(&ServiceManager) + Sync + Send + 'static>(&mut self, system: T) {
        self.systems.push(Box::new(system))
    }

    /// Run all the stored systems
    ///
    /// # Arguments
    /// * `entity_manager` - Entities collection
    /// * `service_manager` - Services collection
    ///
    pub fn run(&self, service_manager: &ServiceManager) {
        self.systems
            .iter()
            .par_bridge()
            .for_each(|system| system(service_manager));
    }
}

impl IntrospectMethods for SystemManager {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }
}

impl Service for SystemManager {}
