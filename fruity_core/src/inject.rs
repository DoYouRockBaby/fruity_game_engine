use crate::resource::resource::Resource;
use crate::resource::resource_container::ResourceContainer;
use crate::resource::resource_reference::ResourceReadGuard;
use crate::resource::resource_reference::ResourceReference;
use crate::resource::resource_reference::ResourceWriteGuard;
use std::sync::Arc;

/// A reference over a resource
pub type Ref<T> = ResourceReference<T>;

/// A read guard over a resource
pub type Const<T> = ResourceReadGuard<T>;

/// A write guard over a resource
pub type Mut<T> = ResourceWriteGuard<T>;

/// A trait for types that can be exposed from resource container
pub trait Injectable: 'static {
    /// Get the object
    fn from_resource_container(resource_container: &Arc<ResourceContainer>) -> Self;
}

impl Injectable for Arc<ResourceContainer> {
    fn from_resource_container(resource_container: &Arc<ResourceContainer>) -> Self {
        resource_container.clone()
    }
}

impl<T: Resource + ?Sized> Injectable for Ref<T> {
    fn from_resource_container(resource_container: &Arc<ResourceContainer>) -> Self {
        resource_container.require::<T>()
    }
}

impl<T: Resource + ?Sized> Injectable for Const<T> {
    fn from_resource_container(resource_container: &Arc<ResourceContainer>) -> Self {
        let reference = Ref::<T>::from_resource_container(resource_container);
        reference.read()
    }
}

impl<T: Resource + ?Sized> Injectable for Mut<T> {
    fn from_resource_container(resource_container: &Arc<ResourceContainer>) -> Self {
        let reference = Ref::<T>::from_resource_container(resource_container);
        reference.write()
    }
}

/// A trait that is implemented by functions that supports dependency injection
pub trait Inject<R = ()> {
    /// Get a function that proceed the injection
    fn inject(self) -> Box<dyn Fn(Arc<ResourceContainer>) -> R + Send + Sync>;
}

/// A shortcut for a boxed inject function
pub struct Inject0<R = ()>(Box<dyn Fn() -> R + Send + Sync>);

impl<R> Inject0<R> {
    /// New instance
    pub fn new(val: impl Fn() -> R + Send + Sync + 'static) -> Self {
        Self(Box::new(val))
    }
}

impl<R: 'static> Inject<R> for Inject0<R> {
    fn inject(self) -> Box<dyn Fn(Arc<ResourceContainer>) -> R + Send + Sync> {
        Box::new(move |_| (self.0)())
    }
}

/// A shortcut for a boxed inject function
pub struct Inject1<T1, R = ()>(Box<dyn Fn(T1) -> R + Send + Sync>);

impl<T1, R> Inject1<T1, R> {
    /// New instance
    pub fn new(val: impl Fn(T1) -> R + Send + Sync + 'static) -> Self {
        Self(Box::new(val))
    }
}

impl<T1: Injectable, R: 'static> Inject<R> for Inject1<T1, R> {
    fn inject(self) -> Box<dyn Fn(Arc<ResourceContainer>) -> R + Send + Sync> {
        Box::new(move |resource_container| {
            (self.0)(T1::from_resource_container(&resource_container))
        })
    }
}

/// A shortcut for a boxed inject function
pub struct Inject2<T1, T2, R = ()>(Box<dyn Fn(T1, T2) -> R + Send + Sync>);

impl<T1, T2, R> Inject2<T1, T2, R> {
    /// New instance
    pub fn new(val: impl Fn(T1, T2) -> R + Send + Sync + 'static) -> Self {
        Self(Box::new(val))
    }
}

impl<T1: Injectable, T2: Injectable, R: 'static> Inject<R> for Inject2<T1, T2, R> {
    fn inject(self) -> Box<dyn Fn(Arc<ResourceContainer>) -> R + Send + Sync> {
        Box::new(move |resource_container| {
            (self.0)(
                T1::from_resource_container(&resource_container),
                T2::from_resource_container(&resource_container),
            )
        })
    }
}

/// A shortcut for a boxed inject function
pub struct Inject3<T1, T2, T3>(Box<dyn Fn(T1, T2, T3) + Send + Sync>);

