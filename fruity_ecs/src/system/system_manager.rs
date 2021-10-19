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
    begin_systems: Vec<Box<System>>,
    end_systems: Vec<Box<System>>,
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
            begin_systems: Vec::new(),
            end_systems: Vec::new(),
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

    /// Add a begin system to the collection
    ///
    /// # Arguments
    /// * `system` - A function that will compute the world
    ///
    pub fn add_begin_system<T: Fn(Arc<RwLock<ServiceManager>>) + Sync + Send + 'static>(
        &mut self,
        system: T,
    ) {
        self.begin_systems.push(Box::new(system))
    }

    /// Add an end system to the collection
    ///
    /// # Arguments
    /// * `system` - A function that will compute the world
    ///
    pub fn add_end_system<T: Fn(Arc<RwLock<ServiceManager>>) + Sync + Send + 'static>(
        &mut self,
        system: T,
    ) {
        self.end_systems.push(Box::new(system))
    }

    /// Run all the stored systems
    pub fn run(&self) {
        self.systems
            .iter()
            .par_bridge()
            .for_each(|system| system(self.service_manager.clone()));
    }

    /// Run all the stored begin systems
    pub fn run_begin(&self) {
        self.begin_systems
            .iter()
            .par_bridge()
            .for_each(|system| system(self.service_manager.clone()));
    }

    /// Run all the stored end systems
    pub fn run_end(&self) {
        self.end_systems
            .iter()
            .par_bridge()
            .for_each(|system| system(self.service_manager.clone()));
    }
}

impl IntrospectMethods<Serialized> for SystemManager {
    fn get_method_infos(&self) -> Vec<MethodInfo<Serialized>> {
        vec![
            MethodInfo {
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
            },
            MethodInfo {
                name: "add_begin_system".to_string(),
                call: MethodCaller::Mut(Arc::new(|this, args| {
                    let this = cast_service_mut::<SystemManager>(this);

                    let mut caster = ArgumentCaster::new("add_begin_system", args);
                    let arg1 = caster.cast_next::<Callback>()?;

                    this.add_begin_system(move |service_manager| {
                        match arg1(service_manager, vec![]) {
                            Ok(_) => (),
                            Err(err) => log_introspect_error(&err),
                        };
                    });

                    Ok(None)
                })),
            },
            MethodInfo {
                name: "add_end_system".to_string(),
                call: MethodCaller::Mut(Arc::new(|this, args| {
                    let this = cast_service_mut::<SystemManager>(this);

                    let mut caster = ArgumentCaster::new("add_end_system", args);
                    let arg1 = caster.cast_next::<Callback>()?;

                    this.add_end_system(move |service_manager| {
                        match arg1(service_manager, vec![]) {
                            Ok(_) => (),
                            Err(err) => log_introspect_error(&err),
                        };
                    });

                    Ok(None)
                })),
            },
        ]
    }
}

impl Service for SystemManager {}
