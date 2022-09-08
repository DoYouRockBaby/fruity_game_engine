use crate::state::inspector::InspectorState;
use crate::ui::context::UIContext;
use crate::ui::elements::UIElement;
use crate::ui::hooks::use_read_service;

pub fn inspector_component(ctx: &mut UIContext) -> UIElement {
    let inspector_state = use_read_service::<InspectorState>(&ctx);
    inspector_state.inspect(ctx)
}
