use crate::editor_menu_service::MenuItem;
use crate::ui::elements::UIAlign;
use crate::ui::elements::UIContext;
use crate::ui::elements::UIElement;
use crate::ui::elements::UISize;
use crate::ui::elements::UIWidget;
use fruity_any::*;
use std::sync::Arc;

#[derive(FruityAny, Default)]
pub struct Empty {}

impl UIWidget for Empty {
    fn elem(self) -> UIElement {
        UIElement::from_widget(self)
    }
}

#[derive(Default)]
pub struct RowItem {
    pub child: UIElement,
    pub size: UISize,
}

#[derive(FruityAny, Default)]
pub struct Row {
    pub children: Vec<RowItem>,
    pub align: UIAlign,
}

impl UIWidget for Row {
    fn elem(self) -> UIElement {
        UIElement::from_widget(self)
    }
}

#[derive(FruityAny, Default)]
pub struct Column {
    pub children: Vec<UIElement>,
    pub align: UIAlign,
}

impl UIWidget for Column {
    fn elem(self) -> UIElement {
        UIElement::from_widget(self)
    }
}

#[derive(FruityAny)]
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
        UIElement::from_widget(self)
    }
}

#[derive(FruityAny)]
pub struct Collapsible {
    pub key: String,
    pub title: String,
    pub on_click: Option<Arc<dyn Fn(&UIContext) + Send + Sync>>,
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
        UIElement::from_widget(self)
    }
}
