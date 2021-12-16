use crate::ui_element::pane::Pane;
use crate::ui_element::pane::PaneGrid;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use crate::use_global;
use crate::EditorPanelsService;
use crate::WorldState;

pub fn panes_component() -> UIElement {
    let world_state = use_global::<WorldState>();
    let editor_panels_service = world_state
        .resource_container
        .require::<EditorPanelsService>();
    let editor_panels_service = editor_panels_service.read();

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
