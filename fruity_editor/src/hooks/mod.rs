pub use comp_state::topo;
pub use comp_state::use_state;
pub use comp_state::CloneState;
pub use comp_state::StateAccess;
use std::any::Any;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::DerefMut;

thread_local!(static GLOBALS: RefCell<HashMap<TypeId, Box<dyn Any>>> = RefCell::new(HashMap::new()));

pub fn declare_global<T: 'static>(value: T) {
    GLOBALS.with(|globals| {
        let mut globals = globals.borrow_mut();
        globals.insert(TypeId::of::<T>(), Box::new(value));
    });
}

pub fn use_global<T: 'static>() -> &'static mut T {
    let globals = GLOBALS.with(|globals| {
        // TODO: Try to find a way to remove that
        let globals = unsafe {
            std::mem::transmute::<
                &mut HashMap<TypeId, Box<dyn Any>>,
                &mut HashMap<TypeId, Box<dyn Any>>,
            >(&mut globals.borrow_mut())
        };

        if globals.contains_key(&TypeId::of::<T>()) {
            globals.get_mut(&TypeId::of::<T>()).unwrap().deref_mut()
        } else {
            globals.get_mut(&TypeId::of::<T>()).unwrap().deref_mut()
        }
    });

    globals.downcast_mut::<T>().unwrap()
}
