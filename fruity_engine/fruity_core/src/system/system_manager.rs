use crate::service::service::Service;
use crate::service::service_manager::ServiceManager;
use crate::service::utils::cast_service;
use crate::service::utils::cast_service_mut;
use crate::service::utils::ArgumentCaster;
use fruity_any::*;
use fruity_introspect::log_introspect_error;
use fruity_introspect::serialized::Callback;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::RwLock;

type SystemCallback = dyn Fn(Arc<RwLock<ServiceManager>>) + Sync + Send + 'static;

struct BeginSystem {
    callback: Box<SystemCallback>,
}

struct EndSystem {
    callback: Box<SystemCallback>,
}

struct FrameSystem {
    callback: Box<SystemCallback>,
    ignore_pause: bool,
}

/// A system pool, see [‘SystemManager‘] for more informations
pub struct SystemPool<T> {
    /// Is the pool ignored, if it's not, it will not be launched when calling [‘SystemManager‘]::run
    ignore_once: RwLock<bool>,

    /// Systems of the pool
    systems: Vec<T>,
}

/// A systems collection
///
/// There is three type of systems:
/// - begin_systems are called just before the rendering but after the resources allocations, it's perfect for initiliazing your entities
/// - end systems is called before closing the software
/// - systems are called every frame
///
/// There is a pool system, when you add a system, you can provide a pool, every systems of the same pool will be executed in parallel
/// Try to use it realy rarely, cause parallel execution is realy usefull
/// Pools from 0 to 10 and from 90 to 100 are reservec by the engine, you should avoid to create pool outside this range
/// Pool 97 is for camera
/// Pool 98 is for drawing
///
#[derive(FruityAny)]
pub struct SystemManager {
    pause: AtomicBool,
    system_pools: BTreeMap<usize, SystemPool<FrameSystem>>,
    begin_system_pools: BTreeMap<usize, SystemPool<BeginSystem>>,
    end_system_pools: BTreeMap<usize, SystemPool<EndSystem>>,
    service_manager: Arc<RwLock<ServiceManager>>,
}

