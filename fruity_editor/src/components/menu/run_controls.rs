use crate::hooks::use_global;
use crate::state::world::WorldState;
use crate::ui_element::input::Button;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use fruity_core::system::system_manager::SystemManager;
use std::sync::Arc;

pub fn run_controls_component() -> Vec<UIElement> {
    let world_state = use_global::<WorldState>();

    let service_manager = world_state.service_manager.clone();
    let service_manager_reader = world_state.service_manager.read().unwrap();
    let system_manager = service_manager_reader.read::<SystemManager>();

    vec![
        if system_manager.is_paused() {
            Button {
                label: "▶".to_string(),
                on_click: Arc::new(move || {
                    let service_manager = service_manager.read().unwrap();
                    let system_manager_reader = service_manager.read::<SystemManager>();
                    system_manager_reader.set_paused(false);
                }),
                ..Default::default()
            }
            .elem()
        } else {
            Button {
                label: "⏸".to_string(),
                on_click: Arc::new(move || {
                    let service_manager = service_manager.read().unwrap();
                    let system_manager_reader = service_manager.read::<SystemManager>();
                    system_manager_reader.set_paused(true);
                }),
                ..Default::default()
            }
            .elem()
        },
        Button {
            label: "◼".to_string(),
            on_click: Arc::new(move || {}),
            enabled: !system_manager.is_paused(),
            ..Default::default()
        }
        .elem(),
    ]
}
