use crate::state::Message;
use iced_wgpu::Renderer;
use iced_winit::Element;

pub mod display;
pub mod input;
pub mod layout;
pub mod list;

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

pub trait UIWidget {
    fn draw<'a>(&self) -> Element<'a, Message, Renderer>;
    fn elem(self) -> UIElement;
}

pub struct UIElement {
    root: Box<dyn UIWidget>,
}

impl UIElement {
    pub fn draw<'a>(&self) -> Element<'a, Message, Renderer> {
        self.root.draw()
    }
}
