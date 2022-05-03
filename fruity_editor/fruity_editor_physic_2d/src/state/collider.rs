use fruity_core::resource::resource_container::ResourceContainer;
use fruity_ecs::component::component_reference::ComponentReference;
use fruity_editor::hooks::use_global;
use fruity_editor::state::inspector::InspectorState;
use std::sync::Arc;

#[derive(Debug)]
pub struct ColliderState {
    current_editing_collider: Option<ComponentReference>,
}

impl ColliderState {
    pub fn new(_resource_container: Arc<ResourceContainer>) -> Self {
        let inspector_state = use_global::<InspectorState>();

        inspector_state.on_selected.add_observer(|_| {
            let collider_state = use_global::<ColliderState>();
            collider_state.current_editing_collider = None;
        });

        inspector_state.on_unselected.add_observer(|_| {
            let collider_state = use_global::<ColliderState>();
            collider_state.current_editing_collider = None;
        });

        Self {
            current_editing_collider: None,
        }
    }

    pub fn edit_collider(&mut self, component: ComponentReference) {
        self.current_editing_collider = Some(component);

        let inspector_state = use_global::<InspectorState>();
        inspector_state.temporary_display_gizmos();
    }

    pub fn is_editing_collider(&self) -> bool {
        match self.current_editing_collider {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get_editing_collider(&self) -> Option<ComponentReference> {
        self.current_editing_collider.clone()
    }
}
