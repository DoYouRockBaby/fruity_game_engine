use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use std::sync::Arc;

#[derive(Clone)]
pub struct Menu {
    pub sections: Vec<MenuSection>,
}

#[derive(Clone)]
pub struct MenuSection {
    pub label: String,
    pub items: Vec<MenuItem>,
}

#[derive(Clone)]
pub struct MenuItem {
    pub label: String,
    pub on_clicked: Arc<dyn Fn() + Send + Sync>,
}

impl UIWidget for Menu {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}
