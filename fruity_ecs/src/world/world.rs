use std::fmt::Debug;
use crate::system::system_manager::SystemManager;
use crate::service::service_manager::ServiceManager;
use crate::entity::entity_manager::EntityManager;

/// The main container of the ECS
#[derive(Debug)]
pub struct World {
    /// The entities main container
    pub entity_manager: EntityManager,

    /// The services container
    pub service_manager: ServiceManager,

    /// The systems container
    pub system_manager: SystemManager,
}

impl<'s> World {
    /// Returns a World
    pub fn new() -> World {
        World {
            entity_manager: EntityManager::new(),
            service_manager: ServiceManager::new(),
            system_manager: SystemManager::new(),
        }
    }
    
    /// Runs all the systems of the world
    pub fn run(&mut self) {
        self.system_manager.run(&mut self.entity_manager, &mut self.service_manager)
    }
}