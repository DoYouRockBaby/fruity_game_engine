use fruity_any::*;
use fruity_core::signal::SignalProperty;
use fruity_ecs::entity::entity::EntityId;
use fruity_ecs::*;

#[derive(Debug, Clone, Default, Component, FruityAny)]
pub struct Parent {
    pub parent_id: SignalProperty<Option<EntityId>>,
}
