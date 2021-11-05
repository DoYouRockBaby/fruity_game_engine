use crate::hooks::topo;
use crate::ui_element::display::Text;
use crate::ui_element::egui::display::draw_text;
use crate::ui_element::egui::input::draw_button;
use crate::ui_element::egui::input::draw_checkbox;
use crate::ui_element::egui::input::draw_float_input;
use crate::ui_element::egui::input::draw_input;
use crate::ui_element::egui::input::draw_integer_input;
use crate::ui_element::egui::layout::draw_column;
use crate::ui_element::egui::layout::draw_container;
use crate::ui_element::egui::layout::draw_empty;
use crate::ui_element::egui::layout::draw_row;
use crate::ui_element::egui::list::draw_list_view;
use crate::ui_element::egui::menu::draw_menu_bar;
use crate::ui_element::egui::pane::draw_pane_grid;
use crate::ui_element::input::Button;
use crate::ui_element::input::Checkbox;
use crate::ui_element::input::FloatInput;
use crate::ui_element::input::Input;
use crate::ui_element::input::IntegerInput;
use crate::ui_element::layout::Column;
use crate::ui_element::layout::Container;
use crate::ui_element::layout::Row;
use crate::ui_element::list::ListView;
use crate::ui_element::menu::MenuBar;
use crate::ui_element::pane::PaneGrid;
use crate::ui_element::UIElement;
use std::any::TypeId;

pub mod app;
pub mod display;
pub mod input;
pub mod layout;
pub mod list;
pub mod menu;
pub mod pane;

#[topo::nested]
pub fn draw_element<'a>(elem: UIElement, ui: &mut egui::Ui) {
    let type_id = elem.root.as_ref().type_id();

    if type_id == TypeId::of::<Text>() {
        draw_text(*elem.root.downcast::<Text>().unwrap(), ui)
    } else if type_id == TypeId::of::<Button>() {
        draw_button(*elem.root.downcast::<Button>().unwrap(), ui)
    } else if type_id == TypeId::of::<Checkbox>() {
        draw_checkbox(*elem.root.downcast::<Checkbox>().unwrap(), ui)
    } else if type_id == TypeId::of::<FloatInput>() {
        draw_float_input(*elem.root.downcast::<FloatInput>().unwrap(), ui)
    } else if type_id == TypeId::of::<Input>() {
        draw_input(*elem.root.downcast::<Input>().unwrap(), ui)
    } else if type_id == TypeId::of::<IntegerInput>() {
        draw_integer_input(*elem.root.downcast::<IntegerInput>().unwrap(), ui)
    } else if type_id == TypeId::of::<Container>() {
        draw_container(*elem.root.downcast::<Container>().unwrap(), ui)
    } else if type_id == TypeId::of::<Column>() {
        draw_column(*elem.root.downcast::<Column>().unwrap(), ui)
    } else if type_id == TypeId::of::<Row>() {
        draw_row(*elem.root.downcast::<Row>().unwrap(), ui)
    } else if type_id == TypeId::of::<ListView>() {
        draw_list_view(*elem.root.downcast::<ListView>().unwrap(), ui)
    } else if type_id == TypeId::of::<PaneGrid>() {
        draw_pane_grid(*elem.root.downcast::<PaneGrid>().unwrap(), ui)
    } else if type_id == TypeId::of::<MenuBar>() {
        draw_menu_bar(*elem.root.downcast::<MenuBar>().unwrap(), ui)
    } else {
        draw_empty(ui)
    }
}
