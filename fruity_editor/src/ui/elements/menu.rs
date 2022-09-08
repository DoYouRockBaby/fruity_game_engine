use crate::editor_menu_service::MenuItem;
use crate::ui::elements::UIElement;
use crate::ui::elements::UIWidget;
use fruity_any::*;
use std::fmt::Debug;

#[derive(FruityAny)]
pub struct MenuBar {
    pub children: Vec<UIElement>,
}

#[derive(FruityAny, Debug, Clone)]
pub struct MenuSection {
    pub label: String,
    pub items: Vec<MenuItem>,
}

impl UIWidget for MenuBar {
    fn elem(self) -> UIElement {
        UIElement::from_widget(self)
    }
}

impl UIWidget for MenuSection {
    fn elem(self) -> UIElement {
        UIElement::from_widget(self)
    }
}
