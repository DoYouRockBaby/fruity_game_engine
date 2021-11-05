use fruity_core::entity::archetype::rwlock::EntitySharedRwLock;

#[derive(Debug, Default)]
pub struct EntityState {
    pub selected_entity: Option<EntitySharedRwLock>,
}