impl<T1, T2, T3> Inject3<T1, T2, T3> {
    /// New instance
    pub fn new(val: impl Fn(T1, T2, T3) + Send + Sync + 'static) -> Self {
        Self(Box::new(val))
    }
}

impl<T1: Injectable, T2: Injectable, T3: Injectable> Inject for Inject3<T1, T2, T3> {
    fn inject(self) -> Box<dyn Fn(Arc<ResourceContainer>) + Send + Sync> {
        Box::new(move |resource_container| {
            (self.0)(
                T1::from_resource_container(&resource_container),
                T2::from_resource_container(&resource_container),
                T3::from_resource_container(&resource_container),
            )
        })
    }
}

/// A shortcut for a boxed inject function
pub struct Inject4<T1, T2, T3, T4>(Box<dyn Fn(T1, T2, T3, T4) + Send + Sync>);

impl<T1, T2, T3, T4> Inject4<T1, T2, T3, T4> {
    /// New instance
    pub fn new(val: impl Fn(T1, T2, T3, T4) + Send + Sync + 'static) -> Self {
        Self(Box::new(val))
    }
}

impl<T1: Injectable, T2: Injectable, T3: Injectable, T4: Injectable> Inject
    for Inject4<T1, T2, T3, T4>
{
    fn inject(self) -> Box<dyn Fn(Arc<ResourceContainer>) + Send + Sync> {
        Box::new(move |resource_container| {
            (self.0)(
                T1::from_resource_container(&resource_container),
                T2::from_resource_container(&resource_container),
                T3::from_resource_container(&resource_container),
                T4::from_resource_container(&resource_container),
            )
        })
    }
}

/// A shortcut for a boxed inject function
pub struct Inject5<T1, T2, T3, T4, T5>(Box<dyn Fn(T1, T2, T3, T4, T5) + Send + Sync>);

impl<T1, T2, T3, T4, T5> Inject5<T1, T2, T3, T4, T5> {
    /// New instance
    pub fn new(val: impl Fn(T1, T2, T3, T4, T5) + Send + Sync + 'static) -> Self {
        Self(Box::new(val))
    }
}

impl<T1: Injectable, T2: Injectable, T3: Injectable, T4: Injectable, T5: Injectable> Inject
    for Inject5<T1, T2, T3, T4, T5>
{
    fn inject(self) -> Box<dyn Fn(Arc<ResourceContainer>) + Send + Sync> {
        Box::new(move |resource_container| {
            (self.0)(
                T1::from_resource_container(&resource_container),
                T2::from_resource_container(&resource_container),
                T3::from_resource_container(&resource_container),
                T4::from_resource_container(&resource_container),
                T5::from_resource_container(&resource_container),
            )
        })
    }
}

/// A shortcut for a boxed inject function
pub struct Inject6<T1, T2, T3, T4, T5, T6>(Box<dyn Fn(T1, T2, T3, T4, T5, T6) + Send + Sync>);

impl<T1, T2, T3, T4, T5, T6> Inject6<T1, T2, T3, T4, T5, T6> {
    /// New instance
    pub fn new(val: impl Fn(T1, T2, T3, T4, T5, T6) + Send + Sync + 'static) -> Self {
        Self(Box::new(val))
    }
}

impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
    > Inject for Inject6<T1, T2, T3, T4, T5, T6>
{
    fn inject(self) -> Box<dyn Fn(Arc<ResourceContainer>) + Send + Sync> {
        Box::new(move |resource_container| {
            (self.0)(
                T1::from_resource_container(&resource_container),
                T2::from_resource_container(&resource_container),
                T3::from_resource_container(&resource_container),
                T4::from_resource_container(&resource_container),
                T5::from_resource_container(&resource_container),
                T6::from_resource_container(&resource_container),
            )
        })
    }
}

/// A shortcut for a boxed inject function
pub struct Inject7<T1, T2, T3, T4, T5, T6, T7>(
    Box<dyn Fn(T1, T2, T3, T4, T5, T6, T7) + Send + Sync>,
);

impl<T1, T2, T3, T4, T5, T6, T7> Inject7<T1, T2, T3, T4, T5, T6, T7> {
    /// New instance
    pub fn new(val: impl Fn(T1, T2, T3, T4, T5, T6, T7) + Send + Sync + 'static) -> Self {
        Self(Box::new(val))
    }
}

impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
    > Inject for Inject7<T1, T2, T3, T4, T5, T6, T7>
{
    fn inject(self) -> Box<dyn Fn(Arc<ResourceContainer>) + Send + Sync> {
        Box::new(move |resource_container| {
            (self.0)(
                T1::from_resource_container(&resource_container),
                T2::from_resource_container(&resource_container),
                T3::from_resource_container(&resource_container),
                T4::from_resource_container(&resource_container),
                T5::from_resource_container(&resource_container),
                T6::from_resource_container(&resource_container),
                T7::from_resource_container(&resource_container),
            )
        })
    }
}

/// A shortcut for a boxed inject function
pub struct Inject8<T1, T2, T3, T4, T5, T6, T7, T8>(
    Box<dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8) + Send + Sync>,
);

impl<T1, T2, T3, T4, T5, T6, T7, T8> Inject8<T1, T2, T3, T4, T5, T6, T7, T8> {
    /// New instance
    pub fn new(val: impl Fn(T1, T2, T3, T4, T5, T6, T7, T8) + Send + Sync + 'static) -> Self {
        Self(Box::new(val))
    }
}

impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
    > Inject for Inject8<T1, T2, T3, T4, T5, T6, T7, T8>
{
    fn inject(self) -> Box<dyn Fn(Arc<ResourceContainer>) + Send + Sync> {
        Box::new(move |resource_container| {
            (self.0)(
                T1::from_resource_container(&resource_container),
                T2::from_resource_container(&resource_container),
                T3::from_resource_container(&resource_container),
                T4::from_resource_container(&resource_container),
                T5::from_resource_container(&resource_container),
                T6::from_resource_container(&resource_container),
                T7::from_resource_container(&resource_container),
                T8::from_resource_container(&resource_container),
            )
        })
    }
}

/// A shortcut for a boxed inject function
pub struct Inject9<T1, T2, T3, T4, T5, T6, T7, T8, T9>(
    Box<dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9) + Send + Sync>,
);

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9> Inject9<T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    /// New instance
    pub fn new(val: impl Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9) + Send + Sync + 'static) -> Self {
        Self(Box::new(val))
    }
}

impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
        T9: Injectable,
    > Inject for Inject9<T1, T2, T3, T4, T5, T6, T7, T8, T9>
{
    fn inject(self) -> Box<dyn Fn(Arc<ResourceContainer>) + Send + Sync> {
        Box::new(move |resource_container| {
            (self.0)(
                T1::from_resource_container(&resource_container),
                T2::from_resource_container(&resource_container),
                T3::from_resource_container(&resource_container),
                T4::from_resource_container(&resource_container),
                T5::from_resource_container(&resource_container),
                T6::from_resource_container(&resource_container),
                T7::from_resource_container(&resource_container),
                T8::from_resource_container(&resource_container),
                T9::from_resource_container(&resource_container),
            )
        })
    }
}

/// A shortcut for a boxed inject function
pub struct Inject10<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>(
    Box<dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) + Send + Sync>,
);

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Inject10<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    /// New instance
    pub fn new(
        val: impl Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) + Send + Sync + 'static,
    ) -> Self {
        Self(Box::new(val))
    }
}

impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
        T9: Injectable,
        T10: Injectable,
    > Inject for Inject10<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>
{
    fn inject(self) -> Box<dyn Fn(Arc<ResourceContainer>) + Send + Sync> {
        Box::new(move |resource_container| {
            (self.0)(
                T1::from_resource_container(&resource_container),
                T2::from_resource_container(&resource_container),
                T3::from_resource_container(&resource_container),
                T4::from_resource_container(&resource_container),
                T5::from_resource_container(&resource_container),
                T6::from_resource_container(&resource_container),
                T7::from_resource_container(&resource_container),
                T8::from_resource_container(&resource_container),
                T9::from_resource_container(&resource_container),
                T10::from_resource_container(&resource_container),
            )
        })
    }
}

/// A shortcut for a boxed inject function
pub struct Inject11<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>(
    Box<dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) + Send + Sync>,
);

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
    Inject11<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
{
    /// New instance
    pub fn new(
        val: impl Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) + Send + Sync + 'static,
    ) -> Self {
        Self(Box::new(val))
    }
}

impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
        T9: Injectable,
        T10: Injectable,
        T11: Injectable,
    > Inject for Inject11<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
{
    fn inject(self) -> Box<dyn Fn(Arc<ResourceContainer>) + Send + Sync> {
        Box::new(move |resource_container| {
            (self.0)(
                T1::from_resource_container(&resource_container),
                T2::from_resource_container(&resource_container),
                T3::from_resource_container(&resource_container),
                T4::from_resource_container(&resource_container),
                T5::from_resource_container(&resource_container),
                T6::from_resource_container(&resource_container),
                T7::from_resource_container(&resource_container),
                T8::from_resource_container(&resource_container),
                T9::from_resource_container(&resource_container),
                T10::from_resource_container(&resource_container),
                T11::from_resource_container(&resource_container),
            )
        })
    }
}

/// A shortcut for a boxed inject function
pub struct Inject12<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>(
    Box<dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) + Send + Sync>,
);

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>
    Inject12<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>
{
    /// New instance
    pub fn new(
        val: impl Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) + Send + Sync + 'static,
    ) -> Self {
        Self(Box::new(val))
    }
}

impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
        T9: Injectable,
        T10: Injectable,
        T11: Injectable,
        T12: Injectable,
    > Inject for Inject12<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>
{
    fn inject(self) -> Box<dyn Fn(Arc<ResourceContainer>) + Send + Sync> {
        Box::new(move |resource_container| {
            (self.0)(
                T1::from_resource_container(&resource_container),
                T2::from_resource_container(&resource_container),
                T3::from_resource_container(&resource_container),
                T4::from_resource_container(&resource_container),
                T5::from_resource_container(&resource_container),
                T6::from_resource_container(&resource_container),
                T7::from_resource_container(&resource_container),
                T8::from_resource_container(&resource_container),
                T9::from_resource_container(&resource_container),
                T10::from_resource_container(&resource_container),
                T11::from_resource_container(&resource_container),
                T12::from_resource_container(&resource_container),
            )
        })
    }
}

/// A shortcut for a boxed inject function
pub struct Inject13<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>(
    Box<dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) + Send + Sync>,
);

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>
    Inject13<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>
{
    /// New instance
    pub fn new(
        val: impl Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) + Send + Sync + 'static,
    ) -> Self {
        Self(Box::new(val))
    }
}

impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
        T9: Injectable,
        T10: Injectable,
        T11: Injectable,
        T12: Injectable,
        T13: Injectable,
    > Inject for Inject13<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>
{
    fn inject(self) -> Box<dyn Fn(Arc<ResourceContainer>) + Send + Sync> {
        Box::new(move |resource_container| {
            (self.0)(
                T1::from_resource_container(&resource_container),
                T2::from_resource_container(&resource_container),
                T3::from_resource_container(&resource_container),
                T4::from_resource_container(&resource_container),
                T5::from_resource_container(&resource_container),
                T6::from_resource_container(&resource_container),
                T7::from_resource_container(&resource_container),
                T8::from_resource_container(&resource_container),
                T9::from_resource_container(&resource_container),
                T10::from_resource_container(&resource_container),
                T11::from_resource_container(&resource_container),
                T12::from_resource_container(&resource_container),
                T13::from_resource_container(&resource_container),
            )
        })
    }
}

/// A shortcut for a boxed inject function
pub struct Inject14<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>(
    Box<dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) + Send + Sync>,
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
        Self(Box::new(val))
    }
}

impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
        T9: Injectable,
        T10: Injectable,
        T11: Injectable,
        T12: Injectable,
        T13: Injectable,
        T14: Injectable,
    > Inject for Inject14<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>
{
    fn inject(self) -> Box<dyn Fn(Arc<ResourceContainer>) + Send + Sync> {
        Box::new(move |resource_container| {
            (self.0)(
                T1::from_resource_container(&resource_container),
                T2::from_resource_container(&resource_container),
                T3::from_resource_container(&resource_container),
                T4::from_resource_container(&resource_container),
                T5::from_resource_container(&resource_container),
                T6::from_resource_container(&resource_container),
                T7::from_resource_container(&resource_container),
                T8::from_resource_container(&resource_container),
                T9::from_resource_container(&resource_container),
                T10::from_resource_container(&resource_container),
                T11::from_resource_container(&resource_container),
                T12::from_resource_container(&resource_container),
                T13::from_resource_container(&resource_container),
                T14::from_resource_container(&resource_container),
            )
        })
    }
}

