use crate::ResourceContainer;
use fruity_any::*;
use fruity_core::convert::FruityInto;
use fruity_core::inject::Inject;
use fruity_core::inject::Inject0;
use fruity_core::introspect::log_introspect_error;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodCaller;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::serialize::serialized::Callback;
use fruity_core::utils::introspect::cast_introspect_mut;
use fruity_core::utils::introspect::cast_introspect_ref;
use fruity_core::utils::introspect::ArgumentCaster;
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::RwLock;

type SystemCallback = dyn Fn(Arc<ResourceContainer>) + Sync + Send + 'static;

#[derive(Clone)]
struct BeginSystem {
    origin: String,
    callback: Arc<SystemCallback>,
}

#[derive(Clone)]
struct EndSystem {
    origin: String,
    callback: Arc<SystemCallback>,
}

#[derive(Clone)]
struct FrameSystem {
    origin: String,
    callback: Arc<SystemCallback>,
    ignore_pause: bool,
}

/// A system pool, see [‘SystemService‘] for more informations
pub struct SystemPool<T> {
    /// Is the pool ignored, if it's not, it will not be launched when calling [‘SystemService‘]::run
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
/// Pool 98 is for camera
/// Pool 99 is for drawing
///
#[derive(FruityAny)]
pub struct SystemService {
    pause: AtomicBool,
    system_pools: BTreeMap<usize, SystemPool<FrameSystem>>,
    begin_system_pools: BTreeMap<usize, SystemPool<BeginSystem>>,
    end_system_pools: BTreeMap<usize, SystemPool<EndSystem>>,
    resource_container: Arc<ResourceContainer>,
}

impl Debug for SystemService {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'s> SystemService {
    /// Returns a SystemService
    pub fn new(resource_container: Arc<ResourceContainer>) -> SystemService {
        SystemService {
            pause: AtomicBool::new(false),
            system_pools: BTreeMap::new(),
            begin_system_pools: BTreeMap::new(),
            end_system_pools: BTreeMap::new(),
            resource_container: resource_container.clone(),
        }
    }

