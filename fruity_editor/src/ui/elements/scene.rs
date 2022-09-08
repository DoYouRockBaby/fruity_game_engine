use crate::ui::elements::UIElement;
use crate::UIWidget;
use fruity_any::*;

#[derive(FruityAny, Default)]
pub struct Scene {}

impl UIWidget for Scene {
    fn elem(self) -> UIElement {
        UIElement::from_widget(self)
    }
}
