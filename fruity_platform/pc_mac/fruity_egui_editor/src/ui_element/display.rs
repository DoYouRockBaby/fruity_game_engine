use crate::ui_element::app::DrawContext;
use fruity_editor::ui_element::display::Text;

pub fn draw_text<'a>(elem: Text, ui: &mut egui::Ui, _ctx: &mut DrawContext) {
    ui.add(egui::Label::new(elem.text));
}
