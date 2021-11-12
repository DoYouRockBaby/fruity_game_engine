use crate::hooks::use_global;
use crate::state::world::WorldState;
use crate::ui_element::input::Button;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use fruity_core::system::system_manager::SystemManager;
use std::sync::Arc;

pub fn run_controls_component() -> Vec<UIElement> {
    let world_state = use_global::<WorldState>();

    let resource_manager = world_state.resource_manager.clone();
    let system_manager = resource_manager.require::<SystemManager>("system_manager");
    let system_manager_reader = system_manager.read();

    let system_manager_2 = system_manager.clone();
    let system_manager_3 = system_manager.clone();
    vec![
        if system_manager_reader.is_paused() {
            Button {
                label: "▶".to_string(),
                on_click: Arc::new(move || {
                    let system_manager = system_manager_2.read();

                    system_manager.set_paused(false);
                }),
                ..Default::default()
            }
            .elem()
        } else {
            Button {
                label: "⏸".to_string(),
                on_click: Arc::new(move || {
                    let system_manager = system_manager_3.read();

                    system_manager.set_paused(true);
                }),
                ..Default::default()
            }
            .elem()
        },
        Button {
            label: "◼".to_string(),
            on_click: Arc::new(move || {}),
            enabled: !system_manager_reader.is_paused(),
            ..Default::default()
        }
        .elem(),
    ]
}
