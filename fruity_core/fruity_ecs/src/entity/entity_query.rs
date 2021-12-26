use crate::component::component::Component;
use crate::component::component_guard::ComponentReadGuard;
use crate::component::component_guard::ComponentWriteGuard;
use crate::component::component_reference::ComponentReference;
use crate::entity::entity::EntityId;
use crate::entity::entity::EntityTypeIdentifier;
use crate::entity::entity_reference::EntityReference;
use std::fmt::Debug;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

/// RAII structure used to release the shared read access of a lock when dropped.
///
/// This structure is created by the [`read`] methods on [`ComponentRwLock`].
///
/// [`read`]: ComponentRwLock::read
///
pub struct Read<'a, T: Component> {
    guard: RwLockReadGuard<'a, ()>,
    component: &'a T,
}

impl<'a, T: Component> Read<'a, T> {
    /// Returns an RwLockReadGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockReadGuard`]
    ///
    pub(crate) fn new(guard: ComponentReadGuard<'a>) -> Self {
        Self {
            guard: guard.guard,
            component: guard.component.as_any_ref().downcast_ref::<T>().unwrap(),
        }
    }
}

impl<'a, T: Component> Deref for Read<'a, T> {
    type Target = T;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.component
    }
}

impl<'a, T: Component> Debug for Read<'a, T> {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        self.guard.fmt(formatter)
    }
}

/// RAII structure used to release the exclusive write access of a lock when dropped.
///
/// This structure is created by the [`write`] methods on [`ComponentRwLock`].
///
/// [`write`]: ComponentRwLock::write
///
pub struct Write<'a, T: Component> {
    guard: RwLockWriteGuard<'a, ()>,
    component: &'a mut T,
}

impl<'a, T: Component> Write<'a, T> {
    /// Returns an ComponentWriteGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockWriteGuard`]
    ///
    pub(crate) fn new(guard: ComponentWriteGuard<'a>) -> Self {
        Self {
            guard: guard.guard,
            component: guard.component.as_any_mut().downcast_mut::<T>().unwrap(),
        }
    }
}

impl<'a, T: Component> Deref for Write<'a, T> {
    type Target = T;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.component
    }
}

impl<'a, T: Component> DerefMut for Write<'a, T> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        self.component
    }
}

impl<'a, T: Component> Debug for Write<'a, T> {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        self.guard.fmt(formatter)
    }
}

/// A trait for types that can be exposed from components references
pub trait QueryInjectable {
    /// Get the object
    fn from_components(
        entity: &EntityReference,
        request_identifier: &mut EntityTypeIdentifier,
    ) -> Self;
}

impl QueryInjectable for EntityReference {
    fn from_components(
        entity: &EntityReference,
        _request_identifier: &mut EntityTypeIdentifier,
    ) -> Self {
        entity.clone()
    }
}

impl QueryInjectable for ComponentReference {
    fn from_components(
        entity: &EntityReference,
        request_identifier: &mut EntityTypeIdentifier,
    ) -> Self {
        let identifier = request_identifier.0.remove(0);
        entity.get_component(&identifier).unwrap()
    }
}

impl QueryInjectable for EntityId {
    fn from_components(
        entity: &EntityReference,
        _request_identifier: &mut EntityTypeIdentifier,
    ) -> Self {
        entity.get_entity_id()
    }
}

impl QueryInjectable for String {
    fn from_components(
        entity: &EntityReference,
        _request_identifier: &mut EntityTypeIdentifier,
    ) -> Self {
        entity.get_name()
    }
}

impl QueryInjectable for bool {
    fn from_components(
        entity: &EntityReference,
        _request_identifier: &mut EntityTypeIdentifier,
    ) -> Self {
        entity.is_enabled()
    }
}

impl<'a, T: Component> QueryInjectable for Read<'a, T> {
    fn from_components(
        entity: &EntityReference,
        request_identifier: &mut EntityTypeIdentifier,
    ) -> Self {
        let identifier = request_identifier.0.remove(0);
        let component = entity.get_component(&identifier).unwrap();

        // TODO: Find a way to remove it
        let reader = unsafe {
            std::mem::transmute::<ComponentReadGuard, ComponentReadGuard>(component.read())
        };

        Read::new(reader)
    }
}

