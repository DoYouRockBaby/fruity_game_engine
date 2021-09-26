use crate::component::component::Component;
use std::sync::RwLock;
use std::sync::RwLockWriteGuard;
use crate::service::service_manager::ServiceManager;
use crate::entity::entity_manager::EntityManager;
use crate::service1::Service1;
use crate::entity::archetype::ArchetypeIdentifier;
use crate::Component1;

macro_rules! archetype {
    ($e:expr) => {{
        let component_names: Vec<&str> = vec![$e];
        ArchetypeIdentifier(component_names
            .iter()
            .map(|e| e.to_string())
            .collect())
    }};
}

pub fn system1(component1: &mut Component1, mut service1: RwLockWriteGuard<Service1>) {
    component1.int1 += 1;

    service1.increment();
    println!("System1 speak: {:#?} {}", component1, service1.value());
}

pub fn system1_untyped(entity_manager: &mut EntityManager, service_manager: &ServiceManager) {
    let service1 = match service_manager.get::<Service1>() {
        Some(service) => service,
        None => {
            log::error!("Service1 service is needed by a system but is not registered");
            return;
        },
    };

    entity_manager.for_each(archetype!["test.component1"], |components: &[&RwLock<dyn Component>]| {
        let mut component1 = match components.get(0) {
            Some(component) => component.write().unwrap(),
            None => {
                log::error!("Tried to launch a system with a component that was not provided, no component with the index {} in the component list {:?}.", 0, components);
                return;
            }
        };

        let component1 = match component1.downcast_mut::<Component1>() {
            Some(component) => {
                component
            },
            None => {
                //log::error!("Tried to launch system system1 with component {:?}, expected type test.component1", component1);
                return;
            },
        };

        let service1 = service1
            .write()
            .unwrap();
        
        system1(component1, service1);
    });
}