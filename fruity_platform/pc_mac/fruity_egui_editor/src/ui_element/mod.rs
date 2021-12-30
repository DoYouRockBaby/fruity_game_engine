use crate::ui_element::app::DrawContext;
use crate::ui_element::display::draw_popup;
use crate::ui_element::display::draw_text;
use crate::ui_element::input::draw_button;
use crate::ui_element::input::draw_checkbox;
use crate::ui_element::input::draw_float_input;
use crate::ui_element::input::draw_image_button;
use crate::ui_element::input::draw_input;
use crate::ui_element::input::draw_integer_input;
use crate::ui_element::layout::draw_collapsible;
use crate::ui_element::layout::draw_column;
use crate::ui_element::layout::draw_empty;
use crate::ui_element::layout::draw_row;
use crate::ui_element::layout::draw_scroll;
use crate::ui_element::list::draw_list_view;
use crate::ui_element::menu::draw_menu_bar;
use crate::ui_element::menu::draw_menu_section;
use crate::ui_element::pane::draw_pane_grid;
use crate::ui_element::profiling::draw_profiling;
use crate::ui_element::scene::draw_scene;
use fruity_editor::hooks::topo;
use fruity_editor::ui_element::display::Popup;
use fruity_editor::ui_element::display::Text;
use fruity_editor::ui_element::input::Button;
use fruity_editor::ui_element::input::Checkbox;
use fruity_editor::ui_element::input::FloatInput;
use fruity_editor::ui_element::input::ImageButton;
use fruity_editor::ui_element::input::Input;
use fruity_editor::ui_element::input::IntegerInput;
use fruity_editor::ui_element::layout::Collapsible;
use fruity_editor::ui_element::layout::Column;
use fruity_editor::ui_element::layout::Row;
use fruity_editor::ui_element::layout::Scroll;
use fruity_editor::ui_element::list::ListView;
use fruity_editor::ui_element::menu::MenuBar;
use fruity_editor::ui_element::menu::MenuSection;
use fruity_editor::ui_element::pane::PaneGrid;
use fruity_editor::ui_element::profiling::Profiling;
use fruity_editor::ui_element::scene::Scene;
use fruity_editor::ui_element::UIElement;
use std::any::TypeId;

pub mod app;
pub mod display;
pub mod input;
pub mod layout;
pub mod list;
pub mod menu;
pub mod pane;
pub mod profiling;
pub mod scene;

#[topo::nested]
pub fn draw_element<'a>(elem: UIElement, ui: &mut egui::Ui, ctx: &mut DrawContext) {
    let type_id = elem.root.as_ref().type_id();

    if type_id == TypeId::of::<Text>() {
        draw_text(*elem.root.downcast::<Text>().unwrap(), ui, ctx)
    } else if type_id == TypeId::of::<Button>() {
        draw_button(*elem.root.downcast::<Button>().unwrap(), ui, ctx)
    } else if type_id == TypeId::of::<ImageButton>() {
        draw_image_button(*elem.root.downcast::<ImageButton>().unwrap(), ui, ctx)
    } else if type_id == TypeId::of::<Checkbox>() {
        draw_checkbox(*elem.root.downcast::<Checkbox>().unwrap(), ui, ctx)
    } else if type_id == TypeId::of::<FloatInput>() {
        draw_float_input(*elem.root.downcast::<FloatInput>().unwrap(), ui, ctx)
    } else if type_id == TypeId::of::<Input>() {
        draw_input(*elem.root.downcast::<Input>().unwrap(), ui, ctx)
    } else if type_id == TypeId::of::<IntegerInput>() {
        draw_integer_input(*elem.root.downcast::<IntegerInput>().unwrap(), ui, ctx)
    } else if type_id == TypeId::of::<Column>() {
        draw_column(*elem.root.downcast::<Column>().unwrap(), ui, ctx)
    } else if type_id == TypeId::of::<Row>() {
        draw_row(*elem.root.downcast::<Row>().unwrap(), ui, ctx)
    } else if type_id == TypeId::of::<Scroll>() {
        draw_scroll(*elem.root.downcast::<Scroll>().unwrap(), ui, ctx)
    } else if type_id == TypeId::of::<Collapsible>() {
        draw_collapsible(*elem.root.downcast::<Collapsible>().unwrap(), ui, ctx)
    } else if type_id == TypeId::of::<ListView>() {
        draw_list_view(*elem.root.downcast::<ListView>().unwrap(), ui, ctx)
    } else if type_id == TypeId::of::<PaneGrid>() {
        draw_pane_grid(*elem.root.downcast::<PaneGrid>().unwrap(), ui, ctx)
    } else if type_id == TypeId::of::<MenuBar>() {
        draw_menu_bar(*elem.root.downcast::<MenuBar>().unwrap(), ui, ctx)
    } else if type_id == TypeId::of::<MenuSection>() {
        draw_menu_section(*elem.root.downcast::<MenuSection>().unwrap(), ui, ctx)
    } else if type_id == TypeId::of::<Popup>() {
        draw_popup(*elem.root.downcast::<Popup>().unwrap(), ui, ctx)
    } else if type_id == TypeId::of::<Profiling>() {
        draw_profiling(ui, ctx)
    } else if type_id == TypeId::of::<Scene>() {
        draw_scene(ui, ctx)
    } else {
        draw_empty(ui)
    }
}
