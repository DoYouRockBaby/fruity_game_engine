use std::sync::RwLock;
use std::mem::size_of;
use std::any::Any;
use crate::component::component::Component;

#[derive(Debug, Clone)]
pub struct Component1 {
    pub str1: String,
    pub int1: i64,
}

impl Component for Component1 {
    fn get_component_type(&self) -> &str {
        "test.component1"
    }

    fn get_untyped_field(&self, property: &str) -> Option<&dyn Any> {
        match property {
            "str1" => Some(&self.str1),
            "int1" => Some(&self.int1),
            _ => None
        }
    }

    fn set_untyped_field(&mut self, property: &str, value: &dyn Any) {
        match property {
            "str1" => match value.downcast_ref::<String>() {
                Some(value) => {
                    self.str1 = value.clone();
                }
                None => {
                    log::error!("Expected a String for property {:?}, got {:#?}", property, value);
                }
            },
            "int1" => match value.downcast_ref::<i64>() {
                Some(value) => {
                    self.int1 = value.clone();
                }
                None => {
                    log::error!("Expected a i64 for property {:?}, got {:#?}", property, value);
                }
            },
            _ => log::error!("Trying to access an inexistant property named {} in the component {:#?}", property, self)
        }
    }
}