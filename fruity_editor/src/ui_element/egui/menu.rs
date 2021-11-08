use crate::hooks::topo;
use crate::ui_element::egui::app::DrawContext;
use crate::ui_element::menu::MenuBar;
use egui::menu;

#[topo::nested]
pub fn draw_menu_bar<'a>(elem: MenuBar, _ui: &mut egui::Ui, ctx: &mut DrawContext) {
    egui::TopBottomPanel::top("menu_bar").show(&ctx.platform.context(), |ui| {
        menu::bar(ui, |ui| {
            elem.sections.into_iter().for_each(|section| {
                menu::menu(ui, section.label, {
                    let items = section.items;
                    |ui| {
                        items.into_iter().for_each({
                            |item| {
                                if ui.button(item.label).clicked() {
                                    (item.on_click)()
                                }
                            }
                        });
                    }
                });
            });
        });
    });
}