impl Debug for SystemManager {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'s> SystemManager {
    /// Returns a SystemManager
    pub fn new(service_manager: &Arc<RwLock<ServiceManager>>) -> SystemManager {
        SystemManager {
            pause: AtomicBool::new(false),
            system_pools: BTreeMap::new(),
            begin_system_pools: BTreeMap::new(),
            end_system_pools: BTreeMap::new(),
            service_manager: service_manager.clone(),
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
        callback: T,
        pool_index: Option<usize>,
    ) {
        let pool_index = pool_index.unwrap_or(50);

        let system = FrameSystem {
            callback: Box::new(callback),
            ignore_pause: false,
        };

        if let Some(pool) = self.system_pools.get_mut(&pool_index) {
            pool.systems.push(system)
        } else {
            // If the pool not exists, we create it
            let systems = vec![system];
            self.system_pools.insert(
                pool_index,
                SystemPool {
                    ignore_once: RwLock::new(false),
                    systems,
                },
            );
        };
    }

    /// Add a system to the collection
    ///
    /// # Arguments
    /// * `system` - A function that will compute the world
    /// * `pool_index` - A pool identifier, all the systems of the same pool will be processed together in parallel
    ///
    pub fn add_system_that_ignore_pause<
        T: Fn(Arc<RwLock<ServiceManager>>) + Sync + Send + 'static,
    >(
        &mut self,
        callback: T,
        pool_index: Option<usize>,
    ) {
        let pool_index = pool_index.unwrap_or(50);

        let system = FrameSystem {
            callback: Box::new(callback),
            ignore_pause: true,
        };

        if let Some(pool) = self.system_pools.get_mut(&pool_index) {
            pool.systems.push(system)
        } else {
            // If the pool not exists, we create it
            let systems = vec![system];
            self.system_pools.insert(
                pool_index,
                SystemPool {
                    ignore_once: RwLock::new(false),
                    systems,
                },
            );
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
        callback: T,
        pool_index: Option<usize>,
    ) {
        let pool_index = pool_index.unwrap_or(50);

        let system = BeginSystem {
            callback: Box::new(callback),
        };

        if let Some(pool) = self.begin_system_pools.get_mut(&pool_index) {
            pool.systems.push(system)
        } else {
            // If the pool not exists, we create it
            let systems = vec![system];
            self.begin_system_pools.insert(
                pool_index,
                SystemPool {
                    ignore_once: RwLock::new(false),
                    systems,
                },
            );
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
        callback: T,
        pool_index: Option<usize>,
    ) {
        let pool_index = pool_index.unwrap_or(50);

        let system = EndSystem {
            callback: Box::new(callback),
        };

        if let Some(pool) = self.end_system_pools.get_mut(&pool_index) {
            pool.systems.push(system)
        } else {
            // If the pool not exists, we create it
            let systems = vec![system];
            self.end_system_pools.insert(
                pool_index,
                SystemPool {
                    ignore_once: RwLock::new(false),
                    systems,
                },
            );
        };
    }

    /// Iter over all the systems pools
    fn iter_system_pools(&self) -> impl Iterator<Item = &SystemPool<FrameSystem>> {
        self.system_pools.iter().map(|pool| pool.1)
    }

    /// Iter over all the begin systems pools
    fn iter_begin_system_pools(&self) -> impl Iterator<Item = &SystemPool<BeginSystem>> {
        self.begin_system_pools.iter().map(|pool| pool.1)
    }

    /// Iter over all the end systems pools
    fn iter_end_system_pools(&self) -> impl Iterator<Item = &SystemPool<EndSystem>> {
        self.end_system_pools.iter().map(|pool| pool.1)
    }

    /// Run all the stored systems
    pub fn run(&self) {
        let service_manager = self.service_manager.clone();
        let is_paused = self.is_paused();

        self.iter_system_pools().for_each(|pool| {
            let pool_ignore_reader = pool.ignore_once.read().unwrap();
            let pool_ignore = pool_ignore_reader.clone();
            std::mem::drop(pool_ignore_reader);

            if !pool_ignore {
                pool.systems.iter().par_bridge().for_each(|system| {
                    if !is_paused || system.ignore_pause {
                        (system.callback)(service_manager.clone());
                    }
                });
            } else {
                let mut pool_ignore_writer = pool.ignore_once.write().unwrap();
                *pool_ignore_writer = false;
            }
        });
    }

    /// Run all the stored begin systems
    pub fn run_begin(&self) {
        let service_manager = self.service_manager.clone();
        self.iter_begin_system_pools().for_each(|pool| {
            let pool_ignore_reader = pool.ignore_once.read().unwrap();
            let pool_ignore = pool_ignore_reader.clone();
            std::mem::drop(pool_ignore_reader);

            if !pool_ignore {
                pool.systems
                    .iter()
                    .par_bridge()
                    .for_each(|system| (system.callback)(service_manager.clone()));
            } else {
                let mut pool_ignore_writer = pool.ignore_once.write().unwrap();
                *pool_ignore_writer = false;
            }
        });
    }

    /// Run all the stored end systems
    pub fn run_end(&self) {
        let service_manager = self.service_manager.clone();
        self.iter_end_system_pools().for_each(|pool| {
            let pool_ignore_reader = pool.ignore_once.read().unwrap();
            let pool_ignore = pool_ignore_reader.clone();
            std::mem::drop(pool_ignore_reader);

            if !pool_ignore {
                pool.systems
                    .iter()
                    .par_bridge()
                    .for_each(|system| (system.callback)(service_manager.clone()));
            } else {
                let mut pool_ignore_writer = pool.ignore_once.write().unwrap();
                *pool_ignore_writer = false;
            }
        });
    }

    /// Run all the stored systems
    pub fn run_pool(&self, index: &usize) {
        if let Some(pool) = self.system_pools.get(index) {
            pool.systems
                .iter()
                .par_bridge()
                .for_each(|system| (system.callback)(self.service_manager.clone()));
        }
    }

    /// Run all the stored begin systems
    pub fn run_pool_begin(&self, index: &usize) {
        if let Some(pool) = self.begin_system_pools.get(index) {
            pool.systems
                .iter()
                .par_bridge()
                .for_each(|system| (system.callback)(self.service_manager.clone()));
        }
    }

    /// Run all the stored end systems
    pub fn run_poll_end(&self, index: &usize) {
        if let Some(pool) = self.end_system_pools.get(index) {
            pool.systems
                .iter()
                .par_bridge()
                .for_each(|system| (system.callback)(self.service_manager.clone()));
        }
    }

    /// Ignore a pool once
    ///
    /// # Arguments
    /// * `index` - The pool index
    ///
    pub fn ignore_pool_once(&self, index: &usize) {
        if let Some(pool) = self.system_pools.get(index) {
            let mut pool_ignore_writer = pool.ignore_once.write().unwrap();
            *pool_ignore_writer = true;
        }
    }

    /// Ignore a begin pool once
    ///
    /// # Arguments
    /// * `index` - The pool index
    ///
    pub fn ignore_begin_pool_once(&self, index: &usize) {
        if let Some(pool) = self.begin_system_pools.get(index) {
            let mut pool_ignore_writer = pool.ignore_once.write().unwrap();
            *pool_ignore_writer = true;
        }
    }

    /// Ignore an end pool once
    ///
    /// # Arguments
    /// * `index` - The pool index
    ///
    pub fn ignore_end_pool_once(&self, index: &usize) {
        if let Some(pool) = self.end_system_pools.get(index) {
            let mut pool_ignore_writer = pool.ignore_once.write().unwrap();
            *pool_ignore_writer = true;
        }
    }

    /// Is systems paused
    pub fn is_paused(&self) -> bool {
        self.pause.load(Ordering::Relaxed)
    }

    /// Set if systems are paused, only systems that ignore pause will be executed
    ///
    /// # Arguments
    /// * `paused` - The paused value
    ///
    pub fn set_paused(&self, paused: bool) {
        self.pause.store(paused, Ordering::Relaxed);
    }
}

impl IntrospectObject for SystemManager {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![
            MethodInfo {
                name: "add_system".to_string(),
                call: MethodCaller::Mut(Arc::new(|this, args| {
                    let this = cast_service_mut::<SystemManager>(this);

                    let mut caster = ArgumentCaster::new("add_system", args);
                    let arg1 = caster.cast_next::<Callback>()?;
                    let arg2 = caster.cast_next_optional::<usize>();

                    this.add_system(
                        move |_| {
                            match arg1(vec![]) {
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
                        move |_| {
                            match arg1(vec![]) {
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
                        move |_| {
                            match arg1(vec![]) {
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
                        move |_| {
                            match arg1(vec![]) {
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
                name: "is_paused".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_service::<SystemManager>(this);

                    let result = this.is_paused();

                    Ok(Some(result.into()))
                })),
            },
            MethodInfo {
                name: "set_paused".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_service::<SystemManager>(this);

                    let mut caster = ArgumentCaster::new("set_paused", args);
                    let arg1 = caster.cast_next::<bool>()?;

                    this.set_paused(arg1);

                    Ok(None)
                })),
            },
        ]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Service for SystemManager {}