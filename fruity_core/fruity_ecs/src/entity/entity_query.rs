use crate::component::component::Component;
use crate::component::component_list_rwlock::ComponentListRwLock;
use std::sync::Arc;

#[derive(Clone, Debug)]
enum EntityQueryError {
    WrongComponentType,
}

impl ToString for EntityQueryError {
    fn to_string(&self) -> String {
        match self {
            EntityQueryError::WrongComponentType => {
                format!("Tried to launch system a system, wrong component type",)
            }
        }
    }
}

trait EntityQueryParam: 'static {
    fn from_component_ref(component: &dyn Component) -> Result<&Self, EntityQueryError>;
    fn from_component_mut<'a, 'b>(
        component: &'a mut dyn Component,
    ) -> Result<&'b mut Self, EntityQueryError>;
}

impl<T: Component> EntityQueryParam for T {
    fn from_component_ref(component: &dyn Component) -> Result<&Self, EntityQueryError> {
        match component.as_any_ref().downcast_ref::<T>() {
            Some(component) => Ok(component),
            None => Err(EntityQueryError::WrongComponentType),
        }
    }

    fn from_component_mut<'a, 'b>(
        component: &'a mut dyn Component,
    ) -> Result<&'b mut Self, EntityQueryError> {
        // TODO: Find a way to remove it
        let component =
            unsafe { std::mem::transmute::<&mut dyn Component, &mut dyn Component>(component) };

        match component.as_any_mut().downcast_mut::<T>() {
            Some(component) => Ok(component),
            None => Err(EntityQueryError::WrongComponentType),
        }
    }
}

/// A callback for entity read actions
pub trait EntityQueryReadCallback: Send + Sync {
    /// Inject the components in the callback
    fn inject_components(&self) -> Box<dyn Fn(ComponentListRwLock) + Send + Sync>;
}

/// A callback for entity write actions
pub trait EntityQueryWriteCallback: Send + Sync {
    /// Inject the components in the callback
    fn inject_components(&self) -> Box<dyn Fn(ComponentListRwLock) + Send + Sync>;
}

/// A shortcut for an arc entity callback
pub struct EntityQueryReadCallback0(Arc<dyn Fn() + Send + Sync>);

impl EntityQueryReadCallback0 {
    /// New instance
    pub fn new(val: impl Fn() + Send + Sync + 'static) -> Self {
        Self(Arc::new(val))
    }
}

/// A shortcut for an arc entity callback
pub struct EntityQueryWriteCallback0(Arc<dyn Fn() + Send + Sync>);

impl EntityQueryWriteCallback0 {
    /// New instance
    pub fn new(val: impl Fn() + Send + Sync + 'static) -> Self {
        Self(Arc::new(val))
    }
}

impl EntityQueryReadCallback for EntityQueryReadCallback0 {
    fn inject_components(&self) -> Box<dyn Fn(ComponentListRwLock) + Send + Sync> {
        let callback = self.0.clone();
        Box::new(move |_| callback())
    }
}

impl EntityQueryWriteCallback for EntityQueryWriteCallback0 {
    fn inject_components(&self) -> Box<dyn Fn(ComponentListRwLock) + Send + Sync> {
        let callback = self.0.clone();
        Box::new(move |_| callback())
    }
}

/// A shortcut for a boxed entity callback
#[derive(Clone)]
pub struct EntityQueryReadCallback1<T1>(Arc<dyn Fn(&T1) + Send + Sync>);

impl<T1> EntityQueryReadCallback1<T1> {
    /// New instance
    pub fn new(val: impl Fn(&T1) + Send + Sync + 'static) -> Self {
        Self(Arc::new(val))
    }
}

/// A shortcut for a boxed entity callback
#[derive(Clone)]
pub struct EntityQueryWriteCallback1<T1>(Arc<dyn Fn(&mut T1) + Send + Sync>);

impl<T1> EntityQueryWriteCallback1<T1> {
    /// New instance
    pub fn new(val: impl Fn(&mut T1) + Send + Sync + 'static) -> Self {
        Self(Arc::new(val))
    }
}

impl<T1: Component> EntityQueryReadCallback for EntityQueryReadCallback1<T1> {
    fn inject_components(&self) -> Box<dyn Fn(ComponentListRwLock) + Send + Sync> {
        let callback = self.0.clone();
        Box::new(move |component_list| {
            let component_list = component_list.read();
            let components = component_list.get_components();

            callback(T1::from_component_ref(components[0]).unwrap())
        })
    }
}

