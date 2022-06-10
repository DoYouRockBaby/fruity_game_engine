use crate::ui_element::menu::MenuItem;
use crate::ui_element::UIAlign;
use crate::ui_element::UIElement;
use crate::ui_element::UISize;
use crate::ui_element::UIWidget;
use std::sync::Arc;

#[derive(Default)]
pub struct Empty {}

impl UIWidget for Empty {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}

#[derive(Default)]
pub struct RowItem {
    pub child: UIElement,
    pub size: UISize,
}

#[derive(Default)]
pub struct Row {
    pub children: Vec<RowItem>,
    pub align: UIAlign,
}

impl UIWidget for Row {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}

#[derive(Default)]
pub struct Column {
    pub children: Vec<UIElement>,
    pub align: UIAlign,
}

impl UIWidget for Column {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}

pub struct Scroll {
    pub child: UIElement,
    pub horizontal: bool,
    pub vertical: bool,
}

impl Default for Scroll {
    fn default() -> Self {
        Scroll {
            child: Empty {}.elem(),
            horizontal: false,
            vertical: true,
        }
    }
}

impl UIWidget for Scroll {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}

pub struct Collapsible {
    pub key: String,
    pub title: String,
    pub on_click: Option<Arc<dyn Fn() + Send + Sync>>,
    pub secondary_actions: Vec<MenuItem>,
    pub child: UIElement,
}

impl Default for Collapsible {
    fn default() -> Self {
        Self {
            key: String::default(),
            title: String::default(),
            on_click: None,
            secondary_actions: Vec::new(),
            child: Empty {}.elem(),
        }
    }
}

impl UIWidget for Collapsible {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}
