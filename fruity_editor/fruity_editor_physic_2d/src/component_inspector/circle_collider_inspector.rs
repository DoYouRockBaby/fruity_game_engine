use crate::ColliderState;
use fruity_ecs::component::component_reference::ComponentReference;
use fruity_editor::components::fields::edit_introspect_fields;
use fruity_editor::ui::context::UIContext;
use fruity_editor::ui::elements::input::Button;
use fruity_editor::ui::elements::layout::Column;
use fruity_editor::ui::elements::UIElement;
use fruity_editor::ui::elements::UIWidget;
use fruity_editor::ui::hooks::use_write_service;
use std::sync::Arc;

pub fn circle_collider_inspector(ctx: &mut UIContext, component: ComponentReference) -> UIElement {
    Column {
        children: vec![
            edit_introspect_fields(ctx, Box::new(component.clone())),
            Button {
                label: "Edit collider".to_string(),
                on_click: Arc::new(move |ctx| {
                    let mut collider_state = use_write_service::<ColliderState>(ctx);
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
