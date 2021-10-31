use crate::ui_element::display::Text;
use crate::ui_element::Message;
use iced::Text as IcedText;
use iced_wgpu::Renderer;
use iced_winit::Element;

pub fn draw_text<'a>(elem: Text) -> Element<'a, Message, Renderer> {
    IcedText::new(elem.text).size(16).into()
}
