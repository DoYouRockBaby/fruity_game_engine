use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_ecs::component::component_reference::ComponentReference;
use fruity_editor::state::inspector::InspectorState;

#[derive(Debug, FruityAny)]
pub struct ColliderState {
    inspector_state: ResourceReference<InspectorState>,
    current_editing_collider: Option<ComponentReference>,
}

impl ColliderState {
    pub fn new(resource_container: ResourceContainer) -> Self {
        let inspector_state = resource_container.require::<InspectorState>();
        let inspector_state_reader = inspector_state.read();

        let resource_container_2 = resource_container.clone();
        inspector_state_reader.on_selected.add_observer(move |_| {
            let collider_state = resource_container.require::<ColliderState>();
            let mut collider_state = collider_state.write();
            collider_state.current_editing_collider = None;
        });

        inspector_state_reader.on_unselected.add_observer(move |_| {
            let collider_state = resource_container_2.require::<ColliderState>();
            let mut collider_state = collider_state.write();
            collider_state.current_editing_collider = None;
        });

        Self {
            inspector_state,
            current_editing_collider: None,
        }
    }

    pub fn edit_collider(&mut self, component: ComponentReference) {
        self.current_editing_collider = Some(component);

        let mut inspector_state_writer = self.inspector_state.write();
        inspector_state_writer.temporary_display_gizmos();
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

// TODO
impl IntrospectObject for ColliderState {
    fn get_class_name(&self) -> String {
        "ColliderState".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for ColliderState {}
