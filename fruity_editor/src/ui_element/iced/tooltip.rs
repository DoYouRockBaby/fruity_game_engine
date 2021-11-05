use crate::hooks::use_global;
use crate::state::theme::ThemeState;
use crate::ui_element::iced::draw_element;
use crate::ui_element::tooltip::Tooltip;
use crate::ui_element::Message;
use iced::tooltip;
use iced_wgpu::Renderer;
use iced_winit::Element;
use iced_winit::Tooltip as IcedTooltip;

pub fn draw_tooltip<'a>(elem: Tooltip) -> Element<'a, Message, Renderer> {
    let theme_state = use_global::<ThemeState>();

    IcedTooltip::new(
        draw_element(elem.child),
        &elem.text,
        tooltip::Position::Bottom,
    )
    .gap(5)
    .padding(10)
    .size(16)
    .style(theme_state.theme.tooltip())
    .into()
}
