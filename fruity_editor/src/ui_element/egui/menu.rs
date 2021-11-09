use crate::hooks::topo;
use crate::ui_element::egui::app::DrawContext;
use crate::ui_element::egui::draw_element;
use crate::ui_element::menu::MenuBar;
use crate::ui_element::menu::MenuSection;
use egui::menu;

#[topo::nested]
pub fn draw_menu_bar<'a>(elem: MenuBar, _ui: &mut egui::Ui, ctx: &mut DrawContext) {
    egui::TopBottomPanel::top("menu_bar").show(&ctx.platform.context(), |ui| {
        menu::bar(ui, |ui| {
            elem.children
                .into_iter()
                .for_each(|child| draw_element(child, ui, ctx));
        });
    });
}

#[topo::nested]
pub fn draw_menu_section<'a>(elem: MenuSection, ui: &mut egui::Ui, _ctx: &mut DrawContext) {
    menu::menu(ui, elem.label, {
        let items = elem.items;
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
}
