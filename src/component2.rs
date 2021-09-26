use std::sync::RwLock;
use std::slice::from_raw_parts;
use std::mem::size_of;
use std::any::Any;
use crate::component::component::Component;
use crate::component::component_rwlock::ComponentRwLock;

#[derive(Debug, Clone)]
pub struct Component2 {
    pub float1: f64,
}

impl Component for Component2 {
    fn get_component_type(&self) -> &str {
        "test.component2"
    }

    fn get_component_size(&self) -> usize {
        size_of::<RwLock<Self>>()
    }

    fn get_untyped_field(&self, property: &str) -> Option<&dyn Any> {
        match property {
            "float1" => Some(&self.float1),
            _ => None
        }
    }

    fn set_untyped_field(&mut self, property: &str, value: &dyn Any) {
        match property {
            "float1" => match value.downcast_ref::<f64>() {
                Some(value) => {
                    self.float1 = value.clone();
                }
                None => {
                    log::error!("Expected a f64 for property {:?}, got {:#?}", property, value);
                }
            },
            _ => log::error!("Trying to access an inexistant property named {} in the component {:#?}", property, self)
        }
    }

    fn encode(&self) -> Vec<u8> {
        let value: RwLock<Self> = RwLock::new(self.clone());

        unsafe {
            from_raw_parts((&value as *const RwLock<Self>) as *const u8, self.get_component_size())
                .to_vec()
        }
    }

    fn decoder(&self) -> fn(datas: &[u8]) -> ComponentRwLock {
        | data | {
            let (_head, body, _tail) = unsafe { data.align_to::<RwLock<Self>>() };
            ComponentRwLock::new(&body[0])
        }
    }
}