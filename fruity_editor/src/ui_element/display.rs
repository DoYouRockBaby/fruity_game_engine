use crate::ui_element::Empty;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;

pub enum ImageSource {
    Local { path: String },
}

pub struct Text {
    pub text: String,
}

impl Default for Text {
    fn default() -> Self {
        Text {
            text: "".to_string(),
        }
    }
}

impl UIWidget for Text {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}

pub struct Popup {
    pub content: UIElement,
}

impl Default for Popup {
    fn default() -> Self {
        Self {
            content: Empty {}.elem(),
        }
    }
}

impl UIWidget for Popup {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}
