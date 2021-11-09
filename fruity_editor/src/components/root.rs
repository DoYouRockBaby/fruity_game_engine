use crate::components::menu::draw_menu_component;
use crate::components::panes::panes_component;
use crate::ui_element::UIElement;

pub fn root_component() -> Vec<UIElement> {
    vec![draw_menu_component(), panes_component()]
}
