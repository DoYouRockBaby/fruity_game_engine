use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use std::any::Any;
use std::sync::Arc;

pub struct ListView {
    pub items: Vec<Arc<dyn Any + Send + Sync>>,
    pub get_key: Box<dyn Fn(&dyn Any) -> usize + Send + Sync>,
    pub render_item: Box<dyn Fn(&dyn Any) -> UIElement + Send + Sync>,
    pub on_clicked: Arc<dyn Fn(&dyn Any) + Send + Sync>,
}

impl UIWidget for ListView {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}
