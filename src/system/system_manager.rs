use rayon::prelude::*;
use std::fmt::Debug;
use crate::entity::entity_manager::EntityManager;
use crate::service::service_manager::ServiceManager;

type System = fn(entity_manager: &mut EntityManager, service_manager: &ServiceManager);

pub struct SystemManager {
    system: Vec<System>,
}

impl Debug for SystemManager {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'s> SystemManager {
    pub fn new() -> SystemManager {
        SystemManager {
            system: Vec::new(),
        }
    }

    pub fn add_system(&mut self, system: System) {
        self.system.push(system)
    }

    pub fn run(&self, entity_manager: &mut EntityManager, service_manager: &ServiceManager) {
        self.system
            .iter()
            .for_each(|system| system(entity_manager, service_manager));
    }
}