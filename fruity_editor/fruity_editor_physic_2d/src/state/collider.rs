use fruity_core::resource::resource_container::ResourceContainer;
use fruity_editor::hooks::use_global;
use fruity_editor::state::inspector::InspectorState;
use std::sync::Arc;

#[derive(Debug)]
pub struct ColliderState {
    pub is_editing_collider: bool,
}

impl ColliderState {
    pub fn new(_resource_container: Arc<ResourceContainer>) -> Self {
        let inspector_state = use_global::<InspectorState>();

        inspector_state.on_selected.add_observer(|_| {
            let collider_state = use_global::<ColliderState>();
            collider_state.is_editing_collider = false;
        });

        inspector_state.on_unselected.add_observer(|_| {
            let collider_state = use_global::<ColliderState>();
            collider_state.is_editing_collider = false;
        });

        Self {
            is_editing_collider: false,
        }
    }

    pub fn edit_collider(&mut self) {
        self.is_editing_collider = true;

        let inspector_state = use_global::<InspectorState>();
        inspector_state.temporary_display_gizmos();
    }

    pub fn is_editing_collider(&self) -> bool {
        self.is_editing_collider
    }
}
