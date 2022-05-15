use crate::component::component::Component;
use crate::component::component::StaticComponent;
use crate::component::component_guard::TypedComponentReadGuard;
use crate::component::component_guard::TypedComponentWriteGuard;
use crate::entity::entity::EntityId;
use crate::entity::entity_guard::EntityReadGuard;
use crate::entity::entity_guard::EntityWriteGuard;
use crate::entity::entity_reference::EntityReference;
use std::sync::Arc;

/// A readable component reference
pub type Read<'s, T> = TypedComponentReadGuard<'s, T>;

/// A writable component reference
pub type Write<'s, T> = TypedComponentWriteGuard<'s, T>;

/// An enum to pass a guard into the [’QueryInjectable’]
#[derive(Clone)]
pub enum RequestedEntityGuard<'a> {
    /// No guard required
    None,
    /// Read guard required
    Read(EntityReadGuard<'a>),
    /// Write guard required
    Write(EntityWriteGuard<'a>),
}

/// A trait for types that can be exposed from components references
pub trait QueryInjectable: Sized + Clone {
    /// Does this require a read guard over the reference
    fn require_read() -> bool;

    /// Does this require a write guard over the reference
    fn require_write() -> bool;

    /// Get the object
    fn from_components(entity: &EntityReference, entity_guard: &RequestedEntityGuard) -> Vec<Self>;
}

impl QueryInjectable for EntityReference {
    fn require_read() -> bool {
        false
    }

    fn require_write() -> bool {
        false
    }

    fn from_components(
        entity: &EntityReference,
        _entity_guard: &RequestedEntityGuard,
    ) -> Vec<Self> {
        vec![entity.clone()]
    }
}

impl QueryInjectable for EntityId {
    fn require_read() -> bool {
        true
    }

    fn require_write() -> bool {
        false
    }

    fn from_components(
        _entity: &EntityReference,
        entity_guard: &RequestedEntityGuard,
    ) -> Vec<Self> {
        match entity_guard {
            RequestedEntityGuard::Read(guard) => vec![guard.get_entity_id()],
            RequestedEntityGuard::Write(guard) => vec![guard.get_entity_id()],
            RequestedEntityGuard::None => panic!(),
        }
    }
}

impl QueryInjectable for String {
    fn require_read() -> bool {
        true
    }

    fn require_write() -> bool {
        false
    }

    fn from_components(
        _entity: &EntityReference,
        entity_guard: &RequestedEntityGuard,
    ) -> Vec<Self> {
        match entity_guard {
            RequestedEntityGuard::Read(guard) => vec![guard.get_name()],
            RequestedEntityGuard::Write(guard) => vec![guard.get_name()],
            RequestedEntityGuard::None => panic!(),
        }
    }
}

impl QueryInjectable for bool {
    fn require_read() -> bool {
        true
    }

    fn require_write() -> bool {
        false
    }

    fn from_components(
        _entity: &EntityReference,
        entity_guard: &RequestedEntityGuard,
    ) -> Vec<Self> {
        match entity_guard {
            RequestedEntityGuard::Read(guard) => vec![guard.is_enabled()],
            RequestedEntityGuard::Write(guard) => vec![guard.is_enabled()],
            RequestedEntityGuard::None => panic!(),
        }
    }
}

impl<T: Component + StaticComponent> QueryInjectable for Read<'_, T> {
    fn require_read() -> bool {
        true
    }

    fn require_write() -> bool {
        false
    }

    fn from_components(
        _entity: &EntityReference,
        entity_guard: &RequestedEntityGuard,
    ) -> Vec<Self> {
        let entity_guard = unsafe {
            std::mem::transmute::<&RequestedEntityGuard, &RequestedEntityGuard>(entity_guard)
        };

        match entity_guard {
            RequestedEntityGuard::Read(guard) => guard.read_components::<T>(),
            RequestedEntityGuard::Write(guard) => guard.read_components::<T>(),
            RequestedEntityGuard::None => panic!(),
        }
    }
}

impl<T: Component + StaticComponent> QueryInjectable for Write<'_, T> {
    fn require_read() -> bool {
        false
    }

    fn require_write() -> bool {
        true
    }

    fn from_components(
        _entity: &EntityReference,
        entity_guard: &RequestedEntityGuard,
    ) -> Vec<Self> {
        let entity_guard = unsafe {
            std::mem::transmute::<&RequestedEntityGuard, &RequestedEntityGuard>(entity_guard)
        };

        match entity_guard {
            RequestedEntityGuard::Read(_) => panic!(),
            RequestedEntityGuard::Write(guard) => guard.write_components::<T>(),
            RequestedEntityGuard::None => panic!(),
        }
    }
}

/// A trait that is implemented by functions that supports dependency injection
pub trait QueryInject: Send + Sync {
    /// Duplicate the query injector
    fn duplicate(&self) -> Self;

    /// Get a function that proceed the injection
    fn inject(self) -> Box<dyn Fn(&EntityReference) + Send + Sync>;
}

/// A shortcut for a boxed inject function
#[derive(Clone)]
pub struct Inject0(Arc<dyn Fn() + Send + Sync>);

impl Inject0 {
    /// New instance
    pub fn new(val: impl Fn() + Send + Sync + 'static) -> Self {
        Self(Arc::new(val))
    }
}

impl QueryInject for Inject0 {
    fn duplicate(&self) -> Self {
        Self(self.0.clone())
    }

    fn inject(self) -> Box<dyn Fn(&EntityReference) + Send + Sync> {
        Box::new(move |_| (self.0)())
    }
}

/// A shortcut for a boxed inject function
#[derive(Clone)]
pub struct Inject1<T1>(Arc<dyn Fn(T1) + Send + Sync>);

impl<T1> Inject1<T1> {
    /// New instance
    pub fn new(val: impl Fn(T1) + Send + Sync + 'static) -> Self {
        Self(Arc::new(val))
    }
}

impl<T1: QueryInjectable + 'static> QueryInject for Inject1<T1> {
    fn duplicate(&self) -> Self {
        Self(self.0.clone())
    }

    fn inject(self) -> Box<dyn Fn(&EntityReference) + Send + Sync> {
        Box::new(move |entity| {
            let require_read = T1::require_read();
            let require_write = T1::require_write();

            let entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            for param1 in T1::from_components(&entity, &entity_guard) {
                (self.0)(param1);
            }
        })
    }
}

/// A shortcut for a boxed inject function
#[derive(Clone)]
pub struct Inject2<T1, T2>(Arc<dyn Fn(T1, T2) + Send + Sync>);

impl<T1, T2> Inject2<T1, T2> {
    /// New instance
    pub fn new(val: impl Fn(T1, T2) + Send + Sync + 'static) -> Self {
        Self(Arc::new(val))
    }
}

impl<T1: QueryInjectable + 'static, T2: QueryInjectable + 'static> QueryInject for Inject2<T1, T2> {
    fn duplicate(&self) -> Self {
        Self(self.0.clone())
    }

    fn inject(self) -> Box<dyn Fn(&EntityReference) + Send + Sync> {
        Box::new(move |entity| {
            let require_read = T1::require_read() || T2::require_read();
            let require_write = T1::require_write() || T2::require_write();

            let entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            for param1 in T1::from_components(&entity, &entity_guard) {
                for param2 in T2::from_components(&entity, &entity_guard) {
                    (self.0)(param1.clone(), param2.clone());
                }
            }
        })
    }
}

/// A shortcut for a boxed inject function
#[derive(Clone)]
pub struct Inject3<T1, T2, T3>(Arc<dyn Fn(T1, T2, T3) + Send + Sync>);

impl<T1, T2, T3> Inject3<T1, T2, T3> {
    /// New instance
    pub fn new(val: impl Fn(T1, T2, T3) + Send + Sync + 'static) -> Self {
        Self(Arc::new(val))
    }
}

