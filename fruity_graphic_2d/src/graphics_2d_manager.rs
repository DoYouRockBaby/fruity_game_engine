use fruity_any::*;
use fruity_ecs::serialize::serialized::Serialized;
use fruity_ecs::service::service::Service;
use fruity_ecs::service::service_rwlock::ServiceRwLock;
use fruity_ecs::world::World;
use fruity_graphic::graphics_manager::GraphicsManager;
use fruity_introspect::IntrospectMethods;
use fruity_introspect::MethodInfo;

#[derive(Debug, FruityAnySyncSend)]
pub struct Graphics2dManager {
    graphics_manager: ServiceRwLock<GraphicsManager>,
}

impl Graphics2dManager {
    pub fn new(world: &World) -> Graphics2dManager {
        let service_manager = world.service_manager.read().unwrap();
        let graphics_manager = service_manager.get::<GraphicsManager>().unwrap();

        Graphics2dManager { graphics_manager }
    }
}

impl IntrospectMethods<Serialized> for Graphics2dManager {
    fn get_method_infos(&self) -> Vec<MethodInfo<Serialized>> {
        vec![]
    }
}

impl Service for Graphics2dManager {}