/// A shortcut for a boxed inject function
pub struct Inject15<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>(
    Box<dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) + Send + Sync>,
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
        Self(Box::new(val))
    }
}

impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
        T9: Injectable,
        T10: Injectable,
        T11: Injectable,
        T12: Injectable,
        T13: Injectable,
        T14: Injectable,
        T15: Injectable,
    > Inject for Inject15<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>
{
    fn inject(self) -> Box<dyn Fn(Arc<ResourceContainer>) + Send + Sync> {
        Box::new(move |resource_container| {
            (self.0)(
                T1::from_resource_container(&resource_container),
                T2::from_resource_container(&resource_container),
                T3::from_resource_container(&resource_container),
                T4::from_resource_container(&resource_container),
                T5::from_resource_container(&resource_container),
                T6::from_resource_container(&resource_container),
                T7::from_resource_container(&resource_container),
                T8::from_resource_container(&resource_container),
                T9::from_resource_container(&resource_container),
                T10::from_resource_container(&resource_container),
                T11::from_resource_container(&resource_container),
                T12::from_resource_container(&resource_container),
                T13::from_resource_container(&resource_container),
                T14::from_resource_container(&resource_container),
                T15::from_resource_container(&resource_container),
            )
        })
    }
}

/// A shortcut for a boxed inject function
pub struct Inject16<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16>(
    Box<
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
        Self(Box::new(val))
    }
}

impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
        T9: Injectable,
        T10: Injectable,
        T11: Injectable,
        T12: Injectable,
        T13: Injectable,
        T14: Injectable,
        T15: Injectable,
        T16: Injectable,
    > Inject for Inject16<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16>
{
    fn inject(self) -> Box<dyn Fn(Arc<ResourceContainer>) + Send + Sync> {
        Box::new(move |resource_container| {
            (self.0)(
                T1::from_resource_container(&resource_container),
                T2::from_resource_container(&resource_container),
                T3::from_resource_container(&resource_container),
                T4::from_resource_container(&resource_container),
                T5::from_resource_container(&resource_container),
                T6::from_resource_container(&resource_container),
                T7::from_resource_container(&resource_container),
                T8::from_resource_container(&resource_container),
                T9::from_resource_container(&resource_container),
                T10::from_resource_container(&resource_container),
                T11::from_resource_container(&resource_container),
                T12::from_resource_container(&resource_container),
                T13::from_resource_container(&resource_container),
                T14::from_resource_container(&resource_container),
                T15::from_resource_container(&resource_container),
                T16::from_resource_container(&resource_container),
            )
        })
    }
}

/// A shortcut for a boxed inject function
pub struct Inject17<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17>(
    Box<
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
        Self(Box::new(val))
    }
}

impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
        T9: Injectable,
        T10: Injectable,
        T11: Injectable,
        T12: Injectable,
        T13: Injectable,
        T14: Injectable,
        T15: Injectable,
        T16: Injectable,
        T17: Injectable,
    > Inject
    for Inject17<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17>
{
    fn inject(self) -> Box<dyn Fn(Arc<ResourceContainer>) + Send + Sync> {
        Box::new(move |resource_container| {
            (self.0)(
                T1::from_resource_container(&resource_container),
                T2::from_resource_container(&resource_container),
                T3::from_resource_container(&resource_container),
                T4::from_resource_container(&resource_container),
                T5::from_resource_container(&resource_container),
                T6::from_resource_container(&resource_container),
                T7::from_resource_container(&resource_container),
                T8::from_resource_container(&resource_container),
                T9::from_resource_container(&resource_container),
                T10::from_resource_container(&resource_container),
                T11::from_resource_container(&resource_container),
                T12::from_resource_container(&resource_container),
                T13::from_resource_container(&resource_container),
                T14::from_resource_container(&resource_container),
                T15::from_resource_container(&resource_container),
                T16::from_resource_container(&resource_container),
                T17::from_resource_container(&resource_container),
            )
        })
    }
}

/// A shortcut for a boxed inject function
pub struct Inject18<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18>(
    Box<
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
        Self(Box::new(val))
    }
}

impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
        T9: Injectable,
        T10: Injectable,
        T11: Injectable,
        T12: Injectable,
        T13: Injectable,
        T14: Injectable,
        T15: Injectable,
        T16: Injectable,
        T17: Injectable,
        T18: Injectable,
    > Inject
    for Inject18<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18>
{
    fn inject(self) -> Box<dyn Fn(Arc<ResourceContainer>) + Send + Sync> {
        Box::new(move |resource_container| {
            (self.0)(
                T1::from_resource_container(&resource_container),
                T2::from_resource_container(&resource_container),
                T3::from_resource_container(&resource_container),
                T4::from_resource_container(&resource_container),
                T5::from_resource_container(&resource_container),
                T6::from_resource_container(&resource_container),
                T7::from_resource_container(&resource_container),
                T8::from_resource_container(&resource_container),
                T9::from_resource_container(&resource_container),
                T10::from_resource_container(&resource_container),
                T11::from_resource_container(&resource_container),
                T12::from_resource_container(&resource_container),
                T13::from_resource_container(&resource_container),
                T14::from_resource_container(&resource_container),
                T15::from_resource_container(&resource_container),
                T16::from_resource_container(&resource_container),
                T17::from_resource_container(&resource_container),
                T18::from_resource_container(&resource_container),
            )
        })
    }
}

/// A shortcut for a boxed inject function
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
    Box<
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
        Self(Box::new(val))
    }
}

impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
        T9: Injectable,
        T10: Injectable,
        T11: Injectable,
        T12: Injectable,
        T13: Injectable,
        T14: Injectable,
        T15: Injectable,
        T16: Injectable,
        T17: Injectable,
        T18: Injectable,
        T19: Injectable,
    > Inject
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
    fn inject(self) -> Box<dyn Fn(Arc<ResourceContainer>) + Send + Sync> {
        Box::new(move |resource_container| {
            (self.0)(
                T1::from_resource_container(&resource_container),
                T2::from_resource_container(&resource_container),
                T3::from_resource_container(&resource_container),
                T4::from_resource_container(&resource_container),
                T5::from_resource_container(&resource_container),
                T6::from_resource_container(&resource_container),
                T7::from_resource_container(&resource_container),
                T8::from_resource_container(&resource_container),
                T9::from_resource_container(&resource_container),
                T10::from_resource_container(&resource_container),
                T11::from_resource_container(&resource_container),
                T12::from_resource_container(&resource_container),
                T13::from_resource_container(&resource_container),
                T14::from_resource_container(&resource_container),
                T15::from_resource_container(&resource_container),
                T16::from_resource_container(&resource_container),
                T17::from_resource_container(&resource_container),
                T18::from_resource_container(&resource_container),
                T19::from_resource_container(&resource_container),
            )
        })
    }
}

/// A shortcut for a boxed inject function
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
    Box<
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
        Self(Box::new(val))
    }
}

impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
        T9: Injectable,
        T10: Injectable,
        T11: Injectable,
        T12: Injectable,
        T13: Injectable,
        T14: Injectable,
        T15: Injectable,
        T16: Injectable,
        T17: Injectable,
        T18: Injectable,
        T19: Injectable,
        T20: Injectable,
    > Inject
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
    fn inject(self) -> Box<dyn Fn(Arc<ResourceContainer>) + Send + Sync> {
        Box::new(move |resource_container| {
            (self.0)(
                T1::from_resource_container(&resource_container),
                T2::from_resource_container(&resource_container),
                T3::from_resource_container(&resource_container),
                T4::from_resource_container(&resource_container),
                T5::from_resource_container(&resource_container),
                T6::from_resource_container(&resource_container),
                T7::from_resource_container(&resource_container),
                T8::from_resource_container(&resource_container),
                T9::from_resource_container(&resource_container),
                T10::from_resource_container(&resource_container),
                T11::from_resource_container(&resource_container),
                T12::from_resource_container(&resource_container),
                T13::from_resource_container(&resource_container),
                T14::from_resource_container(&resource_container),
                T15::from_resource_container(&resource_container),
                T16::from_resource_container(&resource_container),
                T17::from_resource_container(&resource_container),
                T18::from_resource_container(&resource_container),
                T19::from_resource_container(&resource_container),
                T20::from_resource_container(&resource_container),
            )
        })
    }
}
