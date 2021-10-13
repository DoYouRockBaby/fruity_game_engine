use crate::serialize::serialized::Serialized;
use crate::service::service::Service;
use crate::service::service_manager::ServiceManager;
use crate::service::utils::assert_argument_count;
use crate::service::utils::cast_argument;
use crate::service::utils::cast_service_mut;
use fruity_any_derive::*;
use fruity_introspect::IntrospectMethods;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use rayon::prelude::*;
use std::fmt::Debug;

type System = dyn Fn(&ServiceManager) + Sync + Send + 'static;

/// A systems collection
#[derive(FruityAny)]
pub struct SystemManager {
    systems: Vec<Box<System>>,
}

impl Debug for SystemManager {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'s> SystemManager {
    /// Returns a SystemManager
    pub fn new() -> SystemManager {
        SystemManager {
            systems: Vec::new(),
        }
    }

    /// Add a system to the collection
    ///
    /// # Arguments
    /// * `system` - A function that will compute the world
    ///
    pub fn add_system<T: Fn(&ServiceManager) + Sync + Send + 'static>(&mut self, system: T) {
        self.systems.push(Box::new(system))
    }

    /// Run all the stored systems
    ///
    /// # Arguments
    /// * `entity_manager` - Entities collection
    /// * `service_manager` - Services collection
    ///
    pub fn run(&self, service_manager: &ServiceManager) {
        self.systems
            .iter()
            .par_bridge()
            .for_each(|system| system(service_manager));
    }
}

impl IntrospectMethods<Serialized> for SystemManager {
    fn get_method_infos(&self) -> Vec<MethodInfo<Serialized>> {
        vec![MethodInfo {
            name: "add_system".to_string(),
            args: vec!["fn".to_string()],
            return_type: None,
            call: MethodCaller::Mut(|this, args| {
                let this = cast_service_mut::<SystemManager>(this);
                assert_argument_count(1, &args)?;

                let arg1 = cast_argument(0, &args, |arg| arg.as_callback())?;

                this.add_system(move |service_manager: &ServiceManager| {
                    arg1(service_manager, vec![]);
                });
                Ok(None)
            }),
        }]
    }
}

impl Service for SystemManager {}
