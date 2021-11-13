use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::resource::resource_container::ResourceContainer;
use crate::Graphic2dService;
use crate::Position;
use crate::Size;
use crate::Sprite;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity_type;
use std::sync::Arc;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;

pub fn draw_sprite(
    position: &Position,
    size: &Size,
    sprite: &Sprite,
    graphic_2d_service: ResourceReference<dyn Graphic2dService>,
) {
    if let Some(material) = &sprite.material.0 {
        let graphic_2d_service = graphic_2d_service.read();
        graphic_2d_service.draw_square(position.pos, size.size, sprite.z_index, material.clone());
    }
}

pub fn draw_sprite_untyped(resource_container: Arc<ResourceContainer>) {
    let resource1 = resource_container
        .require::<dyn Graphic2dService>("graphic_2d_service");

    let entity_service = resource_container
        .require::<EntityService>("entity_service");

    let entity_service = entity_service.read();
    entity_service.iter_components(
        entity_type!["Position", "Size", "Sprite"],
    )
    .par_bridge()
    .for_each(| components| {
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
                    "Tried to launch system draw_sprite with component {:?}, expected type Position",
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
                    "Tried to launch system draw_sprite with component {:?}, expected type Size",
                    size
                );
                    return;
                }
            };
    
            let sprite = match components.get(2) {
                Some(component) => component,
                None => {
                    log::error!("Tried to launch a system with a component that was not provided");
                    return;
                }
            };

            let sprite = sprite.read();
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

            draw_sprite(position, size, sprite, resource1.clone());
        },
    );
}
