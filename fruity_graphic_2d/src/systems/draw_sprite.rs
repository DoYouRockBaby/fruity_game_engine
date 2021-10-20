use crate::Graphics2dManager;
use crate::Position;
use crate::Size;
use crate::Sprite;
use fruity_core::entity::entity_manager::EntityManager;
use fruity_core::entity_type;
use fruity_core::service::service_guard::ServiceReadGuard;
use fruity_core::service::service_manager::ServiceManager;
use std::sync::Arc;
use std::sync::RwLock;use std::ops::Deref;

pub fn draw_sprite(
    position: &Position,
    size: &Size,
    sprite: &Sprite,
    graphics_2d_manager: ServiceReadGuard<Graphics2dManager>,
) {
    let material = sprite.material.as_ref().unwrap();

    graphics_2d_manager.draw_square(position.x, position.y, size.width, size.height, material.deref());
}

pub fn draw_sprite_untyped(service_manager: Arc<RwLock<ServiceManager>>) {
    let service_manager = service_manager.read().unwrap();
    let entity_manager = service_manager.read::<EntityManager>();

    entity_manager.for_each(
        entity_type!["Position", "Size", "Sprite"],
        |mut components| {
            let position = match components.next() {
                Some(component) => component,
                None => {
                    log::error!("Tried to launch a system with a component that was not provided");
                    return;
                }
            };

            let position = match position.as_any_ref().downcast_ref::<Position>() {
                Some(component) => component,
                None => {
                    log::error!(
                    "Tried to launch system draw_sprite with component {:?}, expected type Position",
                    position
                );
                    return;
                }
            };

            let size = match components.next() {
                Some(component) => component,
                None => {
                    log::error!("Tried to launch a system with a component that was not provided");
                    return;
                }
            };

            let size = match size.as_any_ref().downcast_ref::<Size>() {
                Some(component) => component,
                None => {
                    log::error!(
                    "Tried to launch system draw_sprite with component {:?}, expected type Size",
                    size
                );
                    return;
                }
            };
    
            let sprite = match components.next() {
                Some(component) => component,
                None => {
                    log::error!("Tried to launch a system with a component that was not provided");
                    return;
                }
            };

            let sprite = match sprite.as_any_ref().downcast_ref::<Sprite>() {
                Some(component) => component,
                None => {
                    log::error!(
                    "Tried to launch system draw_sprite with component {:?}, expected type Sprite",
                    sprite
                );
                    return;
                }
            };

            let service1 = service_manager.read::<Graphics2dManager>();
            draw_sprite(position, size, sprite, service1);
        },
    );
}
