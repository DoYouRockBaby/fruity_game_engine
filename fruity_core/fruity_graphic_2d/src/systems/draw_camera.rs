use crate::Camera;
use crate::Graphic2dManager;
use crate::Position;
use crate::Size;
use fruity_core::component::component_rwlock::ComponentRwLock;
use fruity_core::entity::entity_manager::EntityManager;
use fruity_core::entity_type;
use fruity_core::resource::resource_manager::ResourceManager;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::system::system_manager::SystemManager;
use fruity_graphic::math::Matrix4;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use std::sync::Arc;

pub fn draw_camera(
    position: ComponentRwLock,
    size: ComponentRwLock,
    camera: ComponentRwLock,
    graphic_2d_manager: ResourceReference<dyn Graphic2dManager>,
    system_manager: ResourceReference<SystemManager>,
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

    // Start the pass
    {
        let graphic_2d_manager = graphic_2d_manager.read();
        graphic_2d_manager.start_pass(view_proj);
    }

    // Render the draw system pool and avoir the normal system treatment
    {
        let system_manager = system_manager.read();
        system_manager.ignore_pool_once(&98);
        system_manager.run_pool(&98);
    }

    // End the pass
    {
        let graphic_2d_manager = graphic_2d_manager.read();
        graphic_2d_manager.end_pass();
    }
}

pub fn draw_camera_untyped(resource_manager: Arc<ResourceManager>) {
    let service1 = resource_manager.require::<dyn Graphic2dManager>("graphic_2d_manager");
    let service2 = resource_manager.require::<SystemManager>("system_manager");

    let entity_manager = resource_manager.require::<EntityManager>("entity_manager");
    let entity_manager = entity_manager.read();

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

            draw_camera(position, size, camera, service1.clone(), service2.clone());
        });
}
