use crate::ColliderState;
use fruity_ecs::component::component_reference::ComponentReference;
use fruity_editor::components::fields::edit_introspect_fields;
use fruity_editor::hooks::use_global;
use fruity_editor::ui_element::input::Button;
use fruity_editor::ui_element::layout::Column;
use fruity_editor::ui_element::UIElement;
use fruity_editor::ui_element::UIWidget;
use std::sync::Arc;

pub fn rect_collider_inspector(component: ComponentReference) -> UIElement {
    Column {
        children: vec![
            edit_introspect_fields(Box::new(component.clone())),
            Button {
                label: "Edit collider".to_string(),
                on_click: Arc::new(move || {
                    let collider_state = use_global::<ColliderState>();
                    collider_state.edit_collider(component.clone());
                }),
                ..Default::default()
            }
            .elem(),
        ],
        ..Default::default()
    }
    .elem()
}
