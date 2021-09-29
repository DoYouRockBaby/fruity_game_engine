use std::sync::RwLockWriteGuard;
use rayon::prelude::*;
use crate::service::service_manager::ServiceManager;
use crate::entity::entity_manager::EntityManager;
use crate::service1::Service1;
use crate::entity::entity::EntityIdentifier;
use crate::Component1;

macro_rules! archetype {
    ($e:expr) => {{
        let component_names: Vec<&str> = vec![$e];
        EntityIdentifier(component_names
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

pub fn system1_untyped(entity_manager: &EntityManager, service_manager: &ServiceManager) {
    let service1 = match service_manager.get::<Service1>() {
        Some(service) => service,
        None => {
            log::error!("Service1 service is needed by a system but is not registered");
            return;
        },
    };

    entity_manager
        .iter(archetype!["test.component1"])
        .par_bridge()
        .for_each(|entity| {
            entity
                .write()
                .untyped_iter_mut_over_types(archetype!["test.component1"])
                .par_bridge()
                .for_each(|mut components| {
                    let component1 = match components.next() {
                        Some(component) => component,
                        None => {
                            log::error!("Tried to launch a system with a component that was not provided");
                            return;
                        }
                    };
        
                    let component1 = match component1.downcast_mut::<Component1>() {
                        Some(component) => {
                            component
                        },
                        None => {
                            log::error!("Tried to launch system system1 with component {:?}, expected type test.component1", component1);
                            return;
                        },
                    };
        
                    let service1 = service1
                        .write()
                        .unwrap();
                    
                    system1(component1, service1);
                });
            });
}