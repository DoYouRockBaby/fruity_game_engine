use crate::ui_element::app::DrawContext;
use crate::ui_element::draw_element;
use fruity_editor::ui::context::UIContext;
use fruity_editor::ui::elements::pane::Pane;
use fruity_editor::ui::elements::pane::PaneGrid;
use fruity_editor::ui::elements::pane::UIPaneSide;
use fruity_editor::ui::hooks::use_state;

pub fn draw_pane_grid<'a>(
    elem: PaneGrid,
    ctx: &mut UIContext,
    _ui: &mut egui::Ui,
    draw_ctx: &mut DrawContext,
) {
    // Initialize the pane grid state
    let panes = elem.panes.clone();
    let left_panes = panes
        .into_iter()
        .filter(|pane| pane.default_side == UIPaneSide::Left)
        .collect::<Vec<_>>();

    let panes = elem.panes.clone();
    let right_panes = panes
        .into_iter()
        .filter(|pane| pane.default_side == UIPaneSide::Right)
        .collect::<Vec<_>>();

    let panes = elem.panes.clone();
    let bottom_panes = panes
        .into_iter()
        .filter(|pane| pane.default_side == UIPaneSide::Bottom)
        .collect::<Vec<_>>();

    let panes = elem.panes.clone();
    let center_panes = panes
        .into_iter()
        .filter(|pane| pane.default_side == UIPaneSide::Center)
        .collect::<Vec<_>>();

    egui::SidePanel::left("left_panel")
        .resizable(true)
        .default_width(150.0)
        .show(&draw_ctx.platform.context(), |ui| {
            draw_pane(left_panes, ctx, ui, draw_ctx);
        });

    egui::SidePanel::right("right_panel")
        .resizable(true)
        .default_width(150.0)
        .show(&draw_ctx.platform.context(), |ui| {
            draw_pane(right_panes, ctx, ui, draw_ctx);
        });

    egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(true)
        .default_height(150.0)
        .show(&draw_ctx.platform.context(), |ui| {
            draw_pane(bottom_panes, ctx, ui, draw_ctx);
        });

    egui::CentralPanel::default().show(&draw_ctx.platform.context(), |ui| {
        draw_pane(center_panes, ctx, ui, draw_ctx);
    });
}

pub fn draw_pane<'a>(
    panes: Vec<Pane>,
    ctx: &mut UIContext,
    ui: &mut egui::Ui,
    draw_ctx: &mut DrawContext,
) {
    let (mut current_tab, set_current_tab) = use_state(ctx, usize::default());

    ui.horizontal(|ui| {
        panes.iter().enumerate().for_each(|(index, pane)| {
            ui.selectable_value(&mut current_tab, index, &pane.title);
        });
    });
    ui.end_row();
    set_current_tab(current_tab);

    if let Some(current_pane) = panes.get(current_tab) {
        draw_element((current_pane.render)(ctx), ctx, ui, draw_ctx)
    }
}
