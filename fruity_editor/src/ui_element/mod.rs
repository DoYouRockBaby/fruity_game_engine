use crate::ui_element::layout::Empty;
use std::any::Any;

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

pub trait UIWidget: Any {
    fn elem(self) -> UIElement;
}

pub struct UIElement {
    pub root: Box<dyn Any>,
}

impl Default for UIElement {
    fn default() -> Self {
        Empty {}.elem()
    }
}
