use crate::hooks::topo;
use crate::ui_element::egui::draw_element;
use crate::ui_element::list::ListView;
use egui::ScrollArea;
use std::ops::Deref;

#[topo::nested]
pub fn draw_list_view<'a>(elem: ListView, ui: &mut egui::Ui) {
    let mut scroll_area = ScrollArea::vertical()
        .max_height(200.0)
        .auto_shrink([false; 2]);

    let render_item = elem.render_item.clone();
    let (current_scroll, max_scroll) = scroll_area.show(ui, |ui| {
        ui.vertical(|ui| {
            elem.items.into_iter().for_each(|item| {
                let item = render_item(item.deref());

                draw_element(item, ui)
            })
        });

        let margin = ui.visuals().clip_rect_margin;

        let current_scroll = ui.clip_rect().top() - ui.min_rect().top() + margin;
        let max_scroll = ui.min_rect().height() - ui.clip_rect().height() + 2.0 * margin;
        (current_scroll, max_scroll)
    });
}
