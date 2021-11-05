use crate::ui_element::display::Text;

pub fn draw_text<'a>(elem: Text, ui: &mut egui::Ui) {
    ui.label(elem.text);
}
