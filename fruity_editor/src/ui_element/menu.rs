use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use std::sync::Arc;

pub struct MenuBar {
    pub children: Vec<UIElement>,
}

pub struct MenuSection {
    pub label: String,
    pub items: Vec<MenuItem>,
}

pub struct MenuItem {
    pub label: String,
    pub on_click: Arc<dyn Fn() + Send + Sync>,
}

impl UIWidget for MenuBar {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}

impl UIWidget for MenuSection {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}
