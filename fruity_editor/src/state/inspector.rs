use crate::inspector_service::InspectorService;
use crate::ui_element::layout::Empty;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::serialize::serialized::SerializableObject;
use std::sync::Arc;

#[derive(Debug)]
pub struct InspectorState {
    inspect_service: ResourceReference<InspectorService>,
    selected: Option<Box<dyn SerializableObject>>,
}

impl InspectorState {
    pub fn new(resource_container: Arc<ResourceContainer>) -> Self {
        Self {
            inspect_service: resource_container.require::<InspectorService>(),
            selected: None,
        }
    }

    pub fn get_selected(&self) -> Option<&Box<dyn SerializableObject>> {
        self.selected.as_ref()
    }

    pub fn select(&mut self, selection: Box<dyn SerializableObject>) {
        self.selected = Some(selection);
    }

    pub fn unselect(&mut self) {
        self.selected = None;
    }

    pub fn inspect(&mut self) -> UIElement {
        if let Some(selected) = &self.selected {
            let inspect_service = self.inspect_service.read();
            inspect_service.inspect(selected.duplicate())
        } else {
            Empty {}.elem()
        }
    }
}
