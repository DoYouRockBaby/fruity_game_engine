use crate::Camera;
use crate::Graphic2dService;
use crate::Position;
use crate::Size;
use fruity_core::component::component_rwlock::ComponentRwLock;
use fruity_core::entity::entity_service::EntityService;
use fruity_core::entity_type;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::system::system_service::SystemService;
use fruity_graphic::math::Matrix4;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use std::sync::Arc;

pub fn draw_camera(
    position: ComponentRwLock,
    size: ComponentRwLock,
    camera: ComponentRwLock,
    graphic_2d_service: ResourceReference<dyn Graphic2dService>,
    system_service: ResourceReference<SystemService>,
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
        let graphic_2d_service = graphic_2d_service.read();
        graphic_2d_service.start_pass(view_proj);
    }

    // Render the draw system pool and avoir the normal system treatment
    {
        let system_service = system_service.read();
        system_service.ignore_pool_once(&98);
        system_service.run_pool(&98);
    }

    // End the pass
    {
        let graphic_2d_service = graphic_2d_service.read();
        graphic_2d_service.end_pass();
    }
}

pub fn draw_camera_untyped(resource_container: Arc<ResourceContainer>) {
    let resource1 = resource_container.require::<dyn Graphic2dService>("graphic_2d_service");
    let resource2 = resource_container.require::<SystemService>("system_service");

    let entity_service = resource_container.require::<EntityService>("entity_service");
    let entity_service = entity_service.read();

    entity_service
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

            draw_camera(position, size, camera, resource1.clone(), resource2.clone());
        });
}
