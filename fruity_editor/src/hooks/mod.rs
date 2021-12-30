pub use comp_state::topo;
pub use comp_state::use_state;
pub use comp_state::CloneState;
pub use comp_state::StateAccess;
use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::Mutex;

lazy_static! {
    static ref GLOBALS: Mutex<HashMap<TypeId, Box<dyn Any + Send + Sync>>> =
        Mutex::new(HashMap::new());
}

pub fn declare_global<T: Send + Sync + 'static>(value: T) {
    let mut globals = GLOBALS.lock().unwrap();
    globals.insert(TypeId::of::<T>(), Box::new(value));
}

pub fn use_global<'a, T: Send + Sync + 'static>() -> &'a mut T {
    let mut globals = GLOBALS.lock().unwrap();
    let globals = globals.get_mut(&TypeId::of::<T>()).unwrap().deref_mut();
    let result = globals.downcast_mut::<T>().unwrap();

    // TODO: Try to find a way to remove that
    unsafe { std::mem::transmute::<&mut T, &mut T>(result) }
}

pub fn use_memo<T: Clone + 'static, U: Clone + Eq + 'static>(
    data_fn: impl Fn(U) -> T,
    dependency: U,
) -> T {
    let value_state = use_state(|| data_fn(dependency.clone()));
    let dependency_state = use_state(|| dependency.clone());

    if dependency != dependency_state.get() {
        value_state.set(data_fn(dependency.clone()));
    }

    value_state.get()
}
