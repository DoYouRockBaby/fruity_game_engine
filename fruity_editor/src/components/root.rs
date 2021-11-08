use crate::components::panes::panes_component;
use crate::ui_element::menu::MenuBar;
use crate::ui_element::menu::MenuItem;
use crate::ui_element::menu::MenuSection;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use std::sync::Arc;

pub fn root_component() -> Vec<UIElement> {
    vec![
        MenuBar {
            sections: vec![
                MenuSection {
                    label: "File".to_string(),
                    items: vec![
                        MenuItem {
                            label: "Open".to_string(),
                            on_click: Arc::new(|| ()),
                        },
                        MenuItem {
                            label: "Save".to_string(),
                            on_click: Arc::new(|| ()),
                        },
                        MenuItem {
                            label: "Save as".to_string(),
                            on_click: Arc::new(|| ()),
                        },
                    ],
                },
                MenuSection {
                    label: "Project".to_string(),
                    items: vec![
                        MenuItem {
                            label: "Settings".to_string(),
                            on_click: Arc::new(|| ()),
                        },
                        MenuItem {
                            label: "Platforms".to_string(),
                            on_click: Arc::new(|| ()),
                        },
                        MenuItem {
                            label: "Inputs".to_string(),
                            on_click: Arc::new(|| ()),
                        },
                    ],
                },
                MenuSection {
                    label: "Tools".to_string(),
                    items: vec![
                        MenuItem {
                            label: "Grid".to_string(),
                            on_click: Arc::new(|| ()),
                        },
                        MenuItem {
                            label: "Appearance".to_string(),
                            on_click: Arc::new(|| ()),
                        },
                    ],
                },
            ],
        }
        .elem(),
        panes_component(),
    ]
}
