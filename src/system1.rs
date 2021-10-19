use crate::Component1;
use crate::Service1;
use fruity_ecs::entity::entity_manager::EntityManager;
use fruity_ecs::entity_type;
use fruity_ecs::service::service_guard::ServiceWriteGuard;
use fruity_ecs::service::service_manager::ServiceManager;
use std::sync::Arc;
use std::sync::RwLock;

pub fn system1(component1: &mut Component1, mut service1: ServiceWriteGuard<Service1>) {
    component1.int1 += 1;

    service1.increment();
    println!("System1 speak: {:#?} {}", component1, service1.value());
}

pub fn system1_untyped(service_manager: Arc<RwLock<ServiceManager>>) {
    let service_manager = service_manager.read().unwrap();
    let entity_manager = service_manager.read::<EntityManager>();

    entity_manager.for_each_mut(
        entity_type!["Component1", "Component2"],
        |mut components| {
            let component1 = match components.next() {
                Some(component) => component,
                None => {
                    log::error!("Tried to launch a system with a component that was not provided");
                    return;
                }
            };

            let component1 = match component1.as_any_mut().downcast_mut::<Component1>() {
                Some(component) => component,
                None => {
                    log::error!(
                    "Tried to launch system system1 with component {:?}, expected type Component1",
                    component1
                );
                    return;
                }
            };

            let service1 = service_manager.write::<Service1>();
            system1(component1, service1);
        },
    );
}
