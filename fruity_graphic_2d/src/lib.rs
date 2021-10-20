use crate::components::camera::Camera;
use crate::components::position::Position;
use crate::components::size::Size;
use crate::components::sprite::Sprite;
use crate::graphics_2d_manager::Graphics2dManager;
use crate::systems::draw_sprite::draw_sprite_untyped;
use fruity_ecs::component::components_factory::ComponentsFactory;
use fruity_ecs::serialize::serialized::ResourceReference;
use fruity_ecs::system::system_manager::SystemManager;
use fruity_ecs::world::World;

pub mod components;
pub mod graphics_2d_manager;
pub mod systems;

/// Initialize this extension
pub fn initialize(world: &World) {
    let graphic_2d_manager = Graphics2dManager::new(world);

    let mut service_manager = world.service_manager.write().unwrap();
    service_manager.register("graphic_2d_manager", graphic_2d_manager);

    let mut components_factory = service_manager.write::<ComponentsFactory>();

    components_factory.add("Position", || Box::new(Position { x: 0.0, y: 0.0 }));
    components_factory.add("Size", || {
        Box::new(Size {
            width: 0.0,
            height: 0.0,
        })
    });
    components_factory.add("Sprite", || {
        Box::new(Sprite {
            material: ResourceReference::new(),
        })
    });
    components_factory.add("Camera", || {
        Box::new(Camera {
            near: 0.1,
            far: 100.0,
        })
    });

    let mut system_manager = service_manager.write::<SystemManager>();
    system_manager.add_system(draw_sprite_untyped, None);
}
