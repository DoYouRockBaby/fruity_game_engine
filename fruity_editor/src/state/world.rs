use fruity_core::resource::resource_manager::ResourceManager;
use std::sync::Arc;

#[derive(Debug)]
pub struct WorldState {
    pub resource_manager: Arc<ResourceManager>,
}

impl WorldState {
    pub fn new(resource_manager: Arc<ResourceManager>) -> Self {
        WorldState { resource_manager }
    }
}
