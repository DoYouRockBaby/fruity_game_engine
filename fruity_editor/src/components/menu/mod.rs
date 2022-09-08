use crate::components::menu::menu_sections::menu_sections_component;
use crate::components::menu::run_controls::run_controls_component;
use crate::ui::context::UIContext;
use crate::ui::elements::menu::MenuBar;
use crate::ui::elements::UIElement;
use crate::ui::elements::UIWidget;

mod menu_sections;
mod run_controls;

pub fn draw_menu_component(ctx: &mut UIContext) -> UIElement {
    let mut children = menu_sections_component(ctx);
    children.append(&mut run_controls_component(ctx));

    MenuBar { children }.elem()
}
