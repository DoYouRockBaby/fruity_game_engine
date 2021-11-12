use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::resource::resource_manager::ResourceManager;
use crate::Graphic2dManager;
use crate::Position;
use crate::Size;
use crate::Sprite;
use fruity_core::entity::entity_manager::EntityManager;
use fruity_core::entity_type;
use std::sync::Arc;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;

pub fn draw_sprite(
    position: &Position,
    size: &Size,
    sprite: &Sprite,
    graphic_2d_manager: ResourceReference<dyn Graphic2dManager>,
) {
    if let Some(material) = &sprite.material.0 {
        let graphic_2d_manager = graphic_2d_manager.read();
        graphic_2d_manager.draw_square(position.pos, size.size, sprite.z_index, material.clone());
    }
}

pub fn draw_sprite_untyped(resource_manager: Arc<ResourceManager>) {
    let resource1 = resource_manager
        .require::<dyn Graphic2dManager>("graphic_2d_manager");

    let entity_manager = resource_manager
        .require::<EntityManager>("entity_manager");

    let entity_manager = entity_manager.read();
    entity_manager.iter_components(
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
