use crate::ui_element::UIAlign;
use crate::ui_element::UIElement;
use crate::ui_element::UISize;
use crate::ui_element::UIWidget;

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
pub struct Container {
    pub child: UIElement,
    pub width: UISize,
    pub height: UISize,
}

impl UIWidget for Container {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}

#[derive(Default)]
pub struct Row {
    pub children: Vec<UIElement>,
    pub align: UIAlign,
    pub wrapped: bool,
    pub children_with_same_width: bool,
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
    pub title: String,
    pub child: UIElement,
}

impl UIWidget for Collapsible {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}
