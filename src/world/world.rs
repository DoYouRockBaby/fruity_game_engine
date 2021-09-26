use std::fmt::Debug;
use crate::system::system_manager::SystemManager;
use crate::service::service_manager::ServiceManager;
use crate::entity::entity_manager::EntityManager;

#[derive(Debug)]
pub struct World {
    pub entity_manager: EntityManager,
    pub service_manager: ServiceManager,
    pub system_manager: SystemManager,
}

impl<'s> World {
    pub fn new() -> World {
        World {
            entity_manager: EntityManager::new(),
            service_manager: ServiceManager::new(),
            system_manager: SystemManager::new(),
        }
    }

    pub fn run(&mut self) {
        self.system_manager.run(&mut self.entity_manager, &mut self.service_manager)
    }
}