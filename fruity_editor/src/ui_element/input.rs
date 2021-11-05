use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use std::sync::Arc;

pub struct Button {
    pub label: String,
    pub on_click: Arc<dyn Fn() + Send + Sync>,
}

impl UIWidget for Button {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}

pub struct Input {
    pub value: String,
    pub placeholder: String,
    pub on_change: Arc<dyn Fn(&str) + Send + Sync>,
}

impl UIWidget for Input {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}

pub struct IntegerInput {
    pub value: i64,
    pub on_change: Arc<dyn Fn(i64) + Send + Sync>,
}

impl UIWidget for IntegerInput {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}

pub struct FloatInput {
    pub value: f64,
    pub on_change: Arc<dyn Fn(f64) + Send + Sync>,
}

impl UIWidget for FloatInput {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}

pub struct Checkbox {
    pub label: String,
    pub value: bool,
    pub on_change: Arc<dyn Fn(bool) + Send + Sync>,
}

impl UIWidget for Checkbox {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}
