use crate::hooks::use_global;
use crate::state::theme::ThemeState;
use crate::ui_element::iced::draw_element;
use crate::ui_element::layout::Column;
use crate::ui_element::layout::Row;
use crate::ui_element::Message;
use iced::Column as IcedColumn;
use iced::Container as IcedContainer;
use iced::Row as IcedRow;
use iced_wgpu::Renderer;
use iced_winit::Element;

pub fn draw_empty<'a>() -> Element<'a, Message, Renderer> {
    IcedRow::new().into()
}

pub fn draw_row<'a>(elem: Row) -> Element<'a, Message, Renderer> {
    let theme_state = use_global::<ThemeState>();

    IcedContainer::new(elem.children.into_iter().fold(
        IcedRow::new().align_items(elem.align.clone().into()),
        |row, element| row.push(draw_element(element)),
    ))
    .style(theme_state.theme)
    .into()
}

pub fn draw_column<'a>(elem: Column) -> Element<'a, Message, Renderer> {
    let theme_state = use_global::<ThemeState>();

    IcedContainer::new(elem.children.into_iter().fold(
        IcedColumn::new().align_items(elem.align.clone().into()),
        |row, element| row.push(draw_element(element)),
    ))
    .style(theme_state.theme)
    .into()
}
