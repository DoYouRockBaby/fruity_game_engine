use crate::dialog_service::DialogService;
use crate::hooks::use_global;
use crate::ui_element::menu::MenuItem;
use crate::ui_element::menu::MenuSection;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use crate::WorldState;
use fruity_core::serialize::yaml::deserialize_yaml;
use fruity_core::serialize::yaml::serialize_yaml;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity::entity_service::EntityServiceSnapshot;
use fruity_ecs::system::system_service::SystemService;
use std::fs::File;
use std::sync::Arc;

pub fn menu_sections_component() -> Vec<UIElement> {
    let world_state = use_global::<WorldState>();

    let resource_container = world_state.resource_container.clone();
    let entity_service = resource_container.require::<EntityService>();
    let entity_service_2 = entity_service.clone();
    let dialog_service = resource_container.require::<dyn DialogService>();
    let dialog_service_2 = dialog_service.clone();
    let dialog_service_3 = dialog_service.clone();
    let system_service = resource_container.require::<SystemService>();

    vec![
        MenuSection {
            label: "File".to_string(),
            items: vec![
                MenuItem {
                    label: "Open".to_string(),
                    on_click: Arc::new(move || {
                        let dialog_service = dialog_service.read();
                        if let Some(filepath) = dialog_service.open(&["*.fs"]) {
                            if let Ok(mut reader) = File::open(&filepath) {
                                if let Some(snapshot) = deserialize_yaml(&mut reader) {
                                    let world_state = use_global::<WorldState>();
                                    let mut entity_service = entity_service_2.write();
                                    let system_service = system_service.read();

                                    entity_service.restore(&EntityServiceSnapshot(snapshot));
                                    system_service.set_paused(true);
                                    world_state.selected_entity = None;
                                } else {
                                }
                            }
                        }
                    }),
                },
                MenuItem {
                    label: "Save".to_string(),
                    on_click: Arc::new(move || {
                        let dialog_service = dialog_service_2.read();
                        dialog_service.save(&["fs"]);
                    }),
                },
                MenuItem {
                    label: "Save as".to_string(),
                    on_click: Arc::new(move || {
                        let dialog_service = dialog_service_3.read();
                        if let Some(filepath) = dialog_service.save(&["fs"]) {
                            if let Ok(mut writer) = File::create(&filepath) {
                                let entity_service = entity_service.read();
                                let snapshot = entity_service.snapshot();

                                if let Ok(_) = serialize_yaml(&mut writer, &snapshot.0) {
                                } else {
                                }
                            }
                        }
                    }),
                },
            ],
        }
        .elem(),
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
        }
        .elem(),
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
        }
        .elem(),
    ]
}