impl<T1: Component> EntityQueryWriteCallback for EntityQueryWriteCallback1<T1> {
    fn inject_components(&self) -> Box<dyn Fn(ComponentListRwLock) + Send + Sync> {
        let callback = self.0.clone();
        Box::new(move |component_list| {
            let mut component_list = component_list.write();
            let mut components = component_list.get_components_mut();

            callback(T1::from_component_mut(components[0]).unwrap())
        })
    }
}

/// A shortcut for a boxed entity callback
#[derive(Clone)]
pub struct EntityQueryReadCallback2<T1, T2>(Arc<dyn Fn(&T1, &T2) + Send + Sync>);

impl<T1, T2> EntityQueryReadCallback2<T1, T2> {
    /// New instance
    pub fn new(val: impl Fn(&T1, &T2) + Send + Sync + 'static) -> Self {
        Self(Arc::new(val))
    }
}

/// A shortcut for a boxed entity callback
#[derive(Clone)]
pub struct EntityQueryWriteCallback2<T1, T2>(Arc<dyn Fn(&mut T1, &mut T2) + Send + Sync>);

impl<T1, T2> EntityQueryWriteCallback2<T1, T2> {
    /// New instance
    pub fn new(val: impl Fn(&mut T1, &mut T2) + Send + Sync + 'static) -> Self {
        Self(Arc::new(val))
    }
}

impl<T1: Component, T2: Component> EntityQueryReadCallback for EntityQueryReadCallback2<T1, T2> {
    fn inject_components(&self) -> Box<dyn Fn(ComponentListRwLock) + Send + Sync> {
        let callback = self.0.clone();
        Box::new(move |component_list| {
            let component_list = component_list.read();
            let components = component_list.get_components();

            callback(
                T1::from_component_ref(components[0]).unwrap(),
                T2::from_component_ref(components[1]).unwrap(),
            )
        })
    }
}

impl<T1: Component, T2: Component> EntityQueryWriteCallback for EntityQueryWriteCallback2<T1, T2> {
    fn inject_components(&self) -> Box<dyn Fn(ComponentListRwLock) + Send + Sync> {
        let callback = self.0.clone();
        Box::new(move |component_list| {
            let mut component_list = component_list.write();
            let mut components = component_list.get_components_mut();

            callback(
                T1::from_component_mut(components[0]).unwrap(),
                T2::from_component_mut(components[1]).unwrap(),
            )
        })
    }
}

/// A shortcut for a boxed entity callback
#[derive(Clone)]
pub struct EntityQueryReadCallback3<T1, T2, T3>(Arc<dyn Fn(&T1, &T2, &T3) + Send + Sync>);

impl<T1, T2, T3> EntityQueryReadCallback3<T1, T2, T3> {
    /// New instance
    pub fn new(val: impl Fn(&T1, &T2, &T3) + Send + Sync + 'static) -> Self {
        Self(Arc::new(val))
    }
}

/// A shortcut for a boxed entity callback
#[derive(Clone)]
pub struct EntityQueryWriteCallback3<T1, T2, T3>(
    Arc<dyn Fn(&mut T1, &mut T2, &mut T3) + Send + Sync>,
);

impl<T1, T2, T3> EntityQueryWriteCallback3<T1, T2, T3> {
    /// New instance
    pub fn new(val: impl Fn(&mut T1, &mut T2, &mut T3) + Send + Sync + 'static) -> Self {
        Self(Arc::new(val))
    }
}

impl<T1: Component, T2: Component, T3: Component> EntityQueryReadCallback
    for EntityQueryReadCallback3<T1, T2, T3>
{
    fn inject_components(&self) -> Box<dyn Fn(ComponentListRwLock) + Send + Sync> {
        let callback = self.0.clone();
        Box::new(move |component_list| {
            let component_list = component_list.read();
            let components = component_list.get_components();

            callback(
                T1::from_component_ref(components[0]).unwrap(),
                T2::from_component_ref(components[1]).unwrap(),
                T3::from_component_ref(components[2]).unwrap(),
            )
        })
    }
}

impl<T1: Component, T2: Component, T3: Component> EntityQueryWriteCallback
    for EntityQueryWriteCallback3<T1, T2, T3>
{
    fn inject_components(&self) -> Box<dyn Fn(ComponentListRwLock) + Send + Sync> {
        let callback = self.0.clone();
        Box::new(move |component_list| {
            let mut component_list = component_list.write();
            let mut components = component_list.get_components_mut();

            callback(
                T1::from_component_mut(components[0]).unwrap(),
                T2::from_component_mut(components[1]).unwrap(),
                T3::from_component_mut(components[2]).unwrap(),
            )
        })
    }
}

// TODO: Implements 2° entity callback
