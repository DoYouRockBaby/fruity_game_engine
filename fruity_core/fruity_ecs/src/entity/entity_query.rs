use crate::component::component::Component;
use crate::component::component::StaticComponent;
use crate::entity::entity::EntityId;
use crate::entity::entity::EntityTypeIdentifier;
use crate::entity::entity_guard::EntityReadGuard;
use crate::entity::entity_guard::EntityWriteGuard;
use crate::entity::entity_reference::EntityReference;
use itertools::iproduct;
use std::sync::Arc;

#[macro_export]
macro_rules! query_params {
    ($fn:expr,$entity:expr,$requested_entity_guard:expr, $($injection_type:ident),*) => {
        #[allow(unused_parens)]
        for ($ ($injection_type),*) in iproduct!(
            $ ($injection_type::from_components(
                $entity,
                $requested_entity_guard,
            )),*
        ) {
            $fn($ ($injection_type),*);
        }
    };
    ($fn:expr) => {
        query
    };
}

/// An enum to pass a guard into the [’QueryInjectable’]
pub enum RequestedEntityGuard<'a> {
    /// No guard required
    None,
    /// Read guard required
    Read(EntityReadGuard<'a>),
    /// Write guard required
    Write(EntityWriteGuard<'a>),
}

/// A trait for types that can be exposed from components references
pub trait QueryInjectable: Sized {
    /// Does this require a read guard over the reference
    fn require_read() -> bool;

    /// Does this require a write guard over the reference
    fn require_write() -> bool;

    /// Get the object
    fn from_components(
        entity: &EntityReference,
        entity_guard: &mut RequestedEntityGuard,
    ) -> Vec<Self>;
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
        _entity_guard: &mut RequestedEntityGuard,
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
        entity_guard: &mut RequestedEntityGuard,
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
        entity_guard: &mut RequestedEntityGuard,
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
        entity_guard: &mut RequestedEntityGuard,
    ) -> Vec<Self> {
        match entity_guard {
            RequestedEntityGuard::Read(guard) => vec![guard.is_enabled()],
            RequestedEntityGuard::Write(guard) => vec![guard.is_enabled()],
            RequestedEntityGuard::None => panic!(),
        }
    }
}

impl<'a, T: Component + StaticComponent> QueryInjectable for &T {
    fn require_read() -> bool {
        true
    }

    fn require_write() -> bool {
        false
    }

    fn from_components(
        _entity: &EntityReference,
        entity_guard: &mut RequestedEntityGuard,
    ) -> Vec<Self> {
        let identifier = T::get_component_name();
        let components = match entity_guard {
            RequestedEntityGuard::Read(guard) => guard.read_typed_components::<T>(&identifier),
            RequestedEntityGuard::Write(guard) => guard.read_typed_components::<T>(&identifier),
            RequestedEntityGuard::None => panic!(),
        };

        components
            .into_iter()
            .map(|component| {
                // TODO: Find a way to remove it
                unsafe { std::mem::transmute::<&T, &T>(component) }
            })
            .collect::<Vec<_>>()
    }
}

impl<'a, T: Component + StaticComponent> QueryInjectable for &mut T {
    fn require_read() -> bool {
        false
    }

    fn require_write() -> bool {
        true
    }

    fn from_components(
        _entity: &EntityReference,
        entity_guard: &mut RequestedEntityGuard,
    ) -> Vec<Self> {
        let identifier = T::get_component_name();
        let components = match entity_guard {
            RequestedEntityGuard::Read(guard) => panic!(),
            RequestedEntityGuard::Write(guard) => guard.write_typed_components::<T>(&identifier),
            RequestedEntityGuard::None => panic!(),
        };

        components
            .into_iter()
            .map(|component| {
                // TODO: Find a way to remove it
                unsafe { std::mem::transmute::<&mut T, &mut T>(component) }
            })
            .collect::<Vec<_>>()
    }
}

/// A trait that is implemented by functions that supports dependency injection
pub trait QueryInject: Send + Sync {
    /// Duplicate the query injector
    fn duplicate(&self) -> Self;

    /// Get a function that proceed the injection
    fn inject(
        self,
        request_identifier: &EntityTypeIdentifier,
    ) -> Box<dyn Fn(EntityReference) + Send + Sync>;
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

