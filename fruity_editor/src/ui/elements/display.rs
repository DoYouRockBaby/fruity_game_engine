use crate::ui::elements::Empty;
use crate::ui::elements::UIElement;
use crate::ui::elements::UIWidget;
use fruity_any::*;

pub enum ImageSource {
    Local { path: String },
}

#[derive(FruityAny)]
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
        UIElement::from_widget(self)
    }
}

#[derive(FruityAny)]
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
        UIElement::from_widget(self)
    }
}
