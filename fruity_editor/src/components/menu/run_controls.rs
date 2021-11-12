use crate::hooks::use_global;
use crate::state::world::WorldState;
use crate::ui_element::input::Button;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use fruity_core::system::system_service::SystemService;
use std::sync::Arc;

pub fn run_controls_component() -> Vec<UIElement> {
    let world_state = use_global::<WorldState>();

    let resource_container = world_state.resource_container.clone();
    let system_service = resource_container.require::<SystemService>("system_service");
    let system_service_reader = system_service.read();

    let system_service_2 = system_service.clone();
    let system_service_3 = system_service.clone();
    vec![
        if system_service_reader.is_paused() {
            Button {
                label: "▶".to_string(),
                on_click: Arc::new(move || {
                    let system_service = system_service_2.read();

                    system_service.set_paused(false);
                }),
                ..Default::default()
            }
            .elem()
        } else {
            Button {
                label: "⏸".to_string(),
                on_click: Arc::new(move || {
                    let system_service = system_service_3.read();

                    system_service.set_paused(true);
                }),
                ..Default::default()
            }
            .elem()
        },
        Button {
            label: "◼".to_string(),
            on_click: Arc::new(move || {}),
            enabled: !system_service_reader.is_paused(),
            ..Default::default()
        }
        .elem(),
    ]
}
