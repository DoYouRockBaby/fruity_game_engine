use crate::state::Message;
use std::any::Any;

pub mod display;
pub mod iced;
pub mod input;
pub mod layout;
pub mod list;
pub mod pane;

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
    FillPortion(u16),
    Shrink,
    Units(u16),
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
    root: Box<dyn Any>,
}
