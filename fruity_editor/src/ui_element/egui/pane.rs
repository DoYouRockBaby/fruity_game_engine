use crate::hooks::use_state;
use crate::ui_element::egui::app::DrawContext;
use crate::ui_element::egui::draw_element;
use crate::ui_element::pane::Pane;
use crate::ui_element::pane::PaneGrid;
use crate::ui_element::pane::UIPaneSide;
use comp_state::CloneState;

pub fn draw_pane_grid<'a>(elem: PaneGrid, _ui: &mut egui::Ui, ctx: &mut DrawContext) {
    // Initialize the pane grid state
    let panes = elem.panes.clone();
    let left_panes = use_state(|| {
        panes
            .into_iter()
            .filter(|pane| pane.default_side == UIPaneSide::Left)
            .collect::<Vec<_>>()
    });

    let panes = elem.panes.clone();
    let right_panes = use_state(|| {
        panes
            .into_iter()
            .filter(|pane| pane.default_side == UIPaneSide::Right)
            .collect::<Vec<_>>()
    });

    let panes = elem.panes.clone();
    let bottom_panes = use_state(|| {
        panes
            .into_iter()
            .filter(|pane| pane.default_side == UIPaneSide::Bottom)
            .collect::<Vec<_>>()
    });

    egui::SidePanel::left("left_panel")
        .resizable(true)
        .default_width(150.0)
        .show(&ctx.platform.context(), |ui| {
            draw_pane(left_panes.get(), ui, ctx);
        });

    egui::SidePanel::right("right_panel")
        .resizable(true)
        .default_width(150.0)
        .show(&ctx.platform.context(), |ui| {
            draw_pane(right_panes.get(), ui, ctx);
        });

    egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(true)
        .default_height(150.0)
        .show(&ctx.platform.context(), |ui| {
            draw_pane(bottom_panes.get(), ui, ctx);
        });
}

pub fn draw_pane<'a>(panes: Vec<Pane>, ui: &mut egui::Ui, ctx: &mut DrawContext) {
    // TODO: Add tab system
    if let Some(first_pane) = panes.get(0) {
        ui.vertical_centered(|ui| {
            ui.heading(&first_pane.title);
        });

        draw_element((first_pane.render)(), ui, ctx)
    }
}
