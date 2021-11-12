use crate::ui_element::app::DrawContext;
use crate::ui_element::draw_element;
use egui::ScrollArea;
use fruity_editor::hooks::topo;
use fruity_editor::ui_element::list::ListView;
use std::ops::Deref;

#[topo::nested]
pub fn draw_list_view<'a>(elem: ListView, ui: &mut egui::Ui, ctx: &mut DrawContext) {
    let scroll_area = ScrollArea::vertical().auto_shrink([false; 2]);

    let render_item = elem.render_item.clone();
    scroll_area.show(ui, |ui| {
        ui.vertical(|ui| {
            elem.items.into_iter().for_each(|item| {
                let item = render_item(item.deref());

                draw_element(item, ui, ctx)
            })
        });
    });
}
