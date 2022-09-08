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
use fruity_core::serialize::serialized::Serialized;
use fruity_core::utils::collection::drain_filter;
use fruity_core::utils::introspect::cast_introspect_mut;
use fruity_core::utils::introspect::cast_introspect_ref;
use fruity_core::utils::introspect::ArgumentCaster;
use fruity_core::Mutex;
use fruity_ecs_derive::*;
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

/// A callback for a system called every frame
pub type SystemCallback = dyn Fn(ResourceContainer) + Sync + Send + 'static;

/// A callback for a startup system dispose callback
pub type StartupDisposeSystemCallback = Option<Box<dyn FnOnce() + Sync + Send + 'static>>;

/// A callback for a startup system
pub type StartupSystemCallback =
    dyn Fn(ResourceContainer) -> StartupDisposeSystemCallback + Sync + Send + 'static;

/// Params for a system
#[derive(Debug, Clone, FruityAny, SerializableObject, IntrospectObject, InstantiableObject)]
pub struct SystemParams {
    /// The pool index
    pub pool_index: usize,

    /// If true, the system is still running while pause
    pub ignore_pause: bool,
}

impl Default for SystemParams {
    fn default() -> Self {
        Self {
            pool_index: 50,
            ignore_pause: false,
        }
    }
}

/// Params for a system
#[derive(Debug, Clone, FruityAny, SerializableObject, IntrospectObject, InstantiableObject)]
pub struct StartupSystemParams {
    /// If true, the system is still running while pause
    pub ignore_pause: bool,
}

impl Default for StartupSystemParams {
    fn default() -> Self {
        Self {
            ignore_pause: false,
        }
    }
}

#[derive(Clone)]
struct StartupSystem {
    identifier: String,
    origin: String,
    callback: Arc<StartupSystemCallback>,
    ignore_pause: bool,
}

struct StartupDisposeSystem {
    identifier: String,
    origin: String,
    callback: Box<dyn FnOnce() + Sync + Send + 'static>,
}

#[derive(Clone)]
struct FrameSystem {
    identifier: String,
    origin: String,
    callback: Arc<SystemCallback>,
    ignore_pause: bool,
}

/// A system pool, see [‘SystemService‘] for more informations
pub struct SystemPool<T> {
    /// Systems of the pool
    systems: Vec<T>,

