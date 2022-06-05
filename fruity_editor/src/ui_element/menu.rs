use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use crate::MenuItemOptions;
use std::fmt::Debug;
use std::sync::Arc;

pub struct MenuBar {
    pub children: Vec<UIElement>,
}

#[derive(Debug, Clone)]
pub struct MenuSection {
    pub label: String,
    pub items: Vec<MenuItem>,
}

#[derive(Clone)]
pub struct MenuItem {
    pub label: String,
    pub on_click: Arc<dyn Fn() + Send + Sync>,
    pub options: MenuItemOptions,
}

impl Debug for MenuItem {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
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
