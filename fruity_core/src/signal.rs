use crate::convert::FruityInto;
use crate::convert::FruityTryFrom;
use crate::introspect::log_introspect_error;
use crate::introspect::FieldInfo;
use crate::introspect::IntrospectObject;
use crate::introspect::MethodCaller;
use crate::introspect::MethodInfo;
use crate::serialize::serialized::Callback;
use crate::serialize::serialized::SerializableObject;
use crate::serialize::serialized::Serialized;
use crate::utils::introspect::cast_introspect_mut;
use crate::utils::introspect::cast_introspect_ref;
use crate::utils::introspect::ArgumentCaster;
use crate::Mutex;
use crate::RwLock;
use fruity_any::*;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;

struct IdGenerator {
    incrementer: usize,
}

impl IdGenerator {
    pub fn new() -> IdGenerator {
        IdGenerator { incrementer: 0 }
    }

    pub fn generate_id(&mut self) -> usize {
        self.incrementer += 1;
        self.incrementer
    }
}

lazy_static! {
    static ref ID_GENERATOR: Mutex<IdGenerator> = Mutex::new(IdGenerator::new());
}

/// An identifier for a signal observer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObserverIdentifier(usize);

/// A signal subscription handler, can be used to unsubscribe the signal
#[derive(Clone, FruityAny)]
pub struct ObserverHandler<T: 'static> {
    observer_id: ObserverIdentifier,
    intern: Arc<RwLock<InternSignal<T>>>,
}

#[derive(FruityAny)]
struct InternSignal<T: 'static> {
    observers: Vec<(ObserverIdentifier, Arc<dyn Fn(&T) + Sync + Send>)>,
}

/// An observer pattern
#[derive(Clone, FruityAny)]
pub struct Signal<T: 'static> {
    intern: Arc<RwLock<InternSignal<T>>>,
}

impl<T> Default for Signal<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Signal<T> {
    /// Returns a Signal
    pub fn new() -> Signal<T> {
        Signal {
            intern: Arc::new(RwLock::new(InternSignal {
                observers: Vec::new(),
            })),
        }
    }

    /// Add an observer to the signal
    /// An observer is a closure that will be called when the signal will be sent
    pub fn add_observer<F: Fn(&T) + Sync + Send + 'static>(
        &self,
        observer: F,
    ) -> ObserverHandler<T> {
        let mut intern_writer = self.intern.write();

        let mut id_generator = ID_GENERATOR.lock();
        let observer_id = ObserverIdentifier(id_generator.generate_id());
        intern_writer
            .observers
            .push((observer_id, Arc::new(observer)));

        ObserverHandler {
            observer_id,
            intern: self.intern.clone(),
        }
    }

    /// Add an observer to the signal that can dispose itself
    /// An observer is a closure that will be called when the signal will be sent
    pub fn add_self_dispose_observer<F: Fn(&T, &ObserverHandler<T>) + Sync + Send + 'static>(
        &self,
        observer: F,
    ) {
        let mut intern_writer = self.intern.write();

        let mut id_generator = ID_GENERATOR.lock();
        let observer_id = ObserverIdentifier(id_generator.generate_id());

        let handler = ObserverHandler {
            observer_id,
            intern: self.intern.clone(),
        };

        intern_writer.observers.push((
            observer_id,
            Arc::new(move |data| {
                observer(data, &handler);
            }),
        ));
    }

    /// Notify that the event happened
    /// This will launch all the observers that are registered for this signal
    pub fn notify(&self, event: T) {
        let observers = {
            let intern = self.intern.read();
            intern.observers.clone()
        };

        observers.iter().for_each(|(_, observer)| observer(&event));
    }
}

impl<T: FruityInto<Serialized> + Debug + Clone + 'static> IntrospectObject for Signal<T> {
    fn get_class_name(&self) -> String {
        "Signal".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![MethodInfo {
            name: "add_observer".to_string(),
            call: MethodCaller::Mut(Arc::new(|this, args| {
                let this = cast_introspect_mut::<Signal<T>>(this);

                let mut caster = ArgumentCaster::new("add_observer", args);
                let arg1 = caster.cast_next::<Callback>()?;

                this.add_observer(move |arg| {
                    let arg: Serialized = arg.clone().fruity_into();
                    match (arg1.callback)(vec![arg]) {
                        Ok(_) => (),
                        Err(err) => log_introspect_error(&err),
                    };
                });

                Ok(None)
            })),
        }]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl<T: FruityInto<Serialized> + Debug + Clone + 'static> SerializableObject for Signal<T> {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

impl<T: FruityInto<Serialized> + Debug + Clone + 'static> FruityInto<Serialized> for Signal<T> {
    fn fruity_into(self) -> Serialized {
        Serialized::NativeObject(Box::new(self))
    }
}

impl<T> Debug for Signal<T> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

