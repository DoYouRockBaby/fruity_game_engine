use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;

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
