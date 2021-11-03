use crate::gizmos_service::GizmosService;
use crate::hooks::use_global;
use crate::state::entity::EntityState;
use fruity_core::component::component_rwlock::ComponentRwLock;
use fruity_core::entity::entity::EntityId;
use fruity_core::entity::entity_manager::EntityManager;
use fruity_core::entity_type;
use fruity_core::service::service_guard::ServiceReadGuard;
use fruity_core::service::service_manager::ServiceManager;
use fruity_graphic::math::GREEN;
use fruity_graphic::math::RED;
use fruity_graphic_2d::components::position::Position;
use fruity_graphic_2d::components::size::Size;
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread::sleep;
use std::time::Duration;

pub fn draw_gizmos_2d(
    entity_id: &EntityId,
    position: ComponentRwLock,
    size: ComponentRwLock,
    gizmos_service: ServiceReadGuard<GizmosService>,
) {
    let entity = use_global::<EntityState>();

    if let Some(selected_entity) = &entity.selected_entity {
        let selected_entity_id = {
            let selected_entity = selected_entity.read();
            selected_entity.entity_id
        };

        if selected_entity_id == *entity_id {
            let pos = {
                let position = position.read();
                position
                    .as_any_ref()
                    .downcast_ref::<Position>()
                    .unwrap()
                    .pos
            };

            let size = {
                let size = size.read();
                size.as_any_ref().downcast_ref::<Size>().unwrap().size
            };

            gizmos_service.draw_resize_helper(
                pos,
                pos + size,
                GREEN,
                RED,
                move |drag_action| {
                    let position = position.clone();
                    let drag_origin = {
                        let position = position.read();
                        let position = position
                            .as_any_ref()
                            .downcast_ref::<Position>()
                            .unwrap()
                            .pos;

                        position - drag_action.start_pos()
                    };

                    while drag_action.is_dragging() {
                        sleep(Duration::from_millis(20));

                        let mut position = position.write();
                        let position = position.as_any_mut().downcast_mut::<Position>().unwrap();
                        position.pos = drag_action.get_cursor_position() + drag_origin;
                    }
                },
                move |_drag_action| {},
            );
        }
    }
}

pub fn draw_gizmos_2d_untyped(service_manager: Arc<RwLock<ServiceManager>>) {
    let service_manager = service_manager.read().unwrap();
    let entity_manager = service_manager.read::<EntityManager>();

    entity_manager
        .iter_components(entity_type!["Position", "Size"])
        .par_bridge()
        .for_each(|components| {
            let position = match components.get(0) {
                Some(component) => component,
                None => {
                    log::error!("Tried to launch a system with a component that was not provided");
                    return;
                }
            };

            let size = match components.get(1) {
                Some(component) => component,
                None => {
                    log::error!("Tried to launch a system with a component that was not provided");
                    return;
                }
            };

            let service1 = service_manager.read::<GizmosService>();
            draw_gizmos_2d(&components.get_entity_id(), position, size, service1);
        });
}
