use crate::components::menu::draw_menu_component;
use crate::components::panes::panes_component;
use crate::ui::context::UIContext;
use crate::ui::elements::UIElement;

pub fn root_component(ctx: &mut UIContext) -> Vec<UIElement> {
    vec![draw_menu_component(ctx), panes_component(ctx)]
}
