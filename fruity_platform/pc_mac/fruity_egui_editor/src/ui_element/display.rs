use crate::ui_element::app::DrawContext;
use crate::ui_element::draw_element;
use crate::ui_element::topo::CallId;
use fruity_editor::hooks::topo;
use fruity_editor::ui_element::display::Popup;
use fruity_editor::ui_element::display::Text;

pub fn draw_text<'a>(elem: Text, ui: &mut egui::Ui, _ctx: &mut DrawContext) {
    ui.add(egui::Label::new(elem.text));
}

#[topo::nested]
pub fn draw_popup<'a>(elem: Popup, ui: &mut egui::Ui, ctx: &mut DrawContext) {
    let popup_id = ui.make_persistent_id(CallId::current());

    let response =
        ui.allocate_response(egui::vec2(ui.available_size().x, 0.0), egui::Sense::click());
    egui::popup::popup_below_widget(ui, popup_id, &response, |ui| {
        draw_element(elem.content, ui, ctx)
    });
    ui.memory().open_popup(popup_id);
}
