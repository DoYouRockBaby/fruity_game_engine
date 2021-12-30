use crate::ui_element::UIElement;
use crate::UIWidget;

#[derive(Default)]
pub struct Scene {}

impl UIWidget for Scene {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}
