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
