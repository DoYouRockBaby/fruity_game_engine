use crate::ui::context::UIContext;
use crate::ui::elements::pane::Pane;
use crate::ui::elements::pane::PaneGrid;
use crate::ui::elements::UIElement;
use crate::ui::elements::UIWidget;
use crate::ui::hooks::use_read_service;
use crate::EditorPanelsService;

pub fn panes_component(ctx: &mut UIContext) -> UIElement {
    let editor_panels_service = use_read_service::<EditorPanelsService>(ctx);

    PaneGrid {
        panes: editor_panels_service
            .iter_panels()
            .map(|panel| Pane {
                title: panel.label.clone(),
                default_side: panel.default_side,
                render: panel.renderer.clone(),
            })
            .collect::<Vec<_>>(),
    }
    .elem()
}