impl<
        T1: QueryInjectable + 'static,
        T2: QueryInjectable + 'static,
        T3: QueryInjectable + 'static,
    > QueryInject for Inject3<T1, T2, T3>
{
    fn duplicate(&self) -> Self {
        Self(self.0.clone())
    }

    fn inject(self) -> Box<dyn Fn(&EntityReference) + Send + Sync> {
        Box::new(move |entity| {
            let require_read = T1::require_read() || T2::require_read() || T3::require_read();
            let require_write = T1::require_write() || T2::require_write() || T3::require_write();

            let entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            for param1 in T1::from_components(&entity, &entity_guard) {
                for param2 in T2::from_components(&entity, &entity_guard) {
                    for param3 in T3::from_components(&entity, &entity_guard) {
                        (self.0)(param1.clone(), param2.clone(), param3.clone());
                    }
                }
            }
        })
    }
}

/// A shortcut for a boxed inject function
#[derive(Clone)]
pub struct Inject4<T1, T2, T3, T4>(Arc<dyn Fn(T1, T2, T3, T4) + Send + Sync>);

impl<T1, T2, T3, T4> Inject4<T1, T2, T3, T4> {
    /// New instance
    pub fn new(val: impl Fn(T1, T2, T3, T4) + Send + Sync + 'static) -> Self {
        Self(Arc::new(val))
    }
}

impl<
        T1: QueryInjectable + 'static,
        T2: QueryInjectable + 'static,
        T3: QueryInjectable + 'static,
        T4: QueryInjectable + 'static,
    > QueryInject for Inject4<T1, T2, T3, T4>
{
    fn duplicate(&self) -> Self {
        Self(self.0.clone())
    }

    fn inject(self) -> Box<dyn Fn(&EntityReference) + Send + Sync> {
        Box::new(move |entity| {
            let require_read = T1::require_read()
                || T2::require_read()
                || T3::require_read()
                || T4::require_read();
            let require_write = T1::require_write()
                || T2::require_write()
                || T3::require_write()
                || T4::require_write();

            let entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            for param1 in T1::from_components(&entity, &entity_guard) {
                for param2 in T2::from_components(&entity, &entity_guard) {
                    for param3 in T3::from_components(&entity, &entity_guard) {
                        for param4 in T4::from_components(&entity, &entity_guard) {
                            (self.0)(
                                param1.clone(),
                                param2.clone(),
                                param3.clone(),
                                param4.clone(),
                            );
                        }
                    }
                }
            }
        })
    }
}

/// A shortcut for a boxed inject function
#[derive(Clone)]
pub struct Inject5<T1, T2, T3, T4, T5>(Arc<dyn Fn(T1, T2, T3, T4, T5) + Send + Sync>);

impl<T1, T2, T3, T4, T5> Inject5<T1, T2, T3, T4, T5> {
    /// New instance
    pub fn new(val: impl Fn(T1, T2, T3, T4, T5) + Send + Sync + 'static) -> Self {
        Self(Arc::new(val))
    }
}

impl<
        T1: QueryInjectable + 'static,
        T2: QueryInjectable + 'static,
        T3: QueryInjectable + 'static,
        T4: QueryInjectable + 'static,
        T5: QueryInjectable + 'static,
    > QueryInject for Inject5<T1, T2, T3, T4, T5>
{
    fn duplicate(&self) -> Self {
        Self(self.0.clone())
    }

    fn inject(self) -> Box<dyn Fn(&EntityReference) + Send + Sync> {
        Box::new(move |entity| {
            let require_read = T1::require_read()
                || T2::require_read()
                || T3::require_read()
                || T4::require_read()
                || T5::require_read();
            let require_write = T1::require_write()
                || T2::require_write()
                || T3::require_write()
                || T4::require_write()
                || T5::require_write();

            let entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            for param1 in T1::from_components(&entity, &entity_guard) {
                for param2 in T2::from_components(&entity, &entity_guard) {
                    for param3 in T3::from_components(&entity, &entity_guard) {
                        for param4 in T4::from_components(&entity, &entity_guard) {
                            for param5 in T5::from_components(&entity, &entity_guard) {
                                (self.0)(
                                    param1.clone(),
                                    param2.clone(),
                                    param3.clone(),
                                    param4.clone(),
                                    param5.clone(),
                                );
                            }
                        }
                    }
                }
            }
        })
    }
}

/// A shortcut for a boxed inject function
#[derive(Clone)]
pub struct Inject6<T1, T2, T3, T4, T5, T6>(Arc<dyn Fn(T1, T2, T3, T4, T5, T6) + Send + Sync>);

impl<T1, T2, T3, T4, T5, T6> Inject6<T1, T2, T3, T4, T5, T6> {
    /// New instance
    pub fn new(val: impl Fn(T1, T2, T3, T4, T5, T6) + Send + Sync + 'static) -> Self {
        Self(Arc::new(val))
    }
}

