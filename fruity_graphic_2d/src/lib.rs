use crate::components::position::Position;
use crate::components::size::Size;
use crate::graphics_2d_manager::Graphics2dManager;
use fruity_ecs::component::components_factory::ComponentsFactory;
use fruity_ecs::world::World;

pub mod components;
pub mod graphics_2d_manager;

/// Initialize this extension
pub fn initialize(world: &World) {
    let graphic_2d_manager = Graphics2dManager::new(world);

    let mut service_manager = world.service_manager.write().unwrap();
    service_manager.register("graphic_2d_manager", graphic_2d_manager);

    let components_factory = service_manager.get::<ComponentsFactory>().unwrap();
    let mut components_factory = components_factory.write().unwrap();

    components_factory.add("Position", || Box::new(Position { x: 0.0, y: 0.0 }));
    components_factory.add("Size", || {
        Box::new(Size {
            width: 0.0,
            height: 0.0,
        })
    });
}