use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use std::any::Any;
use std::sync::Arc;

pub struct ListView {
    pub items: Vec<Arc<dyn Any + Send + Sync>>,
    pub render_item: Arc<dyn Fn(&dyn Any) -> UIElement + Send + Sync>,
}

impl UIWidget for ListView {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}
