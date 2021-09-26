use std::sync::RwLock;
use std::any::TypeId;
use std::any::type_name;
use std::any::Any;
use std::fmt::Debug;

pub trait Component: Debug + Any {
    fn get_component_type(&self) -> &str;
    fn get_component_size(&self) -> usize;
    fn get_untyped_field(&self, property: &str) -> Option<&dyn Any>;
    fn set_untyped_field(&mut self, property: &str, value: &dyn Any);
    fn encode(&self) -> Vec<u8>;
    fn decoder(&self) -> fn(datas: &[u8]) -> &RwLock<dyn Component>;
}

impl dyn Component {
    pub fn get_field<T: Any>(&self, property: &str) -> Option<&T> {
        match self.get_untyped_field(property) {
            Some(value) => match value.downcast_ref::<T>() {
                Some(value) => {
                    Some(value)
                }
                None => {
                    log::error!("Try to get a {:?} from property {:?}, got {:?}", type_name::<T>(), property, value);
                    None
                }
            },
            None => None,
        }
    }

    pub fn set_field<T: Any>(&mut self, property: &str, value: T) {
        self.set_untyped_field(property, &value);
    }
    
    pub fn is<T: Component>(&self) -> bool {
        // Get `TypeId` of the type this function is instantiated with.
        let t = TypeId::of::<T>();

        // Get `TypeId` of the type in the trait object (`self`).
        let concrete = self.type_id();

        // Compare both `TypeId`s on equality.
        t == concrete
    }

    pub fn downcast_ref<T: Component>(&self) -> Option<&T> {
        if self.is::<T>() {
            // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
            // that check for memory safety because we have implemented Component for all types; no other
            // impls can exist as they would conflict with our impl.
            unsafe { Some(&*(self as *const dyn Component as *const T)) }
        } else {
            None
        }
    }

    pub fn downcast_mut<T: Component>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
            // that check for memory safety because we have implemented Component for all types; no other
            // impls can exist as they would conflict with our impl.
            unsafe { Some(&mut *(self as *mut dyn Component as *mut T)) }
        } else {
            None
        }
    }
}