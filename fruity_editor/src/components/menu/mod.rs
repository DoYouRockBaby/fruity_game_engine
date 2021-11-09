use crate::components::menu::menu_sections::menu_sections_component;
use crate::components::menu::run_controls::run_controls_component;
use crate::ui_element::menu::MenuBar;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;

mod menu_sections;
mod run_controls;

pub fn draw_menu_component() -> UIElement {
    let mut children = menu_sections_component();
    children.append(&mut run_controls_component());

    MenuBar { children }.elem()
}
