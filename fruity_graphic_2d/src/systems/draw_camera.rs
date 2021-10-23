use fruity_graphic::math::Matrix4;
use fruity_core::system::system_manager::SystemManager;
use crate::Graphics2dManager;
use crate::Position;
use crate::Size;
use crate::Camera;
use fruity_core::entity::entity_manager::EntityManager;
use fruity_core::entity_type;
use fruity_core::service::service_guard::ServiceReadGuard;
use fruity_core::service::service_manager::ServiceManager;
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::RwLock;

pub fn draw_camera(
    position: &Position,
    size: &Size,
    camera: &Camera,
     graphics_2d_manager: ServiceReadGuard<Graphics2dManager>,
    system_manager: ServiceReadGuard<SystemManager>,
) {
    let view_proj = Matrix4::from_rect(position.x, position.x + size.width, position.y, position.y + size.height, camera.near, camera.far);
    graphics_2d_manager.start_rendering(view_proj);
    std::mem::drop(graphics_2d_manager);

    // Render the draw system pool and avoir the normal system treatment
    system_manager.ignore_pool_once(&98);
    system_manager.run_pool(&98);
}

pub fn draw_camera_untyped(service_manager: Arc<RwLock<ServiceManager>>) {
    let service_manager = service_manager.read().unwrap();
    let entity_manager = service_manager.read::<EntityManager>();

    //println!("{:#?}", entity_manager.deref());

    entity_manager.iter_components(
        entity_type!["Position", "Size", "Camera"],
    ).par_bridge().for_each(
        | components| {
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
                    "Tried to launch system draw_camera with component {:?}, expected type Position",
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
                    "Tried to launch system draw_camera with component {:?}, expected type Size",
                    size
                );
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

            let camera = camera.read();
            let camera = match camera.as_any_ref().downcast_ref::<Camera>() {
                Some(component) => component,
                None => {
                    log::error!(
                    "Tried to launch system draw_camera with component {:?}, expected type Camera",
                    camera
                );
                    return;
                }
            };

            let service1 = service_manager.read::<Graphics2dManager>();
            let service2 = service_manager.read::<SystemManager>();
            draw_camera(position, size, camera, service1, service2);
        },
    );
}
