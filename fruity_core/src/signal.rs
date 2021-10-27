use crate::service::utils::cast_service_mut;
use crate::service::utils::ArgumentCaster;
use fruity_any::FruityAny;
use fruity_introspect::log_introspect_error;
use fruity_introspect::serializable_object::SerializableObject;
use fruity_introspect::serialized::Callback;
use fruity_introspect::serialized::Serialized;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::Mutex;

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

/// An identifier for the observer, it can be used to unsubscribe to a signal
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObserverIdentifier(usize);

struct InternSignal<T: Into<Serialized> + Debug + Clone + 'static> {
    observers: Vec<(ObserverIdentifier, Box<dyn Fn(&T) + Sync + Send>)>,
}

/// An observer pattern
#[derive(Clone)]
pub struct Signal<T: Into<Serialized> + Debug + Clone + 'static> {
    intern: Arc<Mutex<InternSignal<T>>>,
}

impl<T: Into<Serialized> + Debug + Clone + 'static> Signal<T> {
    /// Returns a Signal
    pub fn new() -> Signal<T> {
        Signal {
            intern: Arc::new(Mutex::new(InternSignal {
                observers: Vec::new(),
            })),
        }
    }

    /// Add an observer to the signal
    /// An observer is a closure that will be called when the signal will be sent
    pub fn add_observer<F: Fn(&T) + Sync + Send + 'static>(
        &self,
        observer: F,
    ) -> ObserverIdentifier {
        let mut intern = self.intern.lock().unwrap();

        let mut id_generator = ID_GENERATOR.lock().unwrap();
        let identifier = ObserverIdentifier(id_generator.generate_id());
        intern.observers.push((identifier, Box::new(observer)));

        identifier
    }

    /// Notify that the event happened
    /// This will launch all the observers that are registered for this signal
    pub fn notify(&self, event: T) {
        let intern = self.intern.lock().unwrap();
        intern
            .observers
            .iter()
            .for_each(|(_, observer)| observer(&event));
    }
}

impl<T: Into<Serialized> + Debug + Clone + 'static> IntrospectObject for Signal<T> {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![MethodInfo {
            name: "add_observer".to_string(),
            call: MethodCaller::Mut(Arc::new(|this, args| {
                let this = cast_service_mut::<Signal<T>>(this);

                let mut caster = ArgumentCaster::new("add_observer", args);
                let arg1 = caster.cast_next::<Callback>()?;

                this.add_observer(move |arg| {
                    let arg: Serialized = arg.clone().into();
                    match arg1(vec![arg]) {
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

impl<T: Into<Serialized> + Debug + Clone + 'static> SerializableObject for Signal<T> {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

// TODO: Improve the macro to handle the generics
impl<T: Into<Serialized> + Debug + Clone + 'static> FruityAny for InternSignal<T> {
    fn as_any_ref(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }

    fn as_any_arc(self: std::sync::Arc<Self>) -> std::sync::Arc<dyn std::any::Any + Send + Sync> {
        self
    }
}

// TODO: Improve the macro to handle the generics
impl<T: Into<Serialized> + Debug + Clone + 'static> FruityAny for Signal<T> {
    fn as_any_ref(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }

    fn as_any_arc(self: std::sync::Arc<Self>) -> std::sync::Arc<dyn std::any::Any + Send + Sync> {
        self
    }
}

/*impl<T: Into<Serialized> + Debug + Clone + 'static> TryFrom<Serialized> for Signal<T> {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => {
                match value.clone().as_any_box().downcast::<Arc<Signal<T>>>() {
                    Ok(value) => match value.as_any_arc().downcast::<T>() {
                        Ok(value) => Ok(ResourceReference::from_resource(value)),
                        _ => Err(format!("Couldn't convert a Serialized to native object")),
                    },
                    _ => Err(format!("Couldn't convert a Serialized to native object")),
                }
            }
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}*/

impl<T: Into<Serialized> + Debug + Clone + 'static> Into<Serialized> for Signal<T> {
    fn into(self) -> Serialized {
        Serialized::NativeObject(Box::new(self))
    }
}

impl<T: Into<Serialized> + Debug + Clone + 'static> Debug for Signal<T> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
