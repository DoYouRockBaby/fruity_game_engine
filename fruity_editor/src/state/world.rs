use fruity_core::resource::resource_container::ResourceContainer;
use fruity_ecs::entity::archetype::rwlock::EntitySharedRwLock;
use fruity_ecs::entity::entity_service::EntityServiceSnapshot;
use std::sync::Arc;

#[derive(Debug)]
pub struct WorldState {
    pub resource_container: Arc<ResourceContainer>,
    pub snapshot: Option<EntityServiceSnapshot>,
    pub selected_entity: Option<EntitySharedRwLock>,
}

impl WorldState {
    pub fn new(resource_container: Arc<ResourceContainer>) -> Self {
        WorldState {
            resource_container,
            snapshot: None,
            selected_entity: None,
        }
    }
}
