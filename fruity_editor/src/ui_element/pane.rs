use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use std::sync::Arc;

#[derive(Clone)]
pub struct PaneGrid {
    pub panes: Vec<Pane>,
}

impl UIWidget for PaneGrid {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum UIPaneSide {
    Left,
    Right,
    Bottom,
}

#[derive(Clone)]
pub struct Pane {
    pub title: String,
    pub default_side: UIPaneSide,
    pub render: Arc<dyn Fn() -> UIElement + Send + Sync>,
}

impl UIWidget for Pane {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}