    /// Is the pool enabled
    enabled: bool,
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
/// Pool 98 is for drawing
/// Pool 99 is for camera
///
#[derive(FruityAny)]
pub struct SystemService {
    pause: AtomicBool,
    system_pools: BTreeMap<usize, SystemPool<FrameSystem>>,
    startup_systems: Vec<StartupSystem>,
    startup_dispose_callbacks: Mutex<Vec<StartupDisposeSystem>>,
    startup_pause_dispose_callbacks: Mutex<Vec<StartupDisposeSystem>>,
    resource_container: ResourceContainer,
}

impl Debug for SystemService {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'s> SystemService {
    /// Returns a SystemService
    pub fn new(resource_container: ResourceContainer) -> SystemService {
        SystemService {
            pause: AtomicBool::new(true),
            system_pools: BTreeMap::new(),
            startup_systems: Vec::new(),
            startup_dispose_callbacks: Mutex::new(Vec::new()),
            startup_pause_dispose_callbacks: Mutex::new(Vec::new()),
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
    pub fn add_system<T: Inject>(
        &mut self,
        identifier: &str,
        origin: &str,
        callback: T,
        params: SystemParams,
    ) {
        let system = FrameSystem {
            identifier: identifier.to_string(),
            origin: origin.to_string(),
            callback: callback.inject().into(),
            ignore_pause: params.ignore_pause,
        };

        if let Some(pool) = self.system_pools.get_mut(&params.pool_index) {
            pool.systems.push(system)
        } else {
            // If the pool not exists, we create it
            let systems = vec![system];
            self.system_pools.insert(
                params.pool_index,
                SystemPool {
                    systems,
                    enabled: true,
                },
            );
        };
    }

    /// Add a startup system
    ///
    /// # Arguments
    /// * `origin` - An identifier for the origin of the system, used for hot reload
    /// * `system` - A function that will compute the world
    /// * `pool_index` - A pool identifier, all the systems of the same pool will be processed together in parallel
    ///
    pub fn add_startup_system<T: Inject<StartupDisposeSystemCallback>>(
        &mut self,
        identifier: &str,
        origin: &str,
        callback: T,
        params: StartupSystemParams,
    ) {
        let system = StartupSystem {
            identifier: identifier.to_string(),
            origin: origin.to_string(),
            callback: callback.inject().into(),
            ignore_pause: params.ignore_pause,
        };

        self.startup_systems.push(system);
    }

    /// Remove all systems with the given origin
    ///
    /// # Arguments
    /// * `origin` - An identifier for the origin of the system, used for hot reload
    ///
    pub fn unload_origin(&mut self, origin: &str) {
        self.system_pools.values_mut().for_each(|pool| {
            drain_filter(&mut pool.systems, |system| system.origin == origin);
        });

        {
            let mut startup_dispose_callbacks = self.startup_pause_dispose_callbacks.lock();

            drain_filter(&mut startup_dispose_callbacks, |system| {
                system.origin == origin
            })
            .into_iter()
            .for_each(|system| {
                let _profiler_scope = if puffin::are_scopes_on() {
                    // Safe cause identifier don't need to be static (from the doc)
                    let identifier = unsafe { &*(&system.identifier as *const _) } as &str;
                    Some(puffin::ProfilerScope::new(identifier, "dispose system", ""))
                } else {
                    None
                };

                (system.callback)()
            });
        }
    }

    /// Iter over all the systems pools
    fn iter_system_pools(&self) -> impl Iterator<Item = &SystemPool<FrameSystem>> {
        self.system_pools.iter().map(|pool| pool.1)
    }

    /// Run all the stored systems
    pub fn run(&self) {
        let resource_container = self.resource_container.clone();
        let is_paused = self.is_paused();

        self.iter_system_pools().for_each(|pool| {
            if pool.enabled {
                pool.systems.iter().par_bridge().for_each(|system| {
                    if !is_paused || system.ignore_pause {
                        let _profiler_scope = if puffin::are_scopes_on() {
                            // Safe cause identifier don't need to be static (from the doc)
                            let identifier = unsafe { &*(&system.identifier as *const _) } as &str;
                            Some(puffin::ProfilerScope::new(identifier, "system", ""))
                        } else {
                            None
                        };
                        (system.callback)(resource_container.clone());
                    }
                });
            }
        });
    }

    /// Run all the startup systems
    pub fn run_start(&self) {
        self.startup_systems
            .iter()
            .filter(|system| system.ignore_pause)
            .for_each(|system| {
                let _profiler_scope = if puffin::are_scopes_on() {
                    // Safe cause identifier don't need to be static (from the doc)
                    let identifier = unsafe { &*(&system.identifier as *const _) } as &str;
                    Some(puffin::ProfilerScope::new(identifier, "system", ""))
                } else {
                    None
                };

                let dispose_callback = (system.callback)(self.resource_container.clone());

                if let Some(dispose_callback) = dispose_callback {
                    let mut startup_dispose_callbacks = self.startup_dispose_callbacks.lock();
                    startup_dispose_callbacks.push(StartupDisposeSystem {
                        identifier: system.identifier.clone(),
                        origin: system.origin.clone(),
                        callback: dispose_callback,
                    });
                }
            });

        if !self.is_paused() {
            self.run_unpause_start();
        }
    }

    /// Run all startup dispose callbacks
    pub fn run_end(&self) {
        if !self.is_paused() {
            self.run_unpause_end();
        }

        let mut startup_dispose_callbacks = self.startup_dispose_callbacks.lock();
        startup_dispose_callbacks.drain(..).for_each(|system| {
            let _profiler_scope = if puffin::are_scopes_on() {
                // Safe cause identifier don't need to be static (from the doc)
                let identifier = unsafe { &*(&system.identifier as *const _) } as &str;
                Some(puffin::ProfilerScope::new(identifier, "dispose system", ""))
            } else {
                None
            };

            (system.callback)()
        });
    }

    /// Run all the startup systems that start when pause is stopped
    pub fn run_unpause_start(&self) {
        self.startup_systems
            .iter()
            .filter(|system| !system.ignore_pause)
            .for_each(|system| {
                let _profiler_scope = if puffin::are_scopes_on() {
                    // Safe cause identifier don't need to be static (from the doc)
                    let identifier = unsafe { &*(&system.identifier as *const _) } as &str;
                    Some(puffin::ProfilerScope::new(identifier, "system", ""))
                } else {
                    None
                };

                let dispose_callback = (system.callback)(self.resource_container.clone());

                if let Some(dispose_callback) = dispose_callback {
                    let mut startup_dispose_callbacks = self.startup_pause_dispose_callbacks.lock();
                    startup_dispose_callbacks.push(StartupDisposeSystem {
                        identifier: system.identifier.clone(),
                        origin: system.origin.clone(),
                        callback: dispose_callback,
                    });
                }
            });
    }

    /// Run all the startup dispose callbacks of systems that start when pause is stopped
    pub fn run_unpause_end(&self) {
        let mut startup_dispose_callbacks = self.startup_pause_dispose_callbacks.lock();
        startup_dispose_callbacks.drain(..).for_each(|system| {
            let _profiler_scope = if puffin::are_scopes_on() {
                // Safe cause identifier don't need to be static (from the doc)
                let identifier = unsafe { &*(&system.identifier as *const _) } as &str;
                Some(puffin::ProfilerScope::new(identifier, "dispose system", ""))
            } else {
                None
            };

            (system.callback)()
        });
    }

    /// Run all the systems contained in a pool
    pub fn run_pool(&self, index: &usize) {
        if let Some(pool) = self.system_pools.get(index) {
            pool.systems
                .iter()
                .par_bridge()
                .for_each(|system| (system.callback)(self.resource_container.clone()));
        }
    }

    /// Enable a pool
    pub fn enable_pool(&mut self, index: &usize) {
        if let Some(pool) = self.system_pools.get_mut(index) {
            pool.enabled = true;
        }
    }

    /// Disable a pool
    pub fn disable_pool(&mut self, index: &usize) {
        if let Some(pool) = self.system_pools.get_mut(index) {
            pool.enabled = false;
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
        if !paused && self.is_paused() {
            self.run_unpause_start();
        }

        if paused && !self.is_paused() {
            self.run_unpause_end();
        }

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
                    let arg1 = caster.cast_next::<String>()?;
                    let arg2 = caster.cast_next::<Callback>()?;
                    let arg3 = caster.cast_next_optional::<SystemParams>();

                    let callback = arg2.callback;
                    this.add_system(
                        &arg1,
                        &arg2.origin,
                        Inject0::new(move || {
                            match callback(vec![]) {
                                Ok(_) => (),
                                Err(err) => log_introspect_error(&err),
                            };
                        }),
                        arg3.unwrap_or_default(),
                    );

                    Ok(None)
                })),
            },
            MethodInfo {
                name: "add_startup_system".to_string(),
                call: MethodCaller::Mut(Arc::new(|this, args| {
                    let this = cast_introspect_mut::<SystemService>(this);

                    let mut caster = ArgumentCaster::new("add_startup_system", args);
                    let arg1 = caster.cast_next::<String>()?;
                    let arg2 = caster.cast_next::<Callback>()?;
                    let arg3 = caster.cast_next_optional::<StartupSystemParams>();

                    let callback = arg2.callback;
                    this.add_startup_system(
                        &arg1,
                        &arg2.origin,
                        Inject0::<StartupDisposeSystemCallback>::new(move || {
                            match callback(vec![]) {
                                Ok(result) => {
                                    if let Some(Serialized::Callback(callback)) = result {
                                        Some(Box::new(move || {
                                            (callback.callback)(vec![]).ok();
                                        }))
                                    } else {
                                        None
                                    }
                                }
                                Err(err) => {
                                    log_introspect_error(&err);
                                    None
                                }
                            }
                        }),
                        arg3.unwrap_or_default(),
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
