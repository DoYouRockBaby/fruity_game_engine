use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SignalIdentifier(usize);

struct InternSignal<T> {
    incrementer: usize,
    observers: HashMap<SignalIdentifier, Box<dyn Fn(&T) + Sync + Send>>,
}

#[derive(Clone)]
pub struct Signal<T> {
    inter: Arc<Mutex<InternSignal<T>>>,
}

impl<T> Signal<T> {
    pub fn new() -> Signal<T> {
        Signal {
            inter: Arc::new(Mutex::new(InternSignal {
                incrementer: 0,
                observers: HashMap::new(),
            })),
        }
    }

    pub fn add_observer<F: Fn(&T) + Sync + Send + 'static>(&self, observer: F) -> SignalIdentifier {
        let mut inter = self.inter.lock().unwrap();

        let identifier = SignalIdentifier(inter.incrementer);
        inter.observers.insert(identifier, Box::new(observer));
        inter.incrementer += 1;

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
