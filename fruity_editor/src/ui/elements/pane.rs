use crate::ui::elements::UIContext;
use crate::ui::elements::UIElement;
use crate::ui::elements::UIWidget;
use fruity_any::*;
use std::sync::Arc;

#[derive(FruityAny, Clone)]
pub struct PaneGrid {
    pub panes: Vec<Pane>,
}

impl UIWidget for PaneGrid {
    fn elem(self) -> UIElement {
        UIElement::from_widget(self)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum UIPaneSide {
    Center,
    Left,
    Right,
    Bottom,
}

#[derive(Clone)]
pub struct Pane {
    pub title: String,
    pub default_side: UIPaneSide,
    pub render: Arc<dyn Fn(&mut UIContext) -> UIElement + Send + Sync>,
}
