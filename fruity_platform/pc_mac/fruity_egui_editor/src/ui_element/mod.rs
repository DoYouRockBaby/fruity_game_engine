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
use fruity_editor::ui::context::UIContext;
use fruity_editor::ui::elements::display::Popup;
use fruity_editor::ui::elements::display::Text;
use fruity_editor::ui::elements::input::Button;
use fruity_editor::ui::elements::input::Checkbox;
use fruity_editor::ui::elements::input::FloatInput;
use fruity_editor::ui::elements::input::ImageButton;
use fruity_editor::ui::elements::input::Input;
use fruity_editor::ui::elements::input::IntegerInput;
use fruity_editor::ui::elements::layout::Collapsible;
use fruity_editor::ui::elements::layout::Column;
use fruity_editor::ui::elements::layout::Row;
use fruity_editor::ui::elements::layout::Scroll;
use fruity_editor::ui::elements::list::ListView;
use fruity_editor::ui::elements::menu::MenuBar;
use fruity_editor::ui::elements::menu::MenuSection;
use fruity_editor::ui::elements::pane::PaneGrid;
use fruity_editor::ui::elements::profiling::Profiling;
use fruity_editor::ui::elements::scene::Scene;
use fruity_editor::ui::elements::UIElement;
use fruity_editor::ui::elements::UIElementContent;
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

pub fn draw_element<'a>(
    elem: UIElement,
    ctx: &mut UIContext,
    ui: &mut egui::Ui,
    draw_ctx: &mut DrawContext,
) {
    match elem.content {
        UIElementContent::Func(func) => {
            let elem = func(ctx);
            draw_element(elem, &mut ctx.new_child(), ui, draw_ctx);
        }
        UIElementContent::Widget(widget) => {
            let type_id = widget.as_ref().type_id();
            let widget = widget.as_any_box();

            if type_id == TypeId::of::<Text>() {
                draw_text(
                    *widget.downcast::<Text>().unwrap(),
                    &mut ctx.new_child(),
                    ui,
                    draw_ctx,
                )
            } else if type_id == TypeId::of::<Button>() {
                draw_button(
                    *widget.downcast::<Button>().unwrap(),
                    &mut ctx.new_child(),
                    ui,
                    draw_ctx,
                )
            } else if type_id == TypeId::of::<ImageButton>() {
                draw_image_button(
                    *widget.downcast::<ImageButton>().unwrap(),
                    ctx,
                    ui,
                    draw_ctx,
                )
            } else if type_id == TypeId::of::<Checkbox>() {
                draw_checkbox(
                    *widget.downcast::<Checkbox>().unwrap(),
                    &mut ctx.new_child(),
                    ui,
                    draw_ctx,
                )
            } else if type_id == TypeId::of::<FloatInput>() {
                draw_float_input(
                    *widget.downcast::<FloatInput>().unwrap(),
                    &mut ctx.new_child(),
                    ui,
                    draw_ctx,
                )
            } else if type_id == TypeId::of::<Input>() {
                draw_input(
                    *widget.downcast::<Input>().unwrap(),
                    &mut ctx.new_child(),
                    ui,
                    draw_ctx,
                )
            } else if type_id == TypeId::of::<IntegerInput>() {
                draw_integer_input(
                    *widget.downcast::<IntegerInput>().unwrap(),
                    &mut ctx.new_child(),
                    ui,
                    draw_ctx,
                )
            } else if type_id == TypeId::of::<Column>() {
                draw_column(
                    *widget.downcast::<Column>().unwrap(),
                    &mut ctx.new_child(),
                    ui,
                    draw_ctx,
                )
            } else if type_id == TypeId::of::<Row>() {
                draw_row(
                    *widget.downcast::<Row>().unwrap(),
                    &mut ctx.new_child(),
                    ui,
                    draw_ctx,
                )
            } else if type_id == TypeId::of::<Scroll>() {
                draw_scroll(
                    *widget.downcast::<Scroll>().unwrap(),
                    &mut ctx.new_child(),
                    ui,
                    draw_ctx,
                )
            } else if type_id == TypeId::of::<Collapsible>() {
                draw_collapsible(
                    *widget.downcast::<Collapsible>().unwrap(),
                    &mut ctx.new_child(),
                    ui,
                    draw_ctx,
                )
            } else if type_id == TypeId::of::<ListView>() {
                draw_list_view(
                    *widget.downcast::<ListView>().unwrap(),
                    &mut ctx.new_child(),
                    ui,
                    draw_ctx,
                )
            } else if type_id == TypeId::of::<PaneGrid>() {
                draw_pane_grid(
                    *widget.downcast::<PaneGrid>().unwrap(),
                    &mut ctx.new_child(),
                    ui,
                    draw_ctx,
                )
            } else if type_id == TypeId::of::<MenuBar>() {
                draw_menu_bar(
                    *widget.downcast::<MenuBar>().unwrap(),
                    &mut ctx.new_child(),
                    ui,
                    draw_ctx,
                )
            } else if type_id == TypeId::of::<MenuSection>() {
                draw_menu_section(
                    *widget.downcast::<MenuSection>().unwrap(),
                    &mut ctx.new_child(),
                    ui,
                    draw_ctx,
                )
            } else if type_id == TypeId::of::<Popup>() {
                draw_popup(
                    *widget.downcast::<Popup>().unwrap(),
                    &mut ctx.new_child(),
                    ui,
                    draw_ctx,
                )
            } else if type_id == TypeId::of::<Profiling>() {
                draw_profiling(&mut ctx.new_child(), ui, draw_ctx)
            } else if type_id == TypeId::of::<Scene>() {
                draw_scene(&mut ctx.new_child(), ui, draw_ctx)
            } else {
                draw_empty(ui)
            }
        }
    };
}
