use crate::serialize::serialized::Callback;
use crate::serialize::serialized::Serialized;
use crate::service::service::Service;
use crate::service::service_manager::ServiceManager;
use crate::service::utils::cast_service_mut;
use crate::service::utils::ArgumentCaster;
use crate::World;
use fruity_any::*;
use fruity_introspect::log_introspect_error;
use fruity_introspect::IntrospectMethods;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use rayon::prelude::*;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

type System = dyn Fn(Arc<RwLock<ServiceManager>>) + Sync + Send + 'static;

/// A systems collection
#[derive(FruityAnySyncSend)]
pub struct SystemManager {
    systems: Vec<Box<System>>,
    service_manager: Arc<RwLock<ServiceManager>>,
}

impl Debug for SystemManager {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'s> SystemManager {
    /// Returns a SystemManager
    pub fn new(world: &World) -> SystemManager {
        SystemManager {
            systems: Vec::new(),
            service_manager: world.service_manager.clone(),
        }
    }

    /// Add a system to the collection
    ///
    /// # Arguments
    /// * `system` - A function that will compute the world
    ///
    pub fn add_system<T: Fn(Arc<RwLock<ServiceManager>>) + Sync + Send + 'static>(
        &mut self,
        system: T,
    ) {
        self.systems.push(Box::new(system))
    }

    /// Run all the stored systems
    ///
    /// # Arguments
    /// * `entity_manager` - Entities collection
    /// * `service_manager` - Services collection
    ///
    pub fn run(&self) {
        self.systems
            .iter()
            .par_bridge()
            .for_each(|system| system(self.service_manager.clone()));
    }
}

impl IntrospectMethods<Serialized> for SystemManager {
    fn get_method_infos(&self) -> Vec<MethodInfo<Serialized>> {
        vec![MethodInfo {
            name: "add_system".to_string(),
            call: MethodCaller::Mut(Arc::new(|this, args| {
                let this = cast_service_mut::<SystemManager>(this);

                let mut caster = ArgumentCaster::new("add_system", args);
                let arg1 = caster.cast_next::<Callback>()?;

                this.add_system(move |service_manager| {
                    match arg1(service_manager, vec![]) {
                        Ok(_) => (),
                        Err(err) => log_introspect_error(&err),
                    };
                });

                Ok(None)
            })),
        }]
    }
}

impl Service for SystemManager {}