/// A write guard over a signal property, when it's dropped, the update signal is sent
pub struct SignalWriteGuard<'a, T: Send + Sync + Clone + 'static> {
    target: &'a mut SignalProperty<T>,
}

impl<'a, T: Send + Sync + Clone> Drop for SignalWriteGuard<'a, T> {
    fn drop(&mut self) {
        self.target.on_updated.notify(self.target.value.clone())
    }
}

impl<'a, T: Send + Sync + Clone> Deref for SignalWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.target.value
    }
}

impl<'a, T: Send + Sync + Clone> DerefMut for SignalWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.target.value
    }
}

/// A variable with a signal that is notified on update
#[derive(Clone, FruityAny)]
pub struct SignalProperty<T: Send + Sync + Clone + 'static> {
    value: T,

    /// A signal sent when the property is updated
    pub on_updated: Signal<T>,
}

impl<T: Send + Sync + Clone + Default> Default for SignalProperty<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Send + Sync + Clone> SignalProperty<T> {
    /// Returns a SignalProperty
    pub fn new(value: T) -> Self {
        Self {
            value,
            on_updated: Signal::new(),
        }
    }

    /// Returns a SignalProperty
    pub fn write(&mut self) -> SignalWriteGuard<T> {
        SignalWriteGuard::<T> { target: self }
    }
}

impl<T: Send + Sync + Clone> Deref for SignalProperty<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: FruityInto<Serialized> + Send + Sync + Debug + Clone + IntrospectObject> IntrospectObject
    for SignalProperty<T>
{
    fn get_class_name(&self) -> String {
        self.value.get_class_name()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![MethodInfo {
            name: "add_observer".to_string(),
            call: MethodCaller::Mut(Arc::new(|this, args| {
                let this = cast_introspect_mut::<Signal<T>>(this);

                let mut caster = ArgumentCaster::new("add_observer", args);
                let arg1 = caster.cast_next::<Callback>()?;

                let handle = this.add_observer(move |arg| {
                    let arg: Serialized = arg.clone().fruity_into();
                    match (arg1.callback)(vec![arg]) {
                        Ok(_) => (),
                        Err(err) => log_introspect_error(&err),
                    };
                });

                Ok(Some(handle.fruity_into()))
            })),
        }]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl<T: FruityInto<Serialized> + Send + Sync + Debug + Clone + IntrospectObject> SerializableObject
    for SignalProperty<T>
{
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

impl<T: FruityInto<Serialized> + Send + Sync + Debug + Clone + IntrospectObject>
    FruityInto<Serialized> for SignalProperty<T>
{
    fn fruity_into(self) -> Serialized {
        self.value.fruity_into()
    }
}

impl<T: FruityTryFrom<Serialized, Error = String> + Send + Sync + Debug + Clone + 'static>
    FruityTryFrom<Serialized> for SignalProperty<T>
{
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        match T::fruity_try_from(value) {
            Ok(value) => Ok(SignalProperty::new(value)),
            Err(err) => Err(err.to_string()),
        }
    }
}

impl<T: Send + Sync + Clone + Debug> Debug for SignalProperty<T> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        self.value.fmt(formatter)
    }
}

impl<T> ObserverHandler<T> {
    /// Remove an observer from the signal
    pub fn dispose(self) {
        let mut intern = self.intern.write();
        let observer_index = intern
            .observers
            .iter()
            .enumerate()
            .find(|(_index, elem)| elem.0 == self.observer_id)
            .map(|elem| elem.0);

        if let Some(observer_index) = observer_index {
            let _ = intern.observers.remove(observer_index);
        }
    }

    /// Remove an observer from the signal
    pub fn dispose_by_ref(&self) {
        let mut intern = self.intern.write();
        let observer_index = intern
            .observers
            .iter()
            .enumerate()
            .find(|(_index, elem)| elem.0 == self.observer_id)
            .map(|elem| elem.0);

        if let Some(observer_index) = observer_index {
            let _ = intern.observers.remove(observer_index);
        }
    }
}

impl<T> Debug for ObserverHandler<T> {
    fn fmt(&self, _: &mut Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<T> IntrospectObject for ObserverHandler<T> {
    fn get_class_name(&self) -> String {
        "ObserverHandler".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![MethodInfo {
            name: "dispose".to_string(),
            call: MethodCaller::Const(Arc::new(|this, _args| {
                let this = cast_introspect_ref::<ObserverHandler<T>>(this);
                this.dispose_by_ref();

                Ok(None)
            })),
        }]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl<T: 'static> SerializableObject for ObserverHandler<T> {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(Self {
            observer_id: self.observer_id,
            intern: self.intern.clone(),
        })
    }
}

impl<T> FruityInto<Serialized> for ObserverHandler<T> {
    fn fruity_into(self) -> Serialized {
        Serialized::NativeObject(Box::new(self))
    }
}
