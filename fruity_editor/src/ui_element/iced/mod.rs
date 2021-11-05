use crate::hooks::topo;
use crate::ui_element::display::Text;
use crate::ui_element::iced::display::draw_text;
use crate::ui_element::iced::input::draw_button;
use crate::ui_element::iced::input::draw_checkbox;
use crate::ui_element::iced::input::draw_float_input;
use crate::ui_element::iced::input::draw_input;
use crate::ui_element::iced::input::draw_integer_input;
use crate::ui_element::iced::layout::draw_column;
use crate::ui_element::iced::layout::draw_container;
use crate::ui_element::iced::layout::draw_empty;
use crate::ui_element::iced::layout::draw_row;
use crate::ui_element::iced::list::draw_list_view;
use crate::ui_element::iced::menu::draw_menu;
use crate::ui_element::iced::pane::draw_pane_grid;
use crate::ui_element::iced::tooltip::draw_tooltip;
use crate::ui_element::input::Button;
use crate::ui_element::input::Checkbox;
use crate::ui_element::input::FloatInput;
use crate::ui_element::input::Input;
use crate::ui_element::input::IntegerInput;
use crate::ui_element::layout::Column;
use crate::ui_element::layout::Container;
use crate::ui_element::layout::Row;
use crate::ui_element::list::ListView;
use crate::ui_element::menu::Menu;
use crate::ui_element::pane::PaneGrid;
use crate::ui_element::tooltip::Tooltip;
use crate::ui_element::Message;
use crate::ui_element::UIElement;
use iced_wgpu::Renderer;
use iced_winit::Element;
use std::any::TypeId;

pub mod display;
pub mod input;
pub mod layout;
pub mod list;
pub mod menu;
pub mod pane;
pub mod program;
pub mod tooltip;

#[topo::nested]
pub fn draw_element<'a>(elem: UIElement) -> Element<'a, Message, Renderer> {
    let type_id = elem.root.as_ref().type_id();

    if type_id == TypeId::of::<Text>() {
        draw_text(*elem.root.downcast::<Text>().unwrap())
    } else if type_id == TypeId::of::<Button>() {
        draw_button(*elem.root.downcast::<Button>().unwrap())
    } else if type_id == TypeId::of::<Checkbox>() {
        draw_checkbox(*elem.root.downcast::<Checkbox>().unwrap())
    } else if type_id == TypeId::of::<FloatInput>() {
        draw_float_input(*elem.root.downcast::<FloatInput>().unwrap())
    } else if type_id == TypeId::of::<Input>() {
        draw_input(*elem.root.downcast::<Input>().unwrap())
    } else if type_id == TypeId::of::<IntegerInput>() {
        draw_integer_input(*elem.root.downcast::<IntegerInput>().unwrap())
    } else if type_id == TypeId::of::<Container>() {
        draw_container(*elem.root.downcast::<Container>().unwrap())
    } else if type_id == TypeId::of::<Column>() {
        draw_column(*elem.root.downcast::<Column>().unwrap())
    } else if type_id == TypeId::of::<Row>() {
        draw_row(*elem.root.downcast::<Row>().unwrap())
    } else if type_id == TypeId::of::<ListView>() {
        draw_list_view(*elem.root.downcast::<ListView>().unwrap())
    } else if type_id == TypeId::of::<PaneGrid>() {
        draw_pane_grid(*elem.root.downcast::<PaneGrid>().unwrap())
    } else if type_id == TypeId::of::<Menu>() {
        draw_menu(*elem.root.downcast::<Menu>().unwrap())
    } else if type_id == TypeId::of::<Tooltip>() {
        draw_tooltip(*elem.root.downcast::<Tooltip>().unwrap())
    } else {
        draw_empty()
    }
}
