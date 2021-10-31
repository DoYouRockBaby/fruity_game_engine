use crate::components::camera::Camera;
use crate::components::position::Position;
use crate::components::size::Size;
use crate::components::sprite::Sprite;
use crate::graphics_2d_manager::Graphics2dManager;
use crate::systems::draw_camera::draw_camera_untyped;
use crate::systems::draw_sprite::draw_sprite_untyped;
use fruity_core::component::component::AnyComponent;
use fruity_core::component::components_factory::ComponentsFactory;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::system::system_manager::SystemManager;
use fruity_core::world::World;

pub mod components;
pub mod graphics_2d_manager;
pub mod systems;

#[no_mangle]
pub fn initialize(world: &World) {
    let graphic_2d_manager = Graphics2dManager::new(world);

    let mut service_manager = world.service_manager.write().unwrap();
    service_manager.register("graphic_2d_manager", graphic_2d_manager);

    let mut components_factory = service_manager.write::<ComponentsFactory>();

    components_factory.add("Position", || {
        AnyComponent::new(Position { x: 0.0, y: 0.0 })
    });
    components_factory.add("Size", || {
        AnyComponent::new(Size {
            width: 0.0,
            height: 0.0,
        })
    });
    components_factory.add("Sprite", || {
        AnyComponent::new(Sprite {
            material: ResourceReference::new(),
        })
    });
    components_factory.add("Camera", || {
        AnyComponent::new(Camera {
            near: -1.0,
            far: 1.0,
        })
    });

    let mut system_manager = service_manager.write::<SystemManager>();
    system_manager.add_system(draw_camera_untyped, Some(97));
    system_manager.add_system(draw_sprite_untyped, Some(98));
}
