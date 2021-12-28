use crate::ui_element::menu::MenuItem;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_graphic::resources::texture_resource::TextureResource;
use std::any::Any;
use std::sync::Arc;

pub struct Button {
    pub label: String,
    pub enabled: bool,
    pub on_click: Arc<dyn Fn() + Send + Sync>,
    pub secondary_actions: Vec<MenuItem>,
    pub drag_item: Option<Arc<dyn Any + Send + Sync>>,
    pub accept_drag: Option<Arc<dyn Fn(&dyn Any) -> bool>>,
    pub on_drag: Option<Arc<dyn Fn(&dyn Any)>>,
}

impl Default for Button {
    fn default() -> Self {
        Button {
            label: Default::default(),
            enabled: true,
            on_click: Arc::new(|| {}),
            secondary_actions: Vec::new(),
            drag_item: None,
            accept_drag: None,
            on_drag: None,
        }
    }
}

impl UIWidget for Button {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}

pub struct ImageButton {
    pub image: ResourceReference<dyn TextureResource>,
    pub enabled: bool,
    pub on_click: Arc<dyn Fn() + Send + Sync>,
    pub width: f32,
    pub height: f32,
    pub drag_item: Option<Arc<dyn Any + Send + Sync>>,
    pub accept_drag: Option<Arc<dyn Fn(&dyn Any) -> bool>>,
    pub on_drag: Option<Arc<dyn Fn(&dyn Any) -> bool>>,
}

impl UIWidget for ImageButton {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}

pub struct Input {
    pub value: String,
    pub placeholder: String,
    pub on_change: Arc<dyn Fn(&str) + Send + Sync>,
    pub on_edit: Arc<dyn Fn(&str) + Send + Sync>,
}

impl Default for Input {
    fn default() -> Self {
        Input {
            value: String::default(),
            placeholder: String::default(),
            on_change: Arc::new(|_| {}),
            on_edit: Arc::new(|_| {}),
        }
    }
}

impl UIWidget for Input {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}

pub struct IntegerInput {
    pub value: i64,
    pub on_change: Arc<dyn Fn(i64) + Send + Sync>,
}

impl UIWidget for IntegerInput {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}

pub struct FloatInput {
    pub value: f64,
    pub on_change: Arc<dyn Fn(f64) + Send + Sync>,
}

impl UIWidget for FloatInput {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}

pub struct Checkbox {
    pub label: String,
    pub value: bool,
    pub on_change: Arc<dyn Fn(bool) + Send + Sync>,
}

impl UIWidget for Checkbox {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}
