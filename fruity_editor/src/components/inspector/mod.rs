use crate::hooks::use_global;
use crate::state::inspector::InspectorState;
use crate::ui_element::UIElement;

pub fn inspector_component() -> UIElement {
    let inspector_state = use_global::<InspectorState>();
    inspector_state.inspect()
}
