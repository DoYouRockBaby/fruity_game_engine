use crate::inspector_service::InspectorService;
use crate::ui_element::layout::Empty;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use crate::EditorComponentService;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_core::signal::Signal;
use fruity_ecs::entity::entity_reference::ComponentReference;
use std::sync::Arc;

#[derive(Debug)]
pub struct InspectorState {
    inspect_service: ResourceReference<InspectorService>,
    inspect_component_service: ResourceReference<EditorComponentService>,
    selected: Option<Box<dyn SerializableObject>>,
    temporary_disable_gizmos: bool,
    pub on_selected: Signal<Box<dyn SerializableObject>>,
    pub on_unselected: Signal<()>,
}

impl InspectorState {
    pub fn new(resource_container: Arc<ResourceContainer>) -> Self {
        Self {
            inspect_service: resource_container.require::<InspectorService>(),
            inspect_component_service: resource_container.require::<EditorComponentService>(),
            selected: None,
            temporary_disable_gizmos: false,
            on_selected: Signal::new(),
            on_unselected: Signal::new(),
        }
    }

    pub fn get_selected(&self) -> Option<&Box<dyn SerializableObject>> {
        self.selected.as_ref()
    }

    pub fn select(&mut self, selection: Box<dyn SerializableObject>) {
        self.temporary_disable_gizmos = false;
        self.selected = Some(selection.duplicate());
        self.on_selected.notify(selection);
    }

    pub fn unselect(&mut self) {
        self.temporary_disable_gizmos = false;
        self.selected = None;
        self.on_unselected.notify(());
    }

    pub fn inspect(&mut self) -> UIElement {
        if let Some(selected) = &self.selected {
            let inspect_service = self.inspect_service.read();
            inspect_service.inspect(selected.duplicate())
        } else {
            Empty {}.elem()
        }
    }

    pub fn inspect_component(&self, component: ComponentReference) -> UIElement {
        let inspect_component_service = self.inspect_component_service.read();
        inspect_component_service.inspect(component)
    }

    pub fn is_gizmos_enabled(&self) -> bool {
        !self.temporary_disable_gizmos
    }

    pub fn temporary_display_gizmos(&mut self) {
        self.temporary_disable_gizmos = true;
    }
}
