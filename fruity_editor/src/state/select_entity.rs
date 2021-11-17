use fruity_ecs::entity::archetype::rwlock::EntitySharedRwLock;

#[derive(Debug, Default)]
pub struct SelectEntityState {
    selected_entity: Option<EntitySharedRwLock>,
}

impl SelectEntityState {
    pub fn get_selected_entity(&self) -> Option<EntitySharedRwLock> {
        self.selected_entity.clone()
    }

    pub fn select_entity(&mut self, entity: EntitySharedRwLock) {
        self.selected_entity = Some(entity);
    }

    pub fn unselect_entity(&mut self) {
        self.selected_entity = None;
    }
}
