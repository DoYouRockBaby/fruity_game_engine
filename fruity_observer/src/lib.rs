use std::fmt::Debug;
use std::sync::Arc;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SignalIdentifier(usize);

struct InternSignal<T> {
    observers: Vec<(SignalIdentifier, Box<dyn Fn(&T) + Sync + Send>)>,
}

#[derive(Clone)]
pub struct Signal<T> {
    inter: Arc<Mutex<InternSignal<T>>>,
}

impl<T> Signal<T> {
    pub fn new() -> Signal<T> {
        Signal {
            inter: Arc::new(Mutex::new(InternSignal {
                observers: Vec::new(),
            })),
        }
    }

    pub fn add_observer<F: Fn(&T) + Sync + Send + 'static>(&self, observer: F) -> SignalIdentifier {
        let mut inter = self.inter.lock().unwrap();

        let mut id_generator = ID_GENERATOR.lock().unwrap();
        let identifier = SignalIdentifier(id_generator.generate_id());
        inter.observers.push((identifier, Box::new(observer)));

        identifier
    }

    pub fn notify(&self, event: T) {
        let inter = self.inter.lock().unwrap();
        inter
            .observers
            .iter()
            .for_each(|(_, observer)| observer(&event));
    }
}

impl<T: Debug> Debug for Signal<T> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
