use crate::ui::elements::UIContext;
use crate::ui::elements::UIElement;
use crate::ui::elements::UIWidget;
use fruity_any::*;
use std::any::Any;
use std::sync::Arc;

#[derive(FruityAny)]
pub struct ListView {
    pub items: Vec<Arc<dyn Any + Send + Sync>>,
    pub render_item: Arc<dyn Fn(&mut UIContext, &dyn Any) -> UIElement + Send + Sync>,
}

impl UIWidget for ListView {
    fn elem(self) -> UIElement {
        UIElement::from_widget(self)
    }
}
