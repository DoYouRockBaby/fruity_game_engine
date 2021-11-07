use crate::ui_element::display::Text;
use crate::ui_element::egui::app::DrawContext;

pub fn draw_text<'a>(elem: Text, ui: &mut egui::Ui, _ctx: &mut DrawContext) {
    ui.label(elem.text);
}
