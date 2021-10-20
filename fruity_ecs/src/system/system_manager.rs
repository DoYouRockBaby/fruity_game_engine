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
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

type System = dyn Fn(Arc<RwLock<ServiceManager>>) + Sync + Send + 'static;

/// A systems collection
///
/// There is three type of systems:
/// - begin_systems are called just before the rendering but after the resources allocations, it's perfect for initiliazing your entities
/// - end systems is called before closing the software
/// - systems are called every frame
///
/// There is a pool system, when you add a system, you can provide a pool, every systems of the same pool will be executed in parallel
/// Try to use it realy rarely, cause parallel execution is realy usefull
/// Pools from 0 to 10 and from 100 to 110 are reservec by the engine, you should avoid to create pool outside this range
///
#[derive(FruityAnySyncSend)]
pub struct SystemManager {
    system_pools: HashMap<usize, Vec<Box<System>>>,
    begin_system_pools: HashMap<usize, Vec<Box<System>>>,
    end_system_pools: HashMap<usize, Vec<Box<System>>>,
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
            system_pools: HashMap::new(),
            begin_system_pools: HashMap::new(),
            end_system_pools: HashMap::new(),
            service_manager: world.service_manager.clone(),
        }
    }

    /// Add a system to the collection
    ///
    /// # Arguments
    /// * `system` - A function that will compute the world
    /// * `pool_index` - A pool identifier, all the systems of the same pool will be processed together in parallel
    ///
    pub fn add_system<T: Fn(Arc<RwLock<ServiceManager>>) + Sync + Send + 'static>(
        &mut self,
        system: T,
        pool_index: Option<usize>,
    ) {
        let pool_index = pool_index.unwrap_or(50);

        if let Some(pool) = self.system_pools.get_mut(&pool_index) {
            pool.push(Box::new(system))
        } else {
            // If the pool not exists, we create it
            let pool = vec![Box::new(system) as Box<System>];
            self.system_pools.insert(pool_index, pool);
        };
    }

    /// Add a begin system to the collection
    ///
    /// # Arguments
    /// * `system` - A function that will compute the world
    /// * `pool_index` - A pool identifier, all the systems of the same pool will be processed together in parallel
    ///
    pub fn add_begin_system<T: Fn(Arc<RwLock<ServiceManager>>) + Sync + Send + 'static>(
        &mut self,
        system: T,
        pool_index: Option<usize>,
    ) {
        let pool_index = pool_index.unwrap_or(50);

        if let Some(pool) = self.begin_system_pools.get_mut(&pool_index) {
            pool.push(Box::new(system))
        } else {
            // If the pool not exists, we create it
            let pool = vec![Box::new(system) as Box<System>];
            self.begin_system_pools.insert(pool_index, pool);
        };
    }

    /// Add an end system to the collection
    ///
    /// # Arguments
    /// * `system` - A function that will compute the world
    /// * `pool_index` - A pool identifier, all the systems of the same pool will be processed together in parallel
    ///
    pub fn add_end_system<T: Fn(Arc<RwLock<ServiceManager>>) + Sync + Send + 'static>(
        &mut self,
        system: T,
        pool_index: Option<usize>,
    ) {
        let pool_index = pool_index.unwrap_or(50);

        if let Some(pool) = self.end_system_pools.get_mut(&pool_index) {
            pool.push(Box::new(system))
        } else {
            // If the pool not exists, we create it
            let pool = vec![Box::new(system) as Box<System>];
            self.end_system_pools.insert(pool_index, pool);
        };
    }

    /// Iter over all the systems pools
    fn iter_system_pools(&self) -> impl Iterator<Item = &Vec<Box<System>>> {
        self.system_pools.iter().map(|pool| pool.1)
    }

    /// Iter over all the begin systems pools
    fn iter_begin_system_pools(&self) -> impl Iterator<Item = &Vec<Box<System>>> {
        self.begin_system_pools.iter().map(|pool| pool.1)
    }

    /// Iter over all the end systems pools
    fn iter_end_system_pools(&self) -> impl Iterator<Item = &Vec<Box<System>>> {
        self.end_system_pools.iter().map(|pool| pool.1)
    }

    /// Run all the stored systems
    pub fn run(&self) {
        self.iter_system_pools().for_each(|pool| {
            pool.iter()
                .par_bridge()
                .for_each(|system| system(self.service_manager.clone()))
        });
    }

    /// Run all the stored begin systems
    pub fn run_begin(&self) {
        self.iter_begin_system_pools().for_each(|pool| {
            pool.iter()
                .par_bridge()
                .for_each(|system| system(self.service_manager.clone()))
        });
    }

    /// Run all the stored end systems
    pub fn run_end(&self) {
        self.iter_end_system_pools().for_each(|pool| {
            pool.iter()
                .par_bridge()
                .for_each(|system| system(self.service_manager.clone()))
        });
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
                    let arg2 = caster.cast_next_optional::<usize>();

                    this.add_system(
                        move |service_manager| {
                            match arg1(service_manager, vec![]) {
                                Ok(_) => (),
                                Err(err) => log_introspect_error(&err),
                            };
                        },
                        arg2,
                    );

                    Ok(None)
                })),
            },
            MethodInfo {
                name: "add_begin_system".to_string(),
                call: MethodCaller::Mut(Arc::new(|this, args| {
                    let this = cast_service_mut::<SystemManager>(this);

                    let mut caster = ArgumentCaster::new("add_begin_system", args);
                    let arg1 = caster.cast_next::<Callback>()?;
                    let arg2 = caster.cast_next_optional::<usize>();

                    this.add_begin_system(
                        move |service_manager| {
                            match arg1(service_manager, vec![]) {
                                Ok(_) => (),
                                Err(err) => log_introspect_error(&err),
                            };
                        },
                        arg2,
                    );

                    Ok(None)
                })),
            },
            MethodInfo {
                name: "add_end_system".to_string(),
                call: MethodCaller::Mut(Arc::new(|this, args| {
                    let this = cast_service_mut::<SystemManager>(this);

                    let mut caster = ArgumentCaster::new("add_end_system", args);
                    let arg1 = caster.cast_next::<Callback>()?;
                    let arg2 = caster.cast_next_optional::<usize>();

                    this.add_end_system(
                        move |service_manager| {
                            match arg1(service_manager, vec![]) {
                                Ok(_) => (),
                                Err(err) => log_introspect_error(&err),
                            };
                        },
                        arg2,
                    );

                    Ok(None)
                })),
            },
        ]
    }
}

impl Service for SystemManager {}