impl<'a, T: Component> QueryInjectable for Write<'a, T> {
    fn from_components(
        entity: &EntityReference,
        request_identifier: &mut EntityTypeIdentifier,
    ) -> Self {
        let identifier = request_identifier.0.remove(0);
        let component = entity.get_component(&identifier).unwrap();

        // TODO: Find a way to remove it
        let writer = unsafe {
            std::mem::transmute::<ComponentWriteGuard, ComponentWriteGuard>(component.write())
        };

        Write::new(writer)
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
            let mut request_identifier = request_identifier.clone();
            (self.0)(T1::from_components(&entity, &mut request_identifier))
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
            let mut request_identifier = request_identifier.clone();
            (self.0)(
                T1::from_components(&entity, &mut request_identifier),
                T2::from_components(&entity, &mut request_identifier),
            )
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

    fn inject(
        self,
        request_identifier: &EntityTypeIdentifier,
    ) -> Box<dyn Fn(EntityReference) + Send + Sync> {
        let request_identifier = request_identifier.clone();
        Box::new(move |entity| {
            let mut request_identifier = request_identifier.clone();
            (self.0)(
                T1::from_components(&entity, &mut request_identifier),
                T2::from_components(&entity, &mut request_identifier),
                T3::from_components(&entity, &mut request_identifier),
            )
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
            let mut request_identifier = request_identifier.clone();
            (self.0)(
                T1::from_components(&entity, &mut request_identifier),
                T2::from_components(&entity, &mut request_identifier),
                T3::from_components(&entity, &mut request_identifier),
                T4::from_components(&entity, &mut request_identifier),
            )
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
            let mut request_identifier = request_identifier.clone();
            (self.0)(
                T1::from_components(&entity, &mut request_identifier),
                T2::from_components(&entity, &mut request_identifier),
                T3::from_components(&entity, &mut request_identifier),
                T4::from_components(&entity, &mut request_identifier),
                T5::from_components(&entity, &mut request_identifier),
            )
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
            let mut request_identifier = request_identifier.clone();
            (self.0)(
                T1::from_components(&entity, &mut request_identifier),
                T2::from_components(&entity, &mut request_identifier),
                T3::from_components(&entity, &mut request_identifier),
                T4::from_components(&entity, &mut request_identifier),
                T5::from_components(&entity, &mut request_identifier),
                T6::from_components(&entity, &mut request_identifier),
            )
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
            let mut request_identifier = request_identifier.clone();
            (self.0)(
                T1::from_components(&entity, &mut request_identifier),
                T2::from_components(&entity, &mut request_identifier),
                T3::from_components(&entity, &mut request_identifier),
                T4::from_components(&entity, &mut request_identifier),
                T5::from_components(&entity, &mut request_identifier),
                T6::from_components(&entity, &mut request_identifier),
                T7::from_components(&entity, &mut request_identifier),
            )
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
            let mut request_identifier = request_identifier.clone();
            (self.0)(
                T1::from_components(&entity, &mut request_identifier),
                T2::from_components(&entity, &mut request_identifier),
                T3::from_components(&entity, &mut request_identifier),
                T4::from_components(&entity, &mut request_identifier),
                T5::from_components(&entity, &mut request_identifier),
                T6::from_components(&entity, &mut request_identifier),
                T7::from_components(&entity, &mut request_identifier),
                T8::from_components(&entity, &mut request_identifier),
            )
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
            let mut request_identifier = request_identifier.clone();
            (self.0)(
                T1::from_components(&entity, &mut request_identifier),
                T2::from_components(&entity, &mut request_identifier),
                T3::from_components(&entity, &mut request_identifier),
                T4::from_components(&entity, &mut request_identifier),
                T5::from_components(&entity, &mut request_identifier),
                T6::from_components(&entity, &mut request_identifier),
                T7::from_components(&entity, &mut request_identifier),
                T8::from_components(&entity, &mut request_identifier),
                T9::from_components(&entity, &mut request_identifier),
            )
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
            let mut request_identifier = request_identifier.clone();
            (self.0)(
                T1::from_components(&entity, &mut request_identifier),
                T2::from_components(&entity, &mut request_identifier),
                T3::from_components(&entity, &mut request_identifier),
                T4::from_components(&entity, &mut request_identifier),
                T5::from_components(&entity, &mut request_identifier),
                T6::from_components(&entity, &mut request_identifier),
                T7::from_components(&entity, &mut request_identifier),
                T8::from_components(&entity, &mut request_identifier),
                T9::from_components(&entity, &mut request_identifier),
                T10::from_components(&entity, &mut request_identifier),
            )
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
            let mut request_identifier = request_identifier.clone();
            (self.0)(
                T1::from_components(&entity, &mut request_identifier),
                T2::from_components(&entity, &mut request_identifier),
                T3::from_components(&entity, &mut request_identifier),
                T4::from_components(&entity, &mut request_identifier),
                T5::from_components(&entity, &mut request_identifier),
                T6::from_components(&entity, &mut request_identifier),
                T7::from_components(&entity, &mut request_identifier),
                T8::from_components(&entity, &mut request_identifier),
                T9::from_components(&entity, &mut request_identifier),
                T10::from_components(&entity, &mut request_identifier),
                T11::from_components(&entity, &mut request_identifier),
            )
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
            let mut request_identifier = request_identifier.clone();
            (self.0)(
                T1::from_components(&entity, &mut request_identifier),
                T2::from_components(&entity, &mut request_identifier),
                T3::from_components(&entity, &mut request_identifier),
                T4::from_components(&entity, &mut request_identifier),
                T5::from_components(&entity, &mut request_identifier),
                T6::from_components(&entity, &mut request_identifier),
                T7::from_components(&entity, &mut request_identifier),
                T8::from_components(&entity, &mut request_identifier),
                T9::from_components(&entity, &mut request_identifier),
                T10::from_components(&entity, &mut request_identifier),
                T11::from_components(&entity, &mut request_identifier),
                T12::from_components(&entity, &mut request_identifier),
            )
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
            let mut request_identifier = request_identifier.clone();
            (self.0)(
                T1::from_components(&entity, &mut request_identifier),
                T2::from_components(&entity, &mut request_identifier),
                T3::from_components(&entity, &mut request_identifier),
                T4::from_components(&entity, &mut request_identifier),
                T5::from_components(&entity, &mut request_identifier),
                T6::from_components(&entity, &mut request_identifier),
                T7::from_components(&entity, &mut request_identifier),
                T8::from_components(&entity, &mut request_identifier),
                T9::from_components(&entity, &mut request_identifier),
                T10::from_components(&entity, &mut request_identifier),
                T11::from_components(&entity, &mut request_identifier),
                T12::from_components(&entity, &mut request_identifier),
                T13::from_components(&entity, &mut request_identifier),
            )
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
            let mut request_identifier = request_identifier.clone();
            (self.0)(
                T1::from_components(&entity, &mut request_identifier),
                T2::from_components(&entity, &mut request_identifier),
                T3::from_components(&entity, &mut request_identifier),
                T4::from_components(&entity, &mut request_identifier),
                T5::from_components(&entity, &mut request_identifier),
                T6::from_components(&entity, &mut request_identifier),
                T7::from_components(&entity, &mut request_identifier),
                T8::from_components(&entity, &mut request_identifier),
                T9::from_components(&entity, &mut request_identifier),
                T10::from_components(&entity, &mut request_identifier),
                T11::from_components(&entity, &mut request_identifier),
                T12::from_components(&entity, &mut request_identifier),
                T13::from_components(&entity, &mut request_identifier),
                T14::from_components(&entity, &mut request_identifier),
            )
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
            let mut request_identifier = request_identifier.clone();
            (self.0)(
                T1::from_components(&entity, &mut request_identifier),
                T2::from_components(&entity, &mut request_identifier),
                T3::from_components(&entity, &mut request_identifier),
                T4::from_components(&entity, &mut request_identifier),
                T5::from_components(&entity, &mut request_identifier),
                T6::from_components(&entity, &mut request_identifier),
                T7::from_components(&entity, &mut request_identifier),
                T8::from_components(&entity, &mut request_identifier),
                T9::from_components(&entity, &mut request_identifier),
                T10::from_components(&entity, &mut request_identifier),
                T11::from_components(&entity, &mut request_identifier),
                T12::from_components(&entity, &mut request_identifier),
                T13::from_components(&entity, &mut request_identifier),
                T14::from_components(&entity, &mut request_identifier),
                T15::from_components(&entity, &mut request_identifier),
            )
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
            let mut request_identifier = request_identifier.clone();
            (self.0)(
                T1::from_components(&entity, &mut request_identifier),
                T2::from_components(&entity, &mut request_identifier),
                T3::from_components(&entity, &mut request_identifier),
                T4::from_components(&entity, &mut request_identifier),
                T5::from_components(&entity, &mut request_identifier),
                T6::from_components(&entity, &mut request_identifier),
                T7::from_components(&entity, &mut request_identifier),
                T8::from_components(&entity, &mut request_identifier),
                T9::from_components(&entity, &mut request_identifier),
                T10::from_components(&entity, &mut request_identifier),
                T11::from_components(&entity, &mut request_identifier),
                T12::from_components(&entity, &mut request_identifier),
                T13::from_components(&entity, &mut request_identifier),
                T14::from_components(&entity, &mut request_identifier),
                T15::from_components(&entity, &mut request_identifier),
                T16::from_components(&entity, &mut request_identifier),
            )
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
            let mut request_identifier = request_identifier.clone();
            (self.0)(
                T1::from_components(&entity, &mut request_identifier),
                T2::from_components(&entity, &mut request_identifier),
                T3::from_components(&entity, &mut request_identifier),
                T4::from_components(&entity, &mut request_identifier),
                T5::from_components(&entity, &mut request_identifier),
                T6::from_components(&entity, &mut request_identifier),
                T7::from_components(&entity, &mut request_identifier),
                T8::from_components(&entity, &mut request_identifier),
                T9::from_components(&entity, &mut request_identifier),
                T10::from_components(&entity, &mut request_identifier),
                T11::from_components(&entity, &mut request_identifier),
                T12::from_components(&entity, &mut request_identifier),
                T13::from_components(&entity, &mut request_identifier),
                T14::from_components(&entity, &mut request_identifier),
                T15::from_components(&entity, &mut request_identifier),
                T16::from_components(&entity, &mut request_identifier),
                T17::from_components(&entity, &mut request_identifier),
            )
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
            let mut request_identifier = request_identifier.clone();
            (self.0)(
                T1::from_components(&entity, &mut request_identifier),
                T2::from_components(&entity, &mut request_identifier),
                T3::from_components(&entity, &mut request_identifier),
                T4::from_components(&entity, &mut request_identifier),
                T5::from_components(&entity, &mut request_identifier),
                T6::from_components(&entity, &mut request_identifier),
                T7::from_components(&entity, &mut request_identifier),
                T8::from_components(&entity, &mut request_identifier),
                T9::from_components(&entity, &mut request_identifier),
                T10::from_components(&entity, &mut request_identifier),
                T11::from_components(&entity, &mut request_identifier),
                T12::from_components(&entity, &mut request_identifier),
                T13::from_components(&entity, &mut request_identifier),
                T14::from_components(&entity, &mut request_identifier),
                T15::from_components(&entity, &mut request_identifier),
                T16::from_components(&entity, &mut request_identifier),
                T17::from_components(&entity, &mut request_identifier),
                T18::from_components(&entity, &mut request_identifier),
            )
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
            let mut request_identifier = request_identifier.clone();
            (self.0)(
                T1::from_components(&entity, &mut request_identifier),
                T2::from_components(&entity, &mut request_identifier),
                T3::from_components(&entity, &mut request_identifier),
                T4::from_components(&entity, &mut request_identifier),
                T5::from_components(&entity, &mut request_identifier),
                T6::from_components(&entity, &mut request_identifier),
                T7::from_components(&entity, &mut request_identifier),
                T8::from_components(&entity, &mut request_identifier),
                T9::from_components(&entity, &mut request_identifier),
                T10::from_components(&entity, &mut request_identifier),
                T11::from_components(&entity, &mut request_identifier),
                T12::from_components(&entity, &mut request_identifier),
                T13::from_components(&entity, &mut request_identifier),
                T14::from_components(&entity, &mut request_identifier),
                T15::from_components(&entity, &mut request_identifier),
                T16::from_components(&entity, &mut request_identifier),
                T17::from_components(&entity, &mut request_identifier),
                T18::from_components(&entity, &mut request_identifier),
                T19::from_components(&entity, &mut request_identifier),
            )
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
            let mut request_identifier = request_identifier.clone();
            (self.0)(
                T1::from_components(&entity, &mut request_identifier),
                T2::from_components(&entity, &mut request_identifier),
                T3::from_components(&entity, &mut request_identifier),
                T4::from_components(&entity, &mut request_identifier),
                T5::from_components(&entity, &mut request_identifier),
                T6::from_components(&entity, &mut request_identifier),
                T7::from_components(&entity, &mut request_identifier),
                T8::from_components(&entity, &mut request_identifier),
                T9::from_components(&entity, &mut request_identifier),
                T10::from_components(&entity, &mut request_identifier),
                T11::from_components(&entity, &mut request_identifier),
                T12::from_components(&entity, &mut request_identifier),
                T13::from_components(&entity, &mut request_identifier),
                T14::from_components(&entity, &mut request_identifier),
                T15::from_components(&entity, &mut request_identifier),
                T16::from_components(&entity, &mut request_identifier),
                T17::from_components(&entity, &mut request_identifier),
                T18::from_components(&entity, &mut request_identifier),
                T19::from_components(&entity, &mut request_identifier),
                T20::from_components(&entity, &mut request_identifier),
            )
        })
    }
}
