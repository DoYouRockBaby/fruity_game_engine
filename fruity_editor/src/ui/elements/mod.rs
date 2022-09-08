use crate::ui::context::UIContext;
use crate::ui::elements::layout::Empty;
use fruity_any::FruityAny;

pub mod display;
pub mod input;
pub mod layout;
pub mod list;
pub mod menu;
pub mod pane;
pub mod profiling;
pub mod scene;

#[derive(Debug, Clone)]
pub enum UIAlign {
    Start,
    Center,
    End,
}

impl Default for UIAlign {
    fn default() -> Self {
        UIAlign::Start
    }
}

#[derive(Debug, Clone)]
pub enum UISize {
    Fill,
    FillPortion(f32),
    Units(f32),
}

impl Default for UISize {
    fn default() -> Self {
        UISize::Fill
    }
}

pub trait UIWidget: FruityAny {
    fn elem(self) -> UIElement;
}

pub enum UIElementContent {
    Widget(Box<dyn UIWidget>),
    Func(Box<dyn Fn(&mut UIContext) -> UIElement + Send + Sync>),
}

impl Default for UIElementContent {
    fn default() -> Self {
        Self::Widget(Box::new(Empty {}))
    }
}

#[derive(Default)]
pub struct UIElement {
    pub key: Option<String>,
    pub content: UIElementContent,
}

impl UIElement {
    pub fn from_widget(widget: impl UIWidget) -> Self {
        Self {
            key: None,
            content: UIElementContent::Widget(Box::new(widget)),
        }
    }
}
