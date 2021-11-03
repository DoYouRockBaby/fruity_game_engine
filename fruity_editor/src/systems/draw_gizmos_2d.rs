use crate::gizmos_service::GizmosService;
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

pub fn draw_gizmos_2d(
    position: &Position,
    size: &Size,
    gizmos_service: ServiceReadGuard<GizmosService>,
) {
    gizmos_service.draw_square_helper(position.pos, position.pos + size.size, GREEN, RED);
}

pub fn draw_gizmos_2d_untyped(service_manager: Arc<RwLock<ServiceManager>>) {
    let service_manager = service_manager.read().unwrap();
    let entity_manager = service_manager.read::<EntityManager>();

    entity_manager.iter_components(
        entity_type!["Position", "Size"],
    ).par_bridge().for_each(| components| {
            let position = match components.get(0) {
                Some(component) => component,
                None => {
                    log::error!("Tried to launch a system with a component that was not provided");
                    return;
                }
            };

            let position = position.read();
            let position = match position.as_any_ref().downcast_ref::<Position>() {
                Some(component) => component,
                None => {
                    log::error!(
                    "Tried to launch system draw_gizmos_2d with component {:?}, expected type Position",
                    position
                );
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

            let size = size.read();
            let size = match size.as_any_ref().downcast_ref::<Size>() {
                Some(component) => component,
                None => {
                    log::error!(
                    "Tried to launch system draw_gizmos_2d with component {:?}, expected type Size",
                    size
                );
                    return;
                }
            };

            let service1 = service_manager.read::<GizmosService>();
            draw_gizmos_2d(position, size, service1);
        },
    );
}
