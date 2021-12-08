use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;

pub struct Profiling {}

impl UIWidget for Profiling {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}
