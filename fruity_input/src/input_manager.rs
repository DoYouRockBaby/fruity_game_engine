use fruity_any::*;
use fruity_core::service::service::Service;
use fruity_core::service::service_manager::ServiceManager;
use fruity_core::service::utils::cast_service;
use fruity_core::service::utils::ArgumentCaster;
use fruity_core::signal::Signal;
use fruity_introspect::serialized::Serialized;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(FruityAny)]
pub struct InputManager {
    pub input_map: HashMap<String, String>,
    pub pressed_inputs: HashSet<String>,
    pub on_pressed: Signal<String>,
    pub on_released: Signal<String>,
}

impl Debug for InputManager {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl InputManager {
    pub fn new(_service_manager: &Arc<RwLock<ServiceManager>>) -> InputManager {
        InputManager {
            input_map: HashMap::new(),
            pressed_inputs: HashSet::new(),
            on_pressed: Signal::new(),
            on_released: Signal::new(),
        }
    }

    pub fn register_input(&mut self, source: &str, input: &str) {
        self.input_map.insert(source.to_string(), input.to_string());
    }

    pub fn is_pressed(&self, input: &str) -> bool {
        self.pressed_inputs.contains(input)
    }
}

impl IntrospectObject for InputManager {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![MethodInfo {
            name: "is_pressed".to_string(),
            call: MethodCaller::Const(Arc::new(|this, args| {
                let this = cast_service::<InputManager>(this);

                let mut caster = ArgumentCaster::new("is_pressed", args);
                let arg1 = caster.cast_next::<String>()?;

                let result = this.is_pressed(&arg1);
                Ok(Some(Serialized::Bool(result)))
            })),
        }]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Service for InputManager {}
