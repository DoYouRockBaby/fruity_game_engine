use crate::ui_element::DrawContext;
use fruity_editor::ui::context::UIContext;

pub fn draw_profiling(_ctx: &mut UIContext, ui: &mut egui::Ui, _draw_ctx: &mut DrawContext) {
    puffin_egui::profiler_ui(ui);
}
