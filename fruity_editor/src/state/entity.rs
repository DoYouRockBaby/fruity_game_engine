use fruity_core::entity::archetype::rwlock::EntitySharedRwLock;

#[derive(Debug, Default)]
pub struct EntityState {
    pub selected_entity: Option<EntitySharedRwLock>,
}

#[derive(Debug, Clone)]
pub enum EntityMessage {
    SelectEntity(EntitySharedRwLock),
    UnselectEntity,
    SetEnabled(bool),
    SetName(String),
}

pub fn update_entity(state: &mut EntityState, message: EntityMessage) {
    match message {
        EntityMessage::SelectEntity(entity) => state.selected_entity = Some(entity),
        EntityMessage::UnselectEntity => state.selected_entity = None,
        EntityMessage::SetEnabled(enabled) => {
            if let Some(entity) = &mut state.selected_entity {
                let mut entity = entity.write();
                entity.enabled = enabled;
            }
        }
        EntityMessage::SetName(name) => {
            if let Some(entity) = &mut state.selected_entity {
                let mut entity = entity.write();
                entity.name = name;
            }
        }
    }
}