    /// Add a system to the collection
    ///
    /// # Arguments
    /// * `origin` - An identifier for the origin of the system, used for hot reload
    /// * `system` - A function that will compute the world
    /// * `pool_index` - A pool identifier, all the systems of the same pool will be processed together in parallel
    ///
    pub fn add_system<T: Inject>(&mut self, origin: &str, callback: T, pool_index: Option<usize>) {
        let pool_index = pool_index.unwrap_or(50);

        let system = FrameSystem {
            origin: origin.to_string(),
            callback: callback.inject().into(),
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
    /// * `origin` - An identifier for the origin of the system, used for hot reload
    /// * `system` - A function that will compute the world
    /// * `pool_index` - A pool identifier, all the systems of the same pool will be processed together in parallel
    ///
    pub fn add_system_that_ignore_pause<T: Inject>(
        &mut self,
        origin: &str,
        callback: T,
        pool_index: Option<usize>,
    ) {
        let pool_index = pool_index.unwrap_or(50);

        let system = FrameSystem {
            origin: origin.to_string(),
            callback: callback.inject().into(),
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
    /// * `origin` - An identifier for the origin of the system, used for hot reload
    /// * `system` - A function that will compute the world
    /// * `pool_index` - A pool identifier, all the systems of the same pool will be processed together in parallel
    ///
    pub fn add_begin_system<T: Inject>(
        &mut self,
        origin: &str,
        callback: T,
        pool_index: Option<usize>,
    ) {
        let pool_index = pool_index.unwrap_or(50);

        let system = BeginSystem {
            origin: origin.to_string(),
            callback: callback.inject().into(),
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
    /// * `origin` - An identifier for the origin of the system, used for hot reload
    /// * `system` - A function that will compute the world
    /// * `pool_index` - A pool identifier, all the systems of the same pool will be processed together in parallel
    ///
    pub fn add_end_system<T: Inject>(
        &mut self,
        origin: &str,
        callback: T,
        pool_index: Option<usize>,
    ) {
        let pool_index = pool_index.unwrap_or(50);

        let system = EndSystem {
            origin: origin.to_string(),
            callback: callback.inject().into(),
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

    /// Remove all systems with the given origin
    ///
    /// # Arguments
    /// * `origin` - An identifier for the origin of the system, used for hot reload
    ///
    pub fn unload_origin(&mut self, origin: &str) {
        self.system_pools.values_mut().for_each(|pool| {
            pool.systems = pool
                .systems
                .clone()
                .into_iter()
                .filter(|system| system.origin != origin)
                .collect::<Vec<_>>();
        });
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
        let resource_container = self.resource_container.clone();
        let is_paused = self.is_paused();

        self.iter_system_pools().for_each(|pool| {
            let pool_ignore_reader = pool.ignore_once.read().unwrap();
            let pool_ignore = pool_ignore_reader.clone();
            std::mem::drop(pool_ignore_reader);

            if !pool_ignore {
                pool.systems.iter().par_bridge().for_each(|system| {
                    if !is_paused || system.ignore_pause {
                        (system.callback)(resource_container.clone());
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
        let resource_container = self.resource_container.clone();
        self.iter_begin_system_pools().for_each(|pool| {
            let pool_ignore_reader = pool.ignore_once.read().unwrap();
            let pool_ignore = pool_ignore_reader.clone();
            std::mem::drop(pool_ignore_reader);

            if !pool_ignore {
                pool.systems
                    .iter()
                    .par_bridge()
                    .for_each(|system| (system.callback)(resource_container.clone()));
            } else {
                let mut pool_ignore_writer = pool.ignore_once.write().unwrap();
                *pool_ignore_writer = false;
            }
        });
    }

    /// Run all the stored end systems
    pub fn run_end(&self) {
        let resource_container = self.resource_container.clone();
        self.iter_end_system_pools().for_each(|pool| {
            let pool_ignore_reader = pool.ignore_once.read().unwrap();
            let pool_ignore = pool_ignore_reader.clone();
            std::mem::drop(pool_ignore_reader);

            if !pool_ignore {
                pool.systems
                    .iter()
                    .par_bridge()
                    .for_each(|system| (system.callback)(resource_container.clone()));
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
                .for_each(|system| (system.callback)(self.resource_container.clone()));
        }
    }

    /// Run all the stored begin systems
    pub fn run_pool_begin(&self, index: &usize) {
        if let Some(pool) = self.begin_system_pools.get(index) {
            pool.systems
                .iter()
                .par_bridge()
                .for_each(|system| (system.callback)(self.resource_container.clone()));
        }
    }

    /// Run all the stored end systems
    pub fn run_poll_end(&self, index: &usize) {
        if let Some(pool) = self.end_system_pools.get(index) {
            pool.systems
                .iter()
                .par_bridge()
                .for_each(|system| (system.callback)(self.resource_container.clone()));
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

impl IntrospectObject for SystemService {
    fn get_class_name(&self) -> String {
        "SystemService".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![
            MethodInfo {
                name: "add_system".to_string(),
                call: MethodCaller::Mut(Arc::new(|this, args| {
                    let this = cast_introspect_mut::<SystemService>(this);

                    let mut caster = ArgumentCaster::new("add_system", args);
                    let arg1 = caster.cast_next::<Callback>()?;
                    let arg2 = caster.cast_next_optional::<usize>();

                    let callback = arg1.callback;
                    this.add_system(
                        &arg1.origin,
                        Inject0::new(move || {
                            match callback(vec![]) {
                                Ok(_) => (),
                                Err(err) => log_introspect_error(&err),
                            };
                        }),
                        arg2,
                    );

                    Ok(None)
                })),
            },
            MethodInfo {
                name: "add_begin_system".to_string(),
                call: MethodCaller::Mut(Arc::new(|this, args| {
                    let this = cast_introspect_mut::<SystemService>(this);

                    let mut caster = ArgumentCaster::new("add_begin_system", args);
                    let arg1 = caster.cast_next::<Callback>()?;
                    let arg2 = caster.cast_next_optional::<usize>();

                    let callback = arg1.callback;
                    this.add_begin_system(
                        &arg1.origin,
                        Inject0::new(move || {
                            match callback(vec![]) {
                                Ok(_) => (),
                                Err(err) => log_introspect_error(&err),
                            };
                        }),
                        arg2,
                    );

                    Ok(None)
                })),
            },
            MethodInfo {
                name: "add_end_system".to_string(),
                call: MethodCaller::Mut(Arc::new(|this, args| {
                    let this = cast_introspect_mut::<SystemService>(this);

                    let mut caster = ArgumentCaster::new("add_end_system", args);
                    let arg1 = caster.cast_next::<Callback>()?;
                    let arg2 = caster.cast_next_optional::<usize>();

                    let callback = arg1.callback;
                    this.add_end_system(
                        &arg1.origin,
                        Inject0::new(move || {
                            match callback(vec![]) {
                                Ok(_) => (),
                                Err(err) => log_introspect_error(&err),
                            };
                        }),
                        arg2,
                    );

                    Ok(None)
                })),
            },
            MethodInfo {
                name: "is_paused".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_introspect_ref::<SystemService>(this);

                    let result = this.is_paused();

                    Ok(Some(result.fruity_into()))
                })),
            },
            MethodInfo {
                name: "set_paused".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_introspect_ref::<SystemService>(this);

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

impl Resource for SystemService {}
