use crate::hooks::use_global;
use crate::state::world::WorldState;
use crate::ui_element::input::Button;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::system::system_service::SystemService;
use std::sync::Arc;

pub fn run_controls_component() -> Vec<UIElement> {
    let world_state = use_global::<WorldState>();

    let resource_container = world_state.resource_container.clone();
    let entity_service = resource_container.require::<EntityService>();
    let system_service = resource_container.require::<SystemService>();
    let system_service_reader = system_service.read();

    let entity_service_2 = entity_service.clone();
    let system_service_2 = system_service.clone();
    let system_service_3 = system_service.clone();
    let system_service_4 = system_service.clone();
    vec![
        if system_service_reader.is_paused() {
            Button {
                label: "▶".to_string(),
                on_click: Arc::new(move || {
                    let world_state = use_global::<WorldState>();

                    let system_service = system_service_2.read();
                    let entity_service = entity_service.read();

                    world_state.snapshot = Some(entity_service.snapshot());
                    world_state.selected_entity = None;
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
            on_click: Arc::new(move || {
                let world_state = use_global::<WorldState>();

                let system_service = system_service_4.read();
                let mut entity_service = entity_service_2.write();

                entity_service.restore(world_state.snapshot.as_ref().unwrap());
                world_state.snapshot = None;
                world_state.selected_entity = None;
                system_service.set_paused(true);
            }),
            enabled: world_state.snapshot.is_some(),
            ..Default::default()
        }
        .elem(),
    ]
}
