use crate::hooks::topo;
use crate::ui_element::menu::MenuBar;
use egui::menu;

#[topo::nested]
pub fn draw_menu_bar<'a>(elem: MenuBar, ui: &mut egui::Ui) {
    egui::TopBottomPanel::top("menu_bar").show_inside(ui, |ui| {
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
