use fruity_core::service::service_manager::ServiceManager;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Debug)]
pub struct WorldState {
    pub service_manager: Arc<RwLock<ServiceManager>>,
}

impl WorldState {
    pub fn new(service_manager: &Arc<RwLock<ServiceManager>>) -> Self {
        WorldState {
            service_manager: service_manager.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum WorldMessage {}

pub fn update_world(_state: &mut WorldState, message: WorldMessage) {
    match message {}
}
