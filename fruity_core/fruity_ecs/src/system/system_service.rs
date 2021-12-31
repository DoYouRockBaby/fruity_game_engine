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
use fruity_ecs_derive::*;
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

type SystemCallback = dyn Fn(Arc<ResourceContainer>) + Sync + Send + 'static;

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
pub struct BeginSystemParams {
    /// The pool index
    pub pool_index: usize,
}

impl Default for BeginSystemParams {
    fn default() -> Self {
        Self { pool_index: 50 }
    }
}

/// Params for a system
#[derive(Debug, Clone, FruityAny, SerializableObject, IntrospectObject, InstantiableObject)]
pub struct EndSystemParams {
    /// The pool index
    pub pool_index: usize,
}

impl Default for EndSystemParams {
    fn default() -> Self {
        Self { pool_index: 50 }
    }
}

#[derive(Clone)]
struct BeginSystem {
    identifier: String,
    origin: String,
    callback: Arc<SystemCallback>,
}

#[derive(Clone)]
struct EndSystem {
    identifier: String,
    origin: String,
    callback: Arc<SystemCallback>,
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
    pub fn add_system<T: Inject>(
        &mut self,
        identifier: &str,
        origin: &str,
        callback: T,
        params: Option<SystemParams>,
    ) {
        let params = params.unwrap_or_default();

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

    /// Add a begin system to the collection
    ///
    /// # Arguments
    /// * `origin` - An identifier for the origin of the system, used for hot reload
    /// * `system` - A function that will compute the world
    /// * `pool_index` - A pool identifier, all the systems of the same pool will be processed together in parallel
    ///
    pub fn add_begin_system<T: Inject>(
        &mut self,
        identifier: &str,
        origin: &str,
        callback: T,
        params: Option<BeginSystemParams>,
    ) {
        let params = params.unwrap_or_default();

        let system = BeginSystem {
            identifier: identifier.to_string(),
            origin: origin.to_string(),
            callback: callback.inject().into(),
        };

        if let Some(pool) = self.begin_system_pools.get_mut(&params.pool_index) {
            pool.systems.push(system)
        } else {
            // If the pool not exists, we create it
            let systems = vec![system];
            self.begin_system_pools.insert(
                params.pool_index,
                SystemPool {
                    systems,
                    enabled: true,
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
        identifier: &str,
        origin: &str,
        callback: T,
        params: Option<EndSystemParams>,
    ) {
        let params = params.unwrap_or_default();

        let system = EndSystem {
            identifier: identifier.to_string(),
            origin: origin.to_string(),
            callback: callback.inject().into(),
        };

        if let Some(pool) = self.end_system_pools.get_mut(&params.pool_index) {
            pool.systems.push(system)
        } else {
            // If the pool not exists, we create it
            let systems = vec![system];
            self.end_system_pools.insert(
                params.pool_index,
                SystemPool {
                    systems,
                    enabled: true,
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

    /// Run all the stored begin systems
    pub fn run_begin(&self) {
        let resource_container = self.resource_container.clone();
        self.iter_begin_system_pools().for_each(|pool| {
            if pool.enabled {
                pool.systems.iter().par_bridge().for_each(|system| {
                    let _profiler_scope = if puffin::are_scopes_on() {
                        // Safe cause identifier don't need to be static (from the doc)
                        let identifier = unsafe { &*(&system.identifier as *const _) } as &str;
                        Some(puffin::ProfilerScope::new(identifier, "system", ""))
                    } else {
                        None
                    };

                    (system.callback)(resource_container.clone());
                });
            }
        });
    }

    /// Run all the stored end systems
    pub fn run_end(&self) {
        let resource_container = self.resource_container.clone();
        self.iter_end_system_pools().for_each(|pool| {
            if pool.enabled {
                pool.systems.iter().par_bridge().for_each(|system| {
                    let _profiler_scope = if puffin::are_scopes_on() {
                        // Safe cause identifier don't need to be static (from the doc)
                        let identifier = unsafe { &*(&system.identifier as *const _) } as &str;
                        Some(puffin::ProfilerScope::new(identifier, "system", ""))
                    } else {
                        None
                    };

                    (system.callback)(resource_container.clone())
                });
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
                        arg3,
                    );

                    Ok(None)
                })),
            },
            MethodInfo {
                name: "add_begin_system".to_string(),
                call: MethodCaller::Mut(Arc::new(|this, args| {
                    let this = cast_introspect_mut::<SystemService>(this);

                    let mut caster = ArgumentCaster::new("add_begin_system", args);
                    let arg1 = caster.cast_next::<String>()?;
                    let arg2 = caster.cast_next::<Callback>()?;
                    let arg3 = caster.cast_next_optional::<BeginSystemParams>();

                    let callback = arg2.callback;
                    this.add_begin_system(
                        &arg1,
                        &arg2.origin,
                        Inject0::new(move || {
                            match callback(vec![]) {
                                Ok(_) => (),
                                Err(err) => log_introspect_error(&err),
                            };
                        }),
                        arg3,
                    );

                    Ok(None)
                })),
            },
            MethodInfo {
                name: "add_end_system".to_string(),
                call: MethodCaller::Mut(Arc::new(|this, args| {
                    let this = cast_introspect_mut::<SystemService>(this);

                    let mut caster = ArgumentCaster::new("add_end_system", args);
                    let arg1 = caster.cast_next::<String>()?;
                    let arg2 = caster.cast_next::<Callback>()?;
                    let arg3 = caster.cast_next_optional::<EndSystemParams>();

                    let callback = arg2.callback;
                    this.add_end_system(
                        &arg1,
                        &arg2.origin,
                        Inject0::new(move || {
                            match callback(vec![]) {
                                Ok(_) => (),
                                Err(err) => log_introspect_error(&err),
                            };
                        }),
                        arg3,
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
