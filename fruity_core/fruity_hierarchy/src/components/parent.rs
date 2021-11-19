use fruity_any::*;
use fruity_ecs::entity::archetype::rwlock::EntitySharedRwLock;
use fruity_ecs::*;

#[derive(Debug, Clone, Component, FruityAny)]
pub struct Parent {
    pub parent: Option<SignalField<EntitySharedRwLock>>,
}

impl Default for Parent {
    fn default() -> Self {
        Self { parent: None }
    }
}
