use crate::components::entity::entity_edit::entity_edit_component;
use crate::components::entity::entity_list::entity_list_component;
use crate::components::file_explorer::file_explorer_component;
use crate::ui_element::pane::Pane;
use crate::ui_element::pane::PaneGrid;
use crate::ui_element::pane::UIPaneSide;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use std::sync::Arc;

pub fn panes_component() -> UIElement {
    PaneGrid {
        panes: vec![
            Pane {
                title: "Entities".to_string(),
                default_side: UIPaneSide::Left,
                render: Arc::new(|| entity_list_component()),
            },
            Pane {
                title: "Edit entity".to_string(),
                default_side: UIPaneSide::Right,
                render: Arc::new(|| entity_edit_component()),
            },
            Pane {
                title: "File explorer".to_string(),
                default_side: UIPaneSide::Bottom,
                render: Arc::new(|| file_explorer_component()),
            },
        ],
    }
    .elem()
}
