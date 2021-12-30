use crate::ui_element::app::DrawContext;
use crate::ui_element::draw_element;
use comp_state::CloneState;
use fruity_editor::hooks::use_state;
use fruity_editor::ui_element::pane::Pane;
use fruity_editor::ui_element::pane::PaneGrid;
use fruity_editor::ui_element::pane::UIPaneSide;

pub fn draw_pane_grid<'a>(elem: PaneGrid, _ui: &mut egui::Ui, ctx: &mut DrawContext) {
    // Initialize the pane grid state
    let panes = elem.panes.clone();
    let center_panes = use_state(|| {
        panes
            .into_iter()
            .filter(|pane| pane.default_side == UIPaneSide::Center)
            .collect::<Vec<_>>()
    });

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

    egui::CentralPanel::default().show(&ctx.platform.context(), |ui| {
        draw_pane(center_panes.get(), ui, ctx);
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
    let current_tab = use_state(|| usize::default());
    let mut current_tab_value = current_tab.get();

    ui.horizontal(|ui| {
        panes.iter().enumerate().for_each(|(index, pane)| {
            ui.selectable_value(&mut current_tab_value, index, &pane.title);
        });
    });
    ui.end_row();
    current_tab.set(current_tab_value);

    if let Some(current_pane) = panes.get(current_tab.get()) {
        draw_element((current_pane.render)(), ui, ctx)
    }
}
