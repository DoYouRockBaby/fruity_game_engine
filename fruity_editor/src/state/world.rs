use fruity_core::resource::resource_container::ResourceContainer;
use std::sync::Arc;

#[derive(Debug)]
pub struct WorldState {
    pub resource_container: Arc<ResourceContainer>,
}

impl WorldState {
    pub fn new(resource_container: Arc<ResourceContainer>) -> Self {
        WorldState {
            resource_container: resource_container.clone(),
        }
    }
}