    fn inject(
        self,
        _request_identifier: &EntityTypeIdentifier,
    ) -> Box<dyn Fn(EntityReference) + Send + Sync> {
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

    fn inject(
        self,
        request_identifier: &EntityTypeIdentifier,
    ) -> Box<dyn Fn(EntityReference) + Send + Sync> {
        let request_identifier = request_identifier.clone();
        Box::new(move |entity| {
            let require_read = T1::require_read();
            let require_write = T1::require_write();

            let mut requested_entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            query_params!(self.0, &entity, &mut requested_entity_guard, T1);
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

    fn inject(
        self,
        request_identifier: &EntityTypeIdentifier,
    ) -> Box<dyn Fn(EntityReference) + Send + Sync> {
        let request_identifier = request_identifier.clone();
        Box::new(move |entity| {
            let require_read = T1::require_read() || T2::require_read();
            let require_write = T1::require_write() || T2::require_write();

            let mut requested_entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            query_params!(self.0, &entity, &mut requested_entity_guard, T1, T2);
        })
    }
}

/*


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

    fn inject(
        self,
        request_identifier: &EntityTypeIdentifier,
    ) -> Box<dyn Fn(EntityReference) + Send + Sync> {
        let request_identifier = request_identifier.clone();
        Box::new(move |entity| {
            let require_read = T1::require_read() || T2::require_read() || T3::require_read();
            let require_write = T1::require_write() || T2::require_write() || T3::require_write();

            let mut requested_entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            let mut request_identifier = request_identifier.clone();
            query_params!(
                self.0,
                &entity,
                &mut requested_entity_guard,
                &mut request_identifier,
                T1,
                T2,
                T3
            );
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
        'a,
        T1: QueryInjectable + 'static,
        T2: QueryInjectable + 'static,
        T3: QueryInjectable + 'static,
        T4: QueryInjectable + 'static,
    > QueryInject for Inject4<T1, T2, T3, T4>
{
    fn duplicate(&self) -> Self {
        Self(self.0.clone())
    }

    fn inject(
        self,
        request_identifier: &EntityTypeIdentifier,
    ) -> Box<dyn Fn(EntityReference) + Send + Sync> {
        let request_identifier = request_identifier.clone();
        Box::new(move |entity| {
            let require_read = T1::require_read()
                || T2::require_read()
                || T3::require_read()
                || T4::require_read();
            let require_write = T1::require_write()
                || T2::require_write()
                || T3::require_write()
                || T4::require_write();

            let mut requested_entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            let mut request_identifier = request_identifier.clone();
            query_params!(
                self.0,
                &entity,
                &mut requested_entity_guard,
                &mut request_identifier,
                T1,
                T2,
                T3,
                T4
            );
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
        'a,
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

    fn inject(
        self,
        request_identifier: &EntityTypeIdentifier,
    ) -> Box<dyn Fn(EntityReference) + Send + Sync> {
        let request_identifier = request_identifier.clone();
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

            let mut requested_entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            let mut request_identifier = request_identifier.clone();
            query_params!(
                self.0,
                &entity,
                &mut requested_entity_guard,
                &mut request_identifier,
                T1,
                T2,
                T3,
                T4,
                T5
            );
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
        'a,
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

    fn inject(
        self,
        request_identifier: &EntityTypeIdentifier,
    ) -> Box<dyn Fn(EntityReference) + Send + Sync> {
        let request_identifier = request_identifier.clone();
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

            let mut requested_entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            let mut request_identifier = request_identifier.clone();
            query_params!(
                self.0,
                &entity,
                &mut requested_entity_guard,
                &mut request_identifier,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6
            );
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
        'a,
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

    fn inject(
        self,
        request_identifier: &EntityTypeIdentifier,
    ) -> Box<dyn Fn(EntityReference) + Send + Sync> {
        let request_identifier = request_identifier.clone();
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

            let mut requested_entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            let mut request_identifier = request_identifier.clone();
            query_params!(
                self.0,
                &entity,
                &mut requested_entity_guard,
                &mut request_identifier,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7
            );
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

    fn inject(
        self,
        request_identifier: &EntityTypeIdentifier,
    ) -> Box<dyn Fn(EntityReference) + Send + Sync> {
        let request_identifier = request_identifier.clone();
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

            let mut requested_entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            let mut request_identifier = request_identifier.clone();
            query_params!(
                self.0,
                &entity,
                &mut requested_entity_guard,
                &mut request_identifier,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8
            );
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

    fn inject(
        self,
        request_identifier: &EntityTypeIdentifier,
    ) -> Box<dyn Fn(EntityReference) + Send + Sync> {
        let request_identifier = request_identifier.clone();
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

            let mut requested_entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            let mut request_identifier = request_identifier.clone();
            query_params!(
                self.0,
                &entity,
                &mut requested_entity_guard,
                &mut request_identifier,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9
            );
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

    fn inject(
        self,
        request_identifier: &EntityTypeIdentifier,
    ) -> Box<dyn Fn(EntityReference) + Send + Sync> {
        let request_identifier = request_identifier.clone();
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

            let mut requested_entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            let mut request_identifier = request_identifier.clone();
            query_params!(
                self.0,
                &entity,
                &mut requested_entity_guard,
                &mut request_identifier,
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10
            );
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

    fn inject(
        self,
        request_identifier: &EntityTypeIdentifier,
    ) -> Box<dyn Fn(EntityReference) + Send + Sync> {
        let request_identifier = request_identifier.clone();
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

            let mut requested_entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            let mut request_identifier = request_identifier.clone();
            query_params!(
                self.0,
                &entity,
                &mut requested_entity_guard,
                &mut request_identifier,
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
                T11
            );
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

    fn inject(
        self,
        request_identifier: &EntityTypeIdentifier,
    ) -> Box<dyn Fn(EntityReference) + Send + Sync> {
        let request_identifier = request_identifier.clone();
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

            let mut requested_entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            let mut request_identifier = request_identifier.clone();
            query_params!(
                self.0,
                &entity,
                &mut requested_entity_guard,
                &mut request_identifier,
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
                T12
            );
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

    fn inject(
        self,
        request_identifier: &EntityTypeIdentifier,
    ) -> Box<dyn Fn(EntityReference) + Send + Sync> {
        let request_identifier = request_identifier.clone();
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

            let mut requested_entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            let mut request_identifier = request_identifier.clone();
            query_params!(
                self.0,
                &entity,
                &mut requested_entity_guard,
                &mut request_identifier,
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
                T13
            );
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

    fn inject(
        self,
        request_identifier: &EntityTypeIdentifier,
    ) -> Box<dyn Fn(EntityReference) + Send + Sync> {
        let request_identifier = request_identifier.clone();
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

            let mut requested_entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            let mut request_identifier = request_identifier.clone();
            query_params!(
                self.0,
                &entity,
                &mut requested_entity_guard,
                &mut request_identifier,
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
                T14
            );
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

    fn inject(
        self,
        request_identifier: &EntityTypeIdentifier,
    ) -> Box<dyn Fn(EntityReference) + Send + Sync> {
        let request_identifier = request_identifier.clone();
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

            let mut requested_entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            let mut request_identifier = request_identifier.clone();
            query_params!(
                self.0,
                &entity,
                &mut requested_entity_guard,
                &mut request_identifier,
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
                T15
            );
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

    fn inject(
        self,
        request_identifier: &EntityTypeIdentifier,
    ) -> Box<dyn Fn(EntityReference) + Send + Sync> {
        let request_identifier = request_identifier.clone();
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

            let mut requested_entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            let mut request_identifier = request_identifier.clone();
            query_params!(
                self.0,
                &entity,
                &mut requested_entity_guard,
                &mut request_identifier,
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
                T16
            );
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

    fn inject(
        self,
        request_identifier: &EntityTypeIdentifier,
    ) -> Box<dyn Fn(EntityReference) + Send + Sync> {
        let request_identifier = request_identifier.clone();
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

            let mut requested_entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            let mut request_identifier = request_identifier.clone();
            query_params!(
                self.0,
                &entity,
                &mut requested_entity_guard,
                &mut request_identifier,
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
                T17
            );
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

    fn inject(
        self,
        request_identifier: &EntityTypeIdentifier,
    ) -> Box<dyn Fn(EntityReference) + Send + Sync> {
        let request_identifier = request_identifier.clone();
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

            let mut requested_entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            let mut request_identifier = request_identifier.clone();
            query_params!(
                self.0,
                &entity,
                &mut requested_entity_guard,
                &mut request_identifier,
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
                T18
            );
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

    fn inject(
        self,
        request_identifier: &EntityTypeIdentifier,
    ) -> Box<dyn Fn(EntityReference) + Send + Sync> {
        let request_identifier = request_identifier.clone();
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

            let mut requested_entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            let mut request_identifier = request_identifier.clone();
            query_params!(
                self.0,
                &entity,
                &mut requested_entity_guard,
                &mut request_identifier,
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
                T19
            );
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

    fn inject(
        self,
        request_identifier: &EntityTypeIdentifier,
    ) -> Box<dyn Fn(EntityReference) + Send + Sync> {
        let request_identifier = request_identifier.clone();
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

            let mut requested_entity_guard = if require_write {
                RequestedEntityGuard::Write(entity.write())
            } else if require_read {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            let mut request_identifier = request_identifier.clone();
            query_params!(
                self.0,
                &entity,
                &mut requested_entity_guard,
                &mut request_identifier,
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
                T20
            );
        })
    }
}


*/
