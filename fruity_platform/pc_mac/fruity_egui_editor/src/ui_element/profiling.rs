use crate::ui_element::DrawContext;

pub fn draw_profiling(ui: &mut egui::Ui, _ctx: &mut DrawContext) {
    puffin_egui::profiler_ui(ui);
}