impl<
        T1: QueryInjectable + 'static,
        T2: QueryInjectable + 'static,
        T3: QueryInjectable + 'static,
        T4: QueryInjectable + 'static,
        T5: QueryInjectable + 'static,
        T6: QueryInjectable + 'static,
    > QueryInject for Inject6<T1, T2, T3, T4, T5, T6>
{
    fn duplicate(&self) -> Self {
        Self(self.0.clone())
    }

    fn inject(self) -> Box<dyn Fn(&EntityReference) + Send + Sync> {
        Box::new(move |entity| {
            let require_read = T1::require_read()
                || T2::require_read()
                || T3::require_read()
                || T4::require_read()
                || T5::require_read()
                || T6::require_read();
            let require_write = T1::require_write()
                || T2::require_write()
                || T3::require_write()
                || T4::require_write()
                || T5::require_write()
                || T6::require_write();

            let entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            for param1 in T1::from_components(&entity, &entity_guard) {
                for param2 in T2::from_components(&entity, &entity_guard) {
                    for param3 in T3::from_components(&entity, &entity_guard) {
                        for param4 in T4::from_components(&entity, &entity_guard) {
                            for param5 in T5::from_components(&entity, &entity_guard) {
                                for param6 in T6::from_components(&entity, &entity_guard) {
                                    (self.0)(
                                        param1.clone(),
                                        param2.clone(),
                                        param3.clone(),
                                        param4.clone(),
                                        param5.clone(),
                                        param6.clone(),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}

/// A shortcut for a boxed inject function
#[derive(Clone)]
pub struct Inject7<T1, T2, T3, T4, T5, T6, T7>(
    Arc<dyn Fn(T1, T2, T3, T4, T5, T6, T7) + Send + Sync>,
);

impl<T1, T2, T3, T4, T5, T6, T7> Inject7<T1, T2, T3, T4, T5, T6, T7> {
    /// New instance
    pub fn new(val: impl Fn(T1, T2, T3, T4, T5, T6, T7) + Send + Sync + 'static) -> Self {
        Self(Arc::new(val))
    }
}

impl<
        T1: QueryInjectable + 'static,
        T2: QueryInjectable + 'static,
        T3: QueryInjectable + 'static,
        T4: QueryInjectable + 'static,
        T5: QueryInjectable + 'static,
        T6: QueryInjectable + 'static,
        T7: QueryInjectable + 'static,
    > QueryInject for Inject7<T1, T2, T3, T4, T5, T6, T7>
{
    fn duplicate(&self) -> Self {
        Self(self.0.clone())
    }

    fn inject(self) -> Box<dyn Fn(&EntityReference) + Send + Sync> {
        Box::new(move |entity| {
            let require_read = T1::require_read()
                || T2::require_read()
                || T3::require_read()
                || T4::require_read()
                || T5::require_read()
                || T6::require_read()
                || T7::require_read();
            let require_write = T1::require_write()
                || T2::require_write()
                || T3::require_write()
                || T4::require_write()
                || T5::require_write()
                || T6::require_write()
                || T7::require_write();

            let entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            for param1 in T1::from_components(&entity, &entity_guard) {
                for param2 in T2::from_components(&entity, &entity_guard) {
                    for param3 in T3::from_components(&entity, &entity_guard) {
                        for param4 in T4::from_components(&entity, &entity_guard) {
                            for param5 in T5::from_components(&entity, &entity_guard) {
                                for param6 in T6::from_components(&entity, &entity_guard) {
                                    for param7 in T7::from_components(&entity, &entity_guard) {
                                        (self.0)(
                                            param1.clone(),
                                            param2.clone(),
                                            param3.clone(),
                                            param4.clone(),
                                            param5.clone(),
                                            param6.clone(),
                                            param7.clone(),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}

/// A shortcut for a boxed inject function
#[derive(Clone)]
pub struct Inject8<T1, T2, T3, T4, T5, T6, T7, T8>(
    Arc<dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8) + Send + Sync>,
);

impl<T1, T2, T3, T4, T5, T6, T7, T8> Inject8<T1, T2, T3, T4, T5, T6, T7, T8> {
    /// New instance
    pub fn new(val: impl Fn(T1, T2, T3, T4, T5, T6, T7, T8) + Send + Sync + 'static) -> Self {
        Self(Arc::new(val))
    }
}

impl<
        T1: QueryInjectable + 'static,
        T2: QueryInjectable + 'static,
        T3: QueryInjectable + 'static,
        T4: QueryInjectable + 'static,
        T5: QueryInjectable + 'static,
        T6: QueryInjectable + 'static,
        T7: QueryInjectable + 'static,
        T8: QueryInjectable + 'static,
    > QueryInject for Inject8<T1, T2, T3, T4, T5, T6, T7, T8>
{
    fn duplicate(&self) -> Self {
        Self(self.0.clone())
    }

    fn inject(self) -> Box<dyn Fn(&EntityReference) + Send + Sync> {
        Box::new(move |entity| {
            let require_read = T1::require_read()
                || T2::require_read()
                || T3::require_read()
                || T4::require_read()
                || T5::require_read()
                || T6::require_read()
                || T7::require_read()
                || T8::require_read();
            let require_write = T1::require_write()
                || T2::require_write()
                || T3::require_write()
                || T4::require_write()
                || T5::require_write()
                || T6::require_write()
                || T7::require_write()
                || T8::require_write();

            let entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            for param1 in T1::from_components(&entity, &entity_guard) {
                for param2 in T2::from_components(&entity, &entity_guard) {
                    for param3 in T3::from_components(&entity, &entity_guard) {
                        for param4 in T4::from_components(&entity, &entity_guard) {
                            for param5 in T5::from_components(&entity, &entity_guard) {
                                for param6 in T6::from_components(&entity, &entity_guard) {
                                    for param7 in T7::from_components(&entity, &entity_guard) {
                                        for param8 in T8::from_components(&entity, &entity_guard) {
                                            (self.0)(
                                                param1.clone(),
                                                param2.clone(),
                                                param3.clone(),
                                                param4.clone(),
                                                param5.clone(),
                                                param6.clone(),
                                                param7.clone(),
                                                param8.clone(),
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}

/// A shortcut for a boxed inject function
#[derive(Clone)]
pub struct Inject9<T1, T2, T3, T4, T5, T6, T7, T8, T9>(
    Arc<dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9) + Send + Sync>,
);

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9> Inject9<T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    /// New instance
    pub fn new(val: impl Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9) + Send + Sync + 'static) -> Self {
        Self(Arc::new(val))
    }
}

impl<
        T1: QueryInjectable + 'static,
        T2: QueryInjectable + 'static,
        T3: QueryInjectable + 'static,
        T4: QueryInjectable + 'static,
        T5: QueryInjectable + 'static,
        T6: QueryInjectable + 'static,
        T7: QueryInjectable + 'static,
        T8: QueryInjectable + 'static,
        T9: QueryInjectable + 'static,
    > QueryInject for Inject9<T1, T2, T3, T4, T5, T6, T7, T8, T9>
{
    fn duplicate(&self) -> Self {
        Self(self.0.clone())
    }

    fn inject(self) -> Box<dyn Fn(&EntityReference) + Send + Sync> {
        Box::new(move |entity| {
            let require_read = T1::require_read()
                || T2::require_read()
                || T3::require_read()
                || T4::require_read()
                || T5::require_read()
                || T6::require_read()
                || T7::require_read()
                || T8::require_read()
                || T9::require_read();
            let require_write = T1::require_write()
                || T2::require_write()
                || T3::require_write()
                || T4::require_write()
                || T5::require_write()
                || T6::require_write()
                || T7::require_write()
                || T8::require_write()
                || T9::require_write();

            let entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            for param1 in T1::from_components(&entity, &entity_guard) {
                for param2 in T2::from_components(&entity, &entity_guard) {
                    for param3 in T3::from_components(&entity, &entity_guard) {
                        for param4 in T4::from_components(&entity, &entity_guard) {
                            for param5 in T5::from_components(&entity, &entity_guard) {
                                for param6 in T6::from_components(&entity, &entity_guard) {
                                    for param7 in T7::from_components(&entity, &entity_guard) {
                                        for param8 in T8::from_components(&entity, &entity_guard) {
                                            for param9 in
                                                T9::from_components(&entity, &entity_guard)
                                            {
                                                (self.0)(
                                                    param1.clone(),
                                                    param2.clone(),
                                                    param3.clone(),
                                                    param4.clone(),
                                                    param5.clone(),
                                                    param6.clone(),
                                                    param7.clone(),
                                                    param8.clone(),
                                                    param9.clone(),
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}

/// A shortcut for a boxed inject function
#[derive(Clone)]
pub struct Inject10<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>(
    Arc<dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) + Send + Sync>,
);

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Inject10<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    /// New instance
    pub fn new(
        val: impl Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) + Send + Sync + 'static,
    ) -> Self {
        Self(Arc::new(val))
    }
}

impl<
        T1: QueryInjectable + 'static,
        T2: QueryInjectable + 'static,
        T3: QueryInjectable + 'static,
        T4: QueryInjectable + 'static,
        T5: QueryInjectable + 'static,
        T6: QueryInjectable + 'static,
        T7: QueryInjectable + 'static,
        T8: QueryInjectable + 'static,
        T9: QueryInjectable + 'static,
        T10: QueryInjectable + 'static,
    > QueryInject for Inject10<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>
{
    fn duplicate(&self) -> Self {
        Self(self.0.clone())
    }

    fn inject(self) -> Box<dyn Fn(&EntityReference) + Send + Sync> {
        Box::new(move |entity| {
            let require_read = T1::require_read()
                || T2::require_read()
                || T3::require_read()
                || T4::require_read()
                || T5::require_read()
                || T6::require_read()
                || T7::require_read()
                || T8::require_read()
                || T9::require_read()
                || T10::require_read();
            let require_write = T1::require_write()
                || T2::require_write()
                || T3::require_write()
                || T4::require_write()
                || T5::require_write()
                || T6::require_write()
                || T7::require_write()
                || T8::require_write()
                || T9::require_write()
                || T10::require_write();

            let entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            for param1 in T1::from_components(&entity, &entity_guard) {
                for param2 in T2::from_components(&entity, &entity_guard) {
                    for param3 in T3::from_components(&entity, &entity_guard) {
                        for param4 in T4::from_components(&entity, &entity_guard) {
                            for param5 in T5::from_components(&entity, &entity_guard) {
                                for param6 in T6::from_components(&entity, &entity_guard) {
                                    for param7 in T7::from_components(&entity, &entity_guard) {
                                        for param8 in T8::from_components(&entity, &entity_guard) {
                                            for param9 in
                                                T9::from_components(&entity, &entity_guard)
                                            {
                                                for param10 in
                                                    T10::from_components(&entity, &entity_guard)
                                                {
                                                    (self.0)(
                                                        param1.clone(),
                                                        param2.clone(),
                                                        param3.clone(),
                                                        param4.clone(),
                                                        param5.clone(),
                                                        param6.clone(),
                                                        param7.clone(),
                                                        param8.clone(),
                                                        param9.clone(),
                                                        param10.clone(),
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}

/// A shortcut for a boxed inject function
#[derive(Clone)]
pub struct Inject11<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>(
    Arc<dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) + Send + Sync>,
);

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
    Inject11<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
{
    /// New instance
    pub fn new(
        val: impl Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) + Send + Sync + 'static,
    ) -> Self {
        Self(Arc::new(val))
    }
}

impl<
        T1: QueryInjectable + 'static,
        T2: QueryInjectable + 'static,
        T3: QueryInjectable + 'static,
        T4: QueryInjectable + 'static,
        T5: QueryInjectable + 'static,
        T6: QueryInjectable + 'static,
        T7: QueryInjectable + 'static,
        T8: QueryInjectable + 'static,
        T9: QueryInjectable + 'static,
        T10: QueryInjectable + 'static,
        T11: QueryInjectable + 'static,
    > QueryInject for Inject11<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
{
    fn duplicate(&self) -> Self {
        Self(self.0.clone())
    }

    fn inject(self) -> Box<dyn Fn(&EntityReference) + Send + Sync> {
        Box::new(move |entity| {
            let require_read = T1::require_read()
                || T2::require_read()
                || T3::require_read()
                || T4::require_read()
                || T5::require_read()
                || T6::require_read()
                || T7::require_read()
                || T8::require_read()
                || T9::require_read()
                || T10::require_read()
                || T11::require_read();
            let require_write = T1::require_write()
                || T2::require_write()
                || T3::require_write()
                || T4::require_write()
                || T5::require_write()
                || T6::require_write()
                || T7::require_write()
                || T8::require_write()
                || T9::require_write()
                || T10::require_write()
                || T11::require_write();

            let entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            for param1 in T1::from_components(&entity, &entity_guard) {
                for param2 in T2::from_components(&entity, &entity_guard) {
                    for param3 in T3::from_components(&entity, &entity_guard) {
                        for param4 in T4::from_components(&entity, &entity_guard) {
                            for param5 in T5::from_components(&entity, &entity_guard) {
                                for param6 in T6::from_components(&entity, &entity_guard) {
                                    for param7 in T7::from_components(&entity, &entity_guard) {
                                        for param8 in T8::from_components(&entity, &entity_guard) {
                                            for param9 in
                                                T9::from_components(&entity, &entity_guard)
                                            {
                                                for param10 in
                                                    T10::from_components(&entity, &entity_guard)
                                                {
                                                    for param11 in
                                                        T11::from_components(&entity, &entity_guard)
                                                    {
                                                        (self.0)(
                                                            param1.clone(),
                                                            param2.clone(),
                                                            param3.clone(),
                                                            param4.clone(),
                                                            param5.clone(),
                                                            param6.clone(),
                                                            param7.clone(),
                                                            param8.clone(),
                                                            param9.clone(),
                                                            param10.clone(),
                                                            param11.clone(),
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}

/// A shortcut for a boxed inject function
#[derive(Clone)]
pub struct Inject12<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>(
    Arc<dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) + Send + Sync>,
);

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>
    Inject12<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>
{
    /// New instance
    pub fn new(
        val: impl Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) + Send + Sync + 'static,
    ) -> Self {
        Self(Arc::new(val))
    }
}

impl<
        T1: QueryInjectable + 'static,
        T2: QueryInjectable + 'static,
        T3: QueryInjectable + 'static,
        T4: QueryInjectable + 'static,
        T5: QueryInjectable + 'static,
        T6: QueryInjectable + 'static,
        T7: QueryInjectable + 'static,
        T8: QueryInjectable + 'static,
        T9: QueryInjectable + 'static,
        T10: QueryInjectable + 'static,
        T11: QueryInjectable + 'static,
        T12: QueryInjectable + 'static,
    > QueryInject for Inject12<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>
{
    fn duplicate(&self) -> Self {
        Self(self.0.clone())
    }

    fn inject(self) -> Box<dyn Fn(&EntityReference) + Send + Sync> {
        Box::new(move |entity| {
            let require_read = T1::require_read()
                || T2::require_read()
                || T3::require_read()
                || T4::require_read()
                || T5::require_read()
                || T6::require_read()
                || T7::require_read()
                || T8::require_read()
                || T9::require_read()
                || T10::require_read()
                || T11::require_read()
                || T12::require_read();
            let require_write = T1::require_write()
                || T2::require_write()
                || T3::require_write()
                || T4::require_write()
                || T5::require_write()
                || T6::require_write()
                || T7::require_write()
                || T8::require_write()
                || T9::require_write()
                || T10::require_write()
                || T11::require_write()
                || T12::require_write();

            let entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            for param1 in T1::from_components(&entity, &entity_guard) {
                for param2 in T2::from_components(&entity, &entity_guard) {
                    for param3 in T3::from_components(&entity, &entity_guard) {
                        for param4 in T4::from_components(&entity, &entity_guard) {
                            for param5 in T5::from_components(&entity, &entity_guard) {
                                for param6 in T6::from_components(&entity, &entity_guard) {
                                    for param7 in T7::from_components(&entity, &entity_guard) {
                                        for param8 in T8::from_components(&entity, &entity_guard) {
                                            for param9 in
                                                T9::from_components(&entity, &entity_guard)
                                            {
                                                for param10 in
                                                    T10::from_components(&entity, &entity_guard)
                                                {
                                                    for param11 in
                                                        T11::from_components(&entity, &entity_guard)
                                                    {
                                                        for param12 in T12::from_components(
                                                            &entity,
                                                            &entity_guard,
                                                        ) {
                                                            (self.0)(
                                                                param1.clone(),
                                                                param2.clone(),
                                                                param3.clone(),
                                                                param4.clone(),
                                                                param5.clone(),
                                                                param6.clone(),
                                                                param7.clone(),
                                                                param8.clone(),
                                                                param9.clone(),
                                                                param10.clone(),
                                                                param11.clone(),
                                                                param12.clone(),
                                                            );
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}

/// A shortcut for a boxed inject function
#[derive(Clone)]
pub struct Inject13<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>(
    Arc<dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) + Send + Sync>,
);

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>
    Inject13<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>
{
    /// New instance
    pub fn new(
        val: impl Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) + Send + Sync + 'static,
    ) -> Self {
        Self(Arc::new(val))
    }
}

impl<
        T1: QueryInjectable + 'static,
        T2: QueryInjectable + 'static,
        T3: QueryInjectable + 'static,
        T4: QueryInjectable + 'static,
        T5: QueryInjectable + 'static,
        T6: QueryInjectable + 'static,
        T7: QueryInjectable + 'static,
        T8: QueryInjectable + 'static,
        T9: QueryInjectable + 'static,
        T10: QueryInjectable + 'static,
        T11: QueryInjectable + 'static,
        T12: QueryInjectable + 'static,
        T13: QueryInjectable + 'static,
    > QueryInject for Inject13<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>
{
    fn duplicate(&self) -> Self {
        Self(self.0.clone())
    }

    fn inject(self) -> Box<dyn Fn(&EntityReference) + Send + Sync> {
        Box::new(move |entity| {
            let require_read = T1::require_read()
                || T2::require_read()
                || T3::require_read()
                || T4::require_read()
                || T5::require_read()
                || T6::require_read()
                || T7::require_read()
                || T8::require_read()
                || T9::require_read()
                || T10::require_read()
                || T11::require_read()
                || T12::require_read()
                || T13::require_read();
            let require_write = T1::require_write()
                || T2::require_write()
                || T3::require_write()
                || T4::require_write()
                || T5::require_write()
                || T6::require_write()
                || T7::require_write()
                || T8::require_write()
                || T9::require_write()
                || T10::require_write()
                || T11::require_write()
                || T12::require_write()
                || T13::require_write();

            let entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            for param1 in T1::from_components(&entity, &entity_guard) {
                for param2 in T2::from_components(&entity, &entity_guard) {
                    for param3 in T3::from_components(&entity, &entity_guard) {
                        for param4 in T4::from_components(&entity, &entity_guard) {
                            for param5 in T5::from_components(&entity, &entity_guard) {
                                for param6 in T6::from_components(&entity, &entity_guard) {
                                    for param7 in T7::from_components(&entity, &entity_guard) {
                                        for param8 in T8::from_components(&entity, &entity_guard) {
                                            for param9 in
                                                T9::from_components(&entity, &entity_guard)
                                            {
                                                for param10 in
                                                    T10::from_components(&entity, &entity_guard)
                                                {
                                                    for param11 in
                                                        T11::from_components(&entity, &entity_guard)
                                                    {
                                                        for param12 in T12::from_components(
                                                            &entity,
                                                            &entity_guard,
                                                        ) {
                                                            for param13 in T13::from_components(
                                                                &entity,
                                                                &entity_guard,
                                                            ) {
                                                                (self.0)(
                                                                    param1.clone(),
                                                                    param2.clone(),
                                                                    param3.clone(),
                                                                    param4.clone(),
                                                                    param5.clone(),
                                                                    param6.clone(),
                                                                    param7.clone(),
                                                                    param8.clone(),
                                                                    param9.clone(),
                                                                    param10.clone(),
                                                                    param11.clone(),
                                                                    param12.clone(),
                                                                    param13.clone(),
                                                                );
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}

/// A shortcut for a boxed inject function
#[derive(Clone)]
pub struct Inject14<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>(
    Arc<dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) + Send + Sync>,
);

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>
    Inject14<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>
{
    /// New instance
    pub fn new(
        val: impl Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self(Arc::new(val))
    }
}

impl<
        T1: QueryInjectable + 'static,
        T2: QueryInjectable + 'static,
        T3: QueryInjectable + 'static,
        T4: QueryInjectable + 'static,
        T5: QueryInjectable + 'static,
        T6: QueryInjectable + 'static,
        T7: QueryInjectable + 'static,
        T8: QueryInjectable + 'static,
        T9: QueryInjectable + 'static,
        T10: QueryInjectable + 'static,
        T11: QueryInjectable + 'static,
        T12: QueryInjectable + 'static,
        T13: QueryInjectable + 'static,
        T14: QueryInjectable + 'static,
    > QueryInject for Inject14<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>
{
    fn duplicate(&self) -> Self {
        Self(self.0.clone())
    }

    fn inject(self) -> Box<dyn Fn(&EntityReference) + Send + Sync> {
        Box::new(move |entity| {
            let require_read = T1::require_read()
                || T2::require_read()
                || T3::require_read()
                || T4::require_read()
                || T5::require_read()
                || T6::require_read()
                || T7::require_read()
                || T8::require_read()
                || T9::require_read()
                || T10::require_read()
                || T11::require_read()
                || T12::require_read()
                || T13::require_read()
                || T14::require_read();
            let require_write = T1::require_write()
                || T2::require_write()
                || T3::require_write()
                || T4::require_write()
                || T5::require_write()
                || T6::require_write()
                || T7::require_write()
                || T8::require_write()
                || T9::require_write()
                || T10::require_write()
                || T11::require_write()
                || T12::require_write()
                || T13::require_write()
                || T14::require_write();

            let entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            for param1 in T1::from_components(&entity, &entity_guard) {
                for param2 in T2::from_components(&entity, &entity_guard) {
                    for param3 in T3::from_components(&entity, &entity_guard) {
                        for param4 in T4::from_components(&entity, &entity_guard) {
                            for param5 in T5::from_components(&entity, &entity_guard) {
                                for param6 in T6::from_components(&entity, &entity_guard) {
                                    for param7 in T7::from_components(&entity, &entity_guard) {
                                        for param8 in T8::from_components(&entity, &entity_guard) {
                                            for param9 in
                                                T9::from_components(&entity, &entity_guard)
                                            {
                                                for param10 in
                                                    T10::from_components(&entity, &entity_guard)
                                                {
                                                    for param11 in
                                                        T11::from_components(&entity, &entity_guard)
                                                    {
                                                        for param12 in T12::from_components(
                                                            &entity,
                                                            &entity_guard,
                                                        ) {
                                                            for param13 in T13::from_components(
                                                                &entity,
                                                                &entity_guard,
                                                            ) {
                                                                for param14 in T14::from_components(
                                                                    &entity,
                                                                    &entity_guard,
                                                                ) {
                                                                    (self.0)(
                                                                        param1.clone(),
                                                                        param2.clone(),
                                                                        param3.clone(),
                                                                        param4.clone(),
                                                                        param5.clone(),
                                                                        param6.clone(),
                                                                        param7.clone(),
                                                                        param8.clone(),
                                                                        param9.clone(),
                                                                        param10.clone(),
                                                                        param11.clone(),
                                                                        param12.clone(),
                                                                        param13.clone(),
                                                                        param14.clone(),
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}

/// A shortcut for a boxed inject function
#[derive(Clone)]
pub struct Inject15<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>(
    Arc<dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) + Send + Sync>,
);

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>
    Inject15<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>
{
    /// New instance
    pub fn new(
        val: impl Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self(Arc::new(val))
    }
}

impl<
        T1: QueryInjectable + 'static,
        T2: QueryInjectable + 'static,
        T3: QueryInjectable + 'static,
        T4: QueryInjectable + 'static,
        T5: QueryInjectable + 'static,
        T6: QueryInjectable + 'static,
        T7: QueryInjectable + 'static,
        T8: QueryInjectable + 'static,
        T9: QueryInjectable + 'static,
        T10: QueryInjectable + 'static,
        T11: QueryInjectable + 'static,
        T12: QueryInjectable + 'static,
        T13: QueryInjectable + 'static,
        T14: QueryInjectable + 'static,
        T15: QueryInjectable + 'static,
    > QueryInject for Inject15<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>
{
    fn duplicate(&self) -> Self {
        Self(self.0.clone())
    }

    fn inject(self) -> Box<dyn Fn(&EntityReference) + Send + Sync> {
        Box::new(move |entity| {
            let require_read = T1::require_read()
                || T2::require_read()
                || T3::require_read()
                || T4::require_read()
                || T5::require_read()
                || T6::require_read()
                || T7::require_read()
                || T8::require_read()
                || T9::require_read()
                || T10::require_read()
                || T11::require_read()
                || T12::require_read()
                || T13::require_read()
                || T14::require_read()
                || T15::require_read();
            let require_write = T1::require_write()
                || T2::require_write()
                || T3::require_write()
                || T4::require_write()
                || T5::require_write()
                || T6::require_write()
                || T7::require_write()
                || T8::require_write()
                || T9::require_write()
                || T10::require_write()
                || T11::require_write()
                || T12::require_write()
                || T13::require_write()
                || T14::require_write()
                || T15::require_write();

            let entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            for param1 in T1::from_components(&entity, &entity_guard) {
                for param2 in T2::from_components(&entity, &entity_guard) {
                    for param3 in T3::from_components(&entity, &entity_guard) {
                        for param4 in T4::from_components(&entity, &entity_guard) {
                            for param5 in T5::from_components(&entity, &entity_guard) {
                                for param6 in T6::from_components(&entity, &entity_guard) {
                                    for param7 in T7::from_components(&entity, &entity_guard) {
                                        for param8 in T8::from_components(&entity, &entity_guard) {
                                            for param9 in
                                                T9::from_components(&entity, &entity_guard)
                                            {
                                                for param10 in
                                                    T10::from_components(&entity, &entity_guard)
                                                {
                                                    for param11 in
                                                        T11::from_components(&entity, &entity_guard)
                                                    {
                                                        for param12 in T12::from_components(
                                                            &entity,
                                                            &entity_guard,
                                                        ) {
                                                            for param13 in T13::from_components(
                                                                &entity,
                                                                &entity_guard,
                                                            ) {
                                                                for param14 in T14::from_components(
                                                                    &entity,
                                                                    &entity_guard,
                                                                ) {
                                                                    for param15 in
                                                                        T15::from_components(
                                                                            &entity,
                                                                            &entity_guard,
                                                                        )
                                                                    {
                                                                        (self.0)(
                                                                            param1.clone(),
                                                                            param2.clone(),
                                                                            param3.clone(),
                                                                            param4.clone(),
                                                                            param5.clone(),
                                                                            param6.clone(),
                                                                            param7.clone(),
                                                                            param8.clone(),
                                                                            param9.clone(),
                                                                            param10.clone(),
                                                                            param11.clone(),
                                                                            param12.clone(),
                                                                            param13.clone(),
                                                                            param14.clone(),
                                                                            param15.clone(),
                                                                        );
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}

/// A shortcut for a boxed inject function
#[derive(Clone)]
pub struct Inject16<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16>(
    Arc<
        dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) + Send + Sync,
    >,
);

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16>
    Inject16<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16>
{
    /// New instance
    pub fn new(
        val: impl Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self(Arc::new(val))
    }
}

impl<
        T1: QueryInjectable + 'static,
        T2: QueryInjectable + 'static,
        T3: QueryInjectable + 'static,
        T4: QueryInjectable + 'static,
        T5: QueryInjectable + 'static,
        T6: QueryInjectable + 'static,
        T7: QueryInjectable + 'static,
        T8: QueryInjectable + 'static,
        T9: QueryInjectable + 'static,
        T10: QueryInjectable + 'static,
        T11: QueryInjectable + 'static,
        T12: QueryInjectable + 'static,
        T13: QueryInjectable + 'static,
        T14: QueryInjectable + 'static,
        T15: QueryInjectable + 'static,
        T16: QueryInjectable + 'static,
    > QueryInject
    for Inject16<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16>
{
    fn duplicate(&self) -> Self {
        Self(self.0.clone())
    }

    fn inject(self) -> Box<dyn Fn(&EntityReference) + Send + Sync> {
        Box::new(move |entity| {
            let require_read = T1::require_read()
                || T2::require_read()
                || T3::require_read()
                || T4::require_read()
                || T5::require_read()
                || T6::require_read()
                || T7::require_read()
                || T8::require_read()
                || T9::require_read()
                || T10::require_read()
                || T11::require_read()
                || T12::require_read()
                || T13::require_read()
                || T14::require_read()
                || T15::require_read()
                || T16::require_read();
            let require_write = T1::require_write()
                || T2::require_write()
                || T3::require_write()
                || T4::require_write()
                || T5::require_write()
                || T6::require_write()
                || T7::require_write()
                || T8::require_write()
                || T9::require_write()
                || T10::require_write()
                || T11::require_write()
                || T12::require_write()
                || T13::require_write()
                || T14::require_write()
                || T15::require_write()
                || T16::require_write();

            let entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            for param1 in T1::from_components(&entity, &entity_guard) {
                for param2 in T2::from_components(&entity, &entity_guard) {
                    for param3 in T3::from_components(&entity, &entity_guard) {
                        for param4 in T4::from_components(&entity, &entity_guard) {
                            for param5 in T5::from_components(&entity, &entity_guard) {
                                for param6 in T6::from_components(&entity, &entity_guard) {
                                    for param7 in T7::from_components(&entity, &entity_guard) {
                                        for param8 in T8::from_components(&entity, &entity_guard) {
                                            for param9 in
                                                T9::from_components(&entity, &entity_guard)
                                            {
                                                for param10 in
                                                    T10::from_components(&entity, &entity_guard)
                                                {
                                                    for param11 in
                                                        T11::from_components(&entity, &entity_guard)
                                                    {
                                                        for param12 in T12::from_components(
                                                            &entity,
                                                            &entity_guard,
                                                        ) {
                                                            for param13 in T13::from_components(
                                                                &entity,
                                                                &entity_guard,
                                                            ) {
                                                                for param14 in T14::from_components(
                                                                    &entity,
                                                                    &entity_guard,
                                                                ) {
                                                                    for param15 in
                                                                        T15::from_components(
                                                                            &entity,
                                                                            &entity_guard,
                                                                        )
                                                                    {
                                                                        for param16 in
                                                                            T16::from_components(
                                                                                &entity,
                                                                                &entity_guard,
                                                                            )
                                                                        {
                                                                            (self.0)(
                                                                                param1.clone(),
                                                                                param2.clone(),
                                                                                param3.clone(),
                                                                                param4.clone(),
                                                                                param5.clone(),
                                                                                param6.clone(),
                                                                                param7.clone(),
                                                                                param8.clone(),
                                                                                param9.clone(),
                                                                                param10.clone(),
                                                                                param11.clone(),
                                                                                param12.clone(),
                                                                                param13.clone(),
                                                                                param14.clone(),
                                                                                param15.clone(),
                                                                                param16.clone(),
                                                                            );
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}

/// A shortcut for a boxed inject function
#[derive(Clone)]
pub struct Inject17<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17>(
    Arc<
        dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
            + Send
            + Sync,
    >,
);

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17>
    Inject17<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17>
{
    /// New instance
    pub fn new(
        val: impl Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self(Arc::new(val))
    }
}

impl<
        T1: QueryInjectable + 'static,
        T2: QueryInjectable + 'static,
        T3: QueryInjectable + 'static,
        T4: QueryInjectable + 'static,
        T5: QueryInjectable + 'static,
        T6: QueryInjectable + 'static,
        T7: QueryInjectable + 'static,
        T8: QueryInjectable + 'static,
        T9: QueryInjectable + 'static,
        T10: QueryInjectable + 'static,
        T11: QueryInjectable + 'static,
        T12: QueryInjectable + 'static,
        T13: QueryInjectable + 'static,
        T14: QueryInjectable + 'static,
        T15: QueryInjectable + 'static,
        T16: QueryInjectable + 'static,
        T17: QueryInjectable + 'static,
    > QueryInject
    for Inject17<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17>
{
    fn duplicate(&self) -> Self {
        Self(self.0.clone())
    }

    fn inject(self) -> Box<dyn Fn(&EntityReference) + Send + Sync> {
        Box::new(move |entity| {
            let require_read = T1::require_read()
                || T2::require_read()
                || T3::require_read()
                || T4::require_read()
                || T5::require_read()
                || T6::require_read()
                || T7::require_read()
                || T8::require_read()
                || T9::require_read()
                || T10::require_read()
                || T11::require_read()
                || T12::require_read()
                || T13::require_read()
                || T14::require_read()
                || T15::require_read()
                || T16::require_read()
                || T17::require_read();
            let require_write = T1::require_write()
                || T2::require_write()
                || T3::require_write()
                || T4::require_write()
                || T5::require_write()
                || T6::require_write()
                || T7::require_write()
                || T8::require_write()
                || T9::require_write()
                || T10::require_write()
                || T11::require_write()
                || T12::require_write()
                || T13::require_write()
                || T14::require_write()
                || T15::require_write()
                || T16::require_write()
                || T17::require_write();

            let entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            for param1 in T1::from_components(&entity, &entity_guard) {
                for param2 in T2::from_components(&entity, &entity_guard) {
                    for param3 in T3::from_components(&entity, &entity_guard) {
                        for param4 in T4::from_components(&entity, &entity_guard) {
                            for param5 in T5::from_components(&entity, &entity_guard) {
                                for param6 in T6::from_components(&entity, &entity_guard) {
                                    for param7 in T7::from_components(&entity, &entity_guard) {
                                        for param8 in T8::from_components(&entity, &entity_guard) {
                                            for param9 in
                                                T9::from_components(&entity, &entity_guard)
                                            {
                                                for param10 in
                                                    T10::from_components(&entity, &entity_guard)
                                                {
                                                    for param11 in
                                                        T11::from_components(&entity, &entity_guard)
                                                    {
                                                        for param12 in T12::from_components(
                                                            &entity,
                                                            &entity_guard,
                                                        ) {
                                                            for param13 in T13::from_components(
                                                                &entity,
                                                                &entity_guard,
                                                            ) {
                                                                for param14 in T14::from_components(
                                                                    &entity,
                                                                    &entity_guard,
                                                                ) {
                                                                    for param15 in
                                                                        T15::from_components(
                                                                            &entity,
                                                                            &entity_guard,
                                                                        )
                                                                    {
                                                                        for param16 in
                                                                            T16::from_components(
                                                                                &entity,
                                                                                &entity_guard,
                                                                            )
                                                                        {
                                                                            for param17 in
                                                                                T17::from_components(
                                                                                    &entity,
                                                                                    &entity_guard,
                                                                                )
                                                                            {
                                                                                (self.0)(
                                                                                    param1.clone(),
                                                                                    param2.clone(),
                                                                                    param3.clone(),
                                                                                    param4.clone(),
                                                                                    param5.clone(),
                                                                                    param6.clone(),
                                                                                    param7.clone(),
                                                                                    param8.clone(),
                                                                                    param9.clone(),
                                                                                    param10.clone(),
                                                                                    param11.clone(),
                                                                                    param12.clone(),
                                                                                    param13.clone(),
                                                                                    param14.clone(),
                                                                                    param15.clone(),
                                                                                    param16.clone(),
                                                                                    param17.clone(),
                                                                                );
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}

/// A shortcut for a boxed inject function
#[derive(Clone)]
pub struct Inject18<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18>(
    Arc<
        dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
            + Send
            + Sync,
    >,
);

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18>
    Inject18<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18>
{
    /// New instance
    pub fn new(
        val: impl Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self(Arc::new(val))
    }
}

impl<
        T1: QueryInjectable + 'static,
        T2: QueryInjectable + 'static,
        T3: QueryInjectable + 'static,
        T4: QueryInjectable + 'static,
        T5: QueryInjectable + 'static,
        T6: QueryInjectable + 'static,
        T7: QueryInjectable + 'static,
        T8: QueryInjectable + 'static,
        T9: QueryInjectable + 'static,
        T10: QueryInjectable + 'static,
        T11: QueryInjectable + 'static,
        T12: QueryInjectable + 'static,
        T13: QueryInjectable + 'static,
        T14: QueryInjectable + 'static,
        T15: QueryInjectable + 'static,
        T16: QueryInjectable + 'static,
        T17: QueryInjectable + 'static,
        T18: QueryInjectable + 'static,
    > QueryInject
    for Inject18<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18>
{
    fn duplicate(&self) -> Self {
        Self(self.0.clone())
    }

    fn inject(self) -> Box<dyn Fn(&EntityReference) + Send + Sync> {
        Box::new(move |entity| {
            let require_read = T1::require_read()
                || T2::require_read()
                || T3::require_read()
                || T4::require_read()
                || T5::require_read()
                || T6::require_read()
                || T7::require_read()
                || T8::require_read()
                || T9::require_read()
                || T10::require_read()
                || T11::require_read()
                || T12::require_read()
                || T13::require_read()
                || T14::require_read()
                || T15::require_read()
                || T16::require_read()
                || T17::require_read()
                || T18::require_read();
            let require_write = T1::require_write()
                || T2::require_write()
                || T3::require_write()
                || T4::require_write()
                || T5::require_write()
                || T6::require_write()
                || T7::require_write()
                || T8::require_write()
                || T9::require_write()
                || T10::require_write()
                || T11::require_write()
                || T12::require_write()
                || T13::require_write()
                || T14::require_write()
                || T15::require_write()
                || T16::require_write()
                || T17::require_write()
                || T18::require_write();

            let entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            for param1 in T1::from_components(&entity, &entity_guard) {
                for param2 in T2::from_components(&entity, &entity_guard) {
                    for param3 in T3::from_components(&entity, &entity_guard) {
                        for param4 in T4::from_components(&entity, &entity_guard) {
                            for param5 in T5::from_components(&entity, &entity_guard) {
                                for param6 in T6::from_components(&entity, &entity_guard) {
                                    for param7 in T7::from_components(&entity, &entity_guard) {
                                        for param8 in T8::from_components(&entity, &entity_guard) {
                                            for param9 in
                                                T9::from_components(&entity, &entity_guard)
                                            {
                                                for param10 in
                                                    T10::from_components(&entity, &entity_guard)
                                                {
                                                    for param11 in
                                                        T11::from_components(&entity, &entity_guard)
                                                    {
                                                        for param12 in T12::from_components(
                                                            &entity,
                                                            &entity_guard,
                                                        ) {
                                                            for param13 in T13::from_components(
                                                                &entity,
                                                                &entity_guard,
                                                            ) {
                                                                for param14 in T14::from_components(
                                                                    &entity,
                                                                    &entity_guard,
                                                                ) {
                                                                    for param15 in
                                                                        T15::from_components(
                                                                            &entity,
                                                                            &entity_guard,
                                                                        )
                                                                    {
                                                                        for param16 in
                                                                            T16::from_components(
                                                                                &entity,
                                                                                &entity_guard,
                                                                            )
                                                                        {
                                                                            for param17 in
                                                                                T17::from_components(
                                                                                    &entity,
                                                                                    &entity_guard,
                                                                                )
                                                                            {
                                                                                for param18 in T18::from_components(&entity, &entity_guard) {
                                                                                    (self.0)(param1.clone(), param2.clone(), param3.clone(), param4.clone(), param5.clone(), param6.clone(), param7.clone(), param8.clone(), param9.clone(), param10.clone(), param11.clone(), param12.clone(), param13.clone(), param14.clone(), param15.clone(), param16.clone(), param17.clone(), param18.clone());
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}

/// A shortcut for a boxed inject function
#[derive(Clone)]
pub struct Inject19<
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11,
    T12,
    T13,
    T14,
    T15,
    T16,
    T17,
    T18,
    T19,
>(
    Arc<
        dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
            + Send
            + Sync,
    >,
);

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19>
    Inject19<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19>
{
    /// New instance
    pub fn new(
        val: impl Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self(Arc::new(val))
    }
}

impl<
        T1: QueryInjectable + 'static,
        T2: QueryInjectable + 'static,
        T3: QueryInjectable + 'static,
        T4: QueryInjectable + 'static,
        T5: QueryInjectable + 'static,
        T6: QueryInjectable + 'static,
        T7: QueryInjectable + 'static,
        T8: QueryInjectable + 'static,
        T9: QueryInjectable + 'static,
        T10: QueryInjectable + 'static,
        T11: QueryInjectable + 'static,
        T12: QueryInjectable + 'static,
        T13: QueryInjectable + 'static,
        T14: QueryInjectable + 'static,
        T15: QueryInjectable + 'static,
        T16: QueryInjectable + 'static,
        T17: QueryInjectable + 'static,
        T18: QueryInjectable + 'static,
        T19: QueryInjectable + 'static,
    > QueryInject
    for Inject19<
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
        T16,
        T17,
        T18,
        T19,
    >
{
    fn duplicate(&self) -> Self {
        Self(self.0.clone())
    }

    fn inject(self) -> Box<dyn Fn(&EntityReference) + Send + Sync> {
        Box::new(move |entity| {
            let require_read = T1::require_read()
                || T2::require_read()
                || T3::require_read()
                || T4::require_read()
                || T5::require_read()
                || T6::require_read()
                || T7::require_read()
                || T8::require_read()
                || T9::require_read()
                || T10::require_read()
                || T11::require_read()
                || T12::require_read()
                || T13::require_read()
                || T14::require_read()
                || T15::require_read()
                || T16::require_read()
                || T17::require_read()
                || T18::require_read()
                || T19::require_read();
            let require_write = T1::require_write()
                || T2::require_write()
                || T3::require_write()
                || T4::require_write()
                || T5::require_write()
                || T6::require_write()
                || T7::require_write()
                || T8::require_write()
                || T9::require_write()
                || T10::require_write()
                || T11::require_write()
                || T12::require_write()
                || T13::require_write()
                || T14::require_write()
                || T15::require_write()
                || T16::require_write()
                || T17::require_write()
                || T18::require_write()
                || T19::require_write();

            let entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            for param1 in T1::from_components(&entity, &entity_guard) {
                for param2 in T2::from_components(&entity, &entity_guard) {
                    for param3 in T3::from_components(&entity, &entity_guard) {
                        for param4 in T4::from_components(&entity, &entity_guard) {
                            for param5 in T5::from_components(&entity, &entity_guard) {
                                for param6 in T6::from_components(&entity, &entity_guard) {
                                    for param7 in T7::from_components(&entity, &entity_guard) {
                                        for param8 in T8::from_components(&entity, &entity_guard) {
                                            for param9 in
                                                T9::from_components(&entity, &entity_guard)
                                            {
                                                for param10 in
                                                    T10::from_components(&entity, &entity_guard)
                                                {
                                                    for param11 in
                                                        T11::from_components(&entity, &entity_guard)
                                                    {
                                                        for param12 in T12::from_components(
                                                            &entity,
                                                            &entity_guard,
                                                        ) {
                                                            for param13 in T13::from_components(
                                                                &entity,
                                                                &entity_guard,
                                                            ) {
                                                                for param14 in T14::from_components(
                                                                    &entity,
                                                                    &entity_guard,
                                                                ) {
                                                                    for param15 in
                                                                        T15::from_components(
                                                                            &entity,
                                                                            &entity_guard,
                                                                        )
                                                                    {
                                                                        for param16 in
                                                                            T16::from_components(
                                                                                &entity,
                                                                                &entity_guard,
                                                                            )
                                                                        {
                                                                            for param17 in
                                                                                T17::from_components(
                                                                                    &entity,
                                                                                    &entity_guard,
                                                                                )
                                                                            {
                                                                                for param18 in T18::from_components(&entity, &entity_guard) {
                                                                                    for param19 in T19::from_components(&entity, &entity_guard) {
                                                                                        (self.0)(param1.clone(), param2.clone(), param3.clone(), param4.clone(), param5.clone(), param6.clone(), param7.clone(), param8.clone(), param9.clone(), param10.clone(), param11.clone(), param12.clone(), param13.clone(), param14.clone(), param15.clone(), param16.clone(), param17.clone(), param18.clone(), param19.clone());
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}

/// A shortcut for a boxed inject function
#[derive(Clone)]
pub struct Inject20<
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11,
    T12,
    T13,
    T14,
    T15,
    T16,
    T17,
    T18,
    T19,
    T20,
>(
    Arc<
        dyn Fn(
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
            ) + Send
            + Sync,
    >,
);

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20>
    Inject20<
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
        T16,
        T17,
        T18,
        T19,
        T20,
    >
{
    /// New instance
    pub fn new(
        val: impl Fn(
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
            ) + Send
            + Sync
            + 'static,
    ) -> Self {
        Self(Arc::new(val))
    }
}

impl<
        T1: QueryInjectable + 'static,
        T2: QueryInjectable + 'static,
        T3: QueryInjectable + 'static,
        T4: QueryInjectable + 'static,
        T5: QueryInjectable + 'static,
        T6: QueryInjectable + 'static,
        T7: QueryInjectable + 'static,
        T8: QueryInjectable + 'static,
        T9: QueryInjectable + 'static,
        T10: QueryInjectable + 'static,
        T11: QueryInjectable + 'static,
        T12: QueryInjectable + 'static,
        T13: QueryInjectable + 'static,
        T14: QueryInjectable + 'static,
        T15: QueryInjectable + 'static,
        T16: QueryInjectable + 'static,
        T17: QueryInjectable + 'static,
        T18: QueryInjectable + 'static,
        T19: QueryInjectable + 'static,
        T20: QueryInjectable + 'static,
    > QueryInject
    for Inject20<
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
        T16,
        T17,
        T18,
        T19,
        T20,
    >
{
    fn duplicate(&self) -> Self {
        Self(self.0.clone())
    }

    fn inject(self) -> Box<dyn Fn(&EntityReference) + Send + Sync> {
        Box::new(move |entity| {
            let require_read = T1::require_read()
                || T2::require_read()
                || T3::require_read()
                || T4::require_read()
                || T5::require_read()
                || T6::require_read()
                || T7::require_read()
                || T8::require_read()
                || T9::require_read()
                || T10::require_read()
                || T11::require_read()
                || T12::require_read()
                || T13::require_read()
                || T14::require_read()
                || T15::require_read()
                || T16::require_read()
                || T17::require_read()
                || T18::require_read()
                || T19::require_read()
                || T20::require_read();
            let require_write = T1::require_write()
                || T2::require_write()
                || T3::require_write()
                || T4::require_write()
                || T5::require_write()
                || T6::require_write()
                || T7::require_write()
                || T8::require_write()
                || T9::require_write()
                || T10::require_write()
                || T11::require_write()
                || T12::require_write()
                || T13::require_write()
                || T14::require_write()
                || T15::require_write()
                || T16::require_write()
                || T17::require_write()
                || T18::require_write()
                || T19::require_write()
                || T20::require_write();

            let entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            for param1 in T1::from_components(&entity, &entity_guard) {
                for param2 in T2::from_components(&entity, &entity_guard) {
                    for param3 in T3::from_components(&entity, &entity_guard) {
                        for param4 in T4::from_components(&entity, &entity_guard) {
                            for param5 in T5::from_components(&entity, &entity_guard) {
                                for param6 in T6::from_components(&entity, &entity_guard) {
                                    for param7 in T7::from_components(&entity, &entity_guard) {
                                        for param8 in T8::from_components(&entity, &entity_guard) {
                                            for param9 in
                                                T9::from_components(&entity, &entity_guard)
                                            {
                                                for param10 in
                                                    T10::from_components(&entity, &entity_guard)
                                                {
                                                    for param11 in
                                                        T11::from_components(&entity, &entity_guard)
                                                    {
                                                        for param12 in T12::from_components(
                                                            &entity,
                                                            &entity_guard,
                                                        ) {
                                                            for param13 in T13::from_components(
                                                                &entity,
                                                                &entity_guard,
                                                            ) {
                                                                for param14 in T14::from_components(
                                                                    &entity,
                                                                    &entity_guard,
                                                                ) {
                                                                    for param15 in
                                                                        T15::from_components(
                                                                            &entity,
                                                                            &entity_guard,
                                                                        )
                                                                    {
                                                                        for param16 in
                                                                            T16::from_components(
                                                                                &entity,
                                                                                &entity_guard,
                                                                            )
                                                                        {
                                                                            for param17 in
                                                                                T17::from_components(
                                                                                    &entity,
                                                                                    &entity_guard,
                                                                                )
                                                                            {
                                                                                for param18 in T18::from_components(&entity, &entity_guard) {
                                                                                    for param19 in T19::from_components(&entity, &entity_guard) {
                                                                                        for param20 in T20::from_components(&entity, &entity_guard) {
                                                                                            (self.0)(param1.clone(), param2.clone(), param3.clone(), param4.clone(), param5.clone(), param6.clone(), param7.clone(), param8.clone(), param9.clone(), param10.clone(), param11.clone(), param12.clone(), param13.clone(), param14.clone(), param15.clone(), param16.clone(), param17.clone(), param18.clone(), param19.clone(), param20.clone());
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}
