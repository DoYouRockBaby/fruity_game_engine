use crate::components::entity::entity_list::entity_list_component;
use crate::components::file_explorer::file_explorer_component;
use crate::components::inspector::inspector_component;
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
                title: "Inspector".to_string(),
                default_side: UIPaneSide::Right,
                render: Arc::new(|| inspector_component()),
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
