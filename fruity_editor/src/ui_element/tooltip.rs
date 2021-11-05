use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;

pub struct Tooltip {
    pub child: UIElement,
    pub text: String,
}

impl UIWidget for Tooltip {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}
