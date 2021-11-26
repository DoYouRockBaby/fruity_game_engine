use crate::ui_element::app::DrawContext;
use crate::ui_element::draw_element;
use egui::CollapsingHeader;
use egui::ScrollArea;
use fruity_editor::ui_element::layout::Collapsible;
use fruity_editor::ui_element::layout::Column;
use fruity_editor::ui_element::layout::Row;
use fruity_editor::ui_element::layout::Scroll;
use fruity_editor::ui_element::UISize;

pub fn draw_empty<'a>(_ui: &mut egui::Ui) {}

pub fn draw_row<'a>(elem: Row, ui: &mut egui::Ui, ctx: &mut DrawContext) {
    // Get the base available space informations
    let available_width = ui.available_size_before_wrap().x;
    let origin_pos = ui.available_rect_before_wrap().left_top();

    // If the size have changed or new child has been added, we the child line break should be changes
    let mut allocated_rect = egui::Rect::from_min_size(origin_pos, egui::Vec2::new(0.0, 0.0));
    let mut relative_pos = egui::Vec2::new(0.0, 0.0);
    let mut current_non_units_width = available_width;
    let mut current_line_height = 0.0;

    for child in elem.children.into_iter() {
        // Get the elem width
        let child_width = match child.size {
            UISize::Fill => current_non_units_width,
            UISize::FillPortion(portion) => current_non_units_width * portion,
            UISize::Units(units) => {
                current_non_units_width -= units;
                units
            }
        };

        // Proceed newline if needed
        let remaining_width = available_width - relative_pos.x;
        if remaining_width < child_width {
            relative_pos.x = 0.0;
            relative_pos.y += current_line_height;
            current_line_height = 0.0;
            current_non_units_width = available_width;
        }

        // Build the base child rect
        let child_rect = egui::Rect::from_min_size(
            origin_pos + relative_pos,
            egui::Vec2::new(
                child_width - ui.spacing().item_spacing.x,
                ui.spacing().interact_size.y,
            ),
        );

        let mut child_ui = ui.child_ui(
            child_rect,
            egui::Layout::top_down_justified(egui::Align::LEFT),
        );

        // Draw the child
        draw_element(child.child, &mut child_ui, ctx);
        let final_child_rect = child_ui.min_rect();

        // We update the position where the next child will be rendered
        relative_pos +=
            egui::Vec2::new(final_child_rect.size().x + ui.spacing().item_spacing.x, 0.0);
        current_line_height = f32::max(
            final_child_rect.size().y + ui.spacing().item_spacing.y,
            current_line_height,
        );

        // We update the global rect
        allocated_rect.max = egui::Pos2::new(
            f32::max(allocated_rect.max.x, final_child_rect.max.x),
            f32::max(allocated_rect.max.y, final_child_rect.max.y),
        );
    }

    // Request the drawed rect to make egui aware and react about all the child
    ui.allocate_rect(allocated_rect, egui::Sense::click());
}

pub fn draw_column<'a>(elem: Column, ui: &mut egui::Ui, ctx: &mut DrawContext) {
    ui.vertical(|ui| {
        elem.children
            .into_iter()
            .for_each(|child| draw_element(child, ui, ctx));
    });
}

pub fn draw_scroll<'a>(elem: Scroll, ui: &mut egui::Ui, ctx: &mut DrawContext) {
    let scroll_area = match (elem.horizontal, elem.vertical) {
        (false, false) => ScrollArea::neither().auto_shrink([false; 2]),
        (true, false) => ScrollArea::horizontal().auto_shrink([false; 2]),
        (false, true) => ScrollArea::vertical().auto_shrink([false; 2]),
        (true, true) => ScrollArea::both().auto_shrink([false; 2]),
    };

    scroll_area.show(ui, |ui| draw_element(elem.child, ui, ctx));
}

pub fn draw_collapsible<'a>(elem: Collapsible, ui: &mut egui::Ui, ctx: &mut DrawContext) {
    let title = elem.title.clone();
    let on_click = elem.on_click.clone();
    let response = CollapsingHeader::new(title)
        .selectable(true)
        .show(ui, |ui| draw_element(elem.child, ui, ctx));

    if response.header_response.clicked() {
        if let Some(on_click) = on_click {
            on_click();
        }
    }
}
