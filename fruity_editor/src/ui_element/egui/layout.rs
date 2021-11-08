use crate::ui_element::egui::app::DrawContext;
use crate::ui_element::egui::custom_layout::flex_row_layout;
use crate::ui_element::egui::draw_element;
use crate::ui_element::layout::Collapsible;
use crate::ui_element::layout::Column;
use crate::ui_element::layout::Container;
use crate::ui_element::layout::Row;
use crate::ui_element::layout::Scroll;
use egui::ScrollArea;

pub fn draw_empty<'a>(_ui: &mut egui::Ui) {}

pub fn draw_container<'a>(elem: Container, ui: &mut egui::Ui, ctx: &mut DrawContext) {
    ui.horizontal(|ui| {
        draw_element(elem.child, ui, ctx);
    });
}

pub fn draw_row<'a>(elem: Row, ui: &mut egui::Ui, ctx: &mut DrawContext) {
    if elem.children_with_same_width {
        ui.columns(elem.children.len(), |ui| {
            elem.children
                .into_iter()
                .enumerate()
                .for_each(|(index, child)| draw_element(child, &mut ui[index], ctx));
        });
    } else if elem.wrapped {
        flex_row_layout(elem.children, ui, ctx);
    } else {
        ui.horizontal(|ui| {
            elem.children
                .into_iter()
                .for_each(|child| draw_element(child, ui, ctx));
        });
    }
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
    ui.collapsing(title, |ui| draw_element(elem.child, ui, ctx));
}
