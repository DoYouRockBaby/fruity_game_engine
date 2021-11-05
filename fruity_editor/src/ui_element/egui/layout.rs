use crate::ui_element::egui::draw_element;
use crate::ui_element::layout::Column;
use crate::ui_element::layout::Container;
use crate::ui_element::layout::Row;

pub fn draw_empty<'a>(_ui: &mut egui::Ui) {}

pub fn draw_container<'a>(elem: Container, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        draw_element(elem.child, ui);
    });
}

pub fn draw_row<'a>(elem: Row, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        elem.children
            .into_iter()
            .for_each(|child| draw_element(child, ui));
    });
}

pub fn draw_column<'a>(elem: Column, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        elem.children
            .into_iter()
            .for_each(|child| draw_element(child, ui));
    });
}
