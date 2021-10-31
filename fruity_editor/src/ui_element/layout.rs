use crate::ui_element::UIAlign;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;

pub struct Empty {}

impl UIWidget for Empty {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}

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
