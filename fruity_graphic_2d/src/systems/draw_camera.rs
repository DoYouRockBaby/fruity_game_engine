use crate::Camera;
use crate::Graphics2dManager;
use crate::Position;
use crate::Size;
use fruity_core::component::component_rwlock::ComponentRwLock;
use fruity_core::entity::entity_manager::EntityManager;
use fruity_core::entity_type;
use fruity_core::service::service_guard::ServiceReadGuard;
use fruity_core::service::service_manager::ServiceManager;
use fruity_core::system::system_manager::SystemManager;
use fruity_graphic::math::Matrix4;
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::RwLock;

pub fn draw_camera(
    position: ComponentRwLock,
    size: ComponentRwLock,
    camera: ComponentRwLock,
    graphics_2d_manager: ServiceReadGuard<Graphics2dManager>,
    system_manager: ServiceReadGuard<SystemManager>,
) {
    let view_proj = {
        let position = position.read();
        let position = position.as_any_ref().downcast_ref::<Position>().unwrap();

        let size = size.read();
        let size = size.as_any_ref().downcast_ref::<Size>().unwrap();

        let camera = camera.read();
        let camera = camera.as_any_ref().downcast_ref::<Camera>().unwrap();

        Matrix4::from_rect(
            position.pos.x,
            position.pos.x + size.size.x,
            position.pos.y,
            position.pos.y + size.size.y,
            camera.near,
            camera.far,
        )
    };

    graphics_2d_manager.start_rendering(view_proj);
    std::mem::drop(graphics_2d_manager);

    // Render the draw system pool and avoir the normal system treatment
    system_manager.ignore_pool_once(&98);
    system_manager.run_pool(&98);
}

pub fn draw_camera_untyped(service_manager: Arc<RwLock<ServiceManager>>) {
    let service_manager = service_manager.read().unwrap();
    let entity_manager = service_manager.read::<EntityManager>();

    entity_manager
        .iter_components(entity_type!["Position", "Size", "Camera"])
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
            let camera = match components.get(2) {
                Some(component) => component,
                None => {
                    log::error!("Tried to launch a system with a component that was not provided");
                    return;
                }
            };

            let service1 = service_manager.read::<Graphics2dManager>();
            let service2 = service_manager.read::<SystemManager>();
            draw_camera(position, size, camera, service1, service2);
        });
}
