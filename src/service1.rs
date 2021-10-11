use fruity_ecs::service::service::Service;
use fruity_introspect::FieldInfo;
use fruity_introspect::Introspect;
use fruity_introspect::IntrospectError;
use fruity_introspect::MethodInfo;
use std::any::Any;

pub struct Service1 {
    incrementer: u32,
}

impl Service1 {
    pub fn new() -> Service1 {
        Service1 { incrementer: 0 }
    }

    pub fn increment(&mut self) {
        self.incrementer += 1;
    }

    pub fn increment_by(&mut self, more: u32) {
        self.incrementer += more;
    }

    pub fn value(&self) -> u32 {
        self.incrementer
    }
}

impl Introspect for Service1 {
    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }

    fn get_any_field(&self, _property: &str) -> Option<&dyn Any> {
        None
    }

    fn set_any_field(&mut self, _property: &str, _value: &dyn Any) {}

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![
            MethodInfo {
                name: "increment".to_string(),
                args: vec![],
                return_type: None,
            },
            MethodInfo {
                name: "increment_by".to_string(),
                args: vec!["u32".to_string()],
                return_type: None,
            },
            MethodInfo {
                name: "value".to_string(),
                args: vec![],
                return_type: Some("u32".to_string()),
            },
        ]
    }

    fn call_method(
        &self,
        name: &str,
        args: Vec<Box<dyn Any>>,
    ) -> Result<Box<dyn Any>, IntrospectError> {
        match name {
            "value" => {
                if args.len() != 0 {
                    return Err(IntrospectError::WrongNumberArguments {
                        have: args.len(),
                        expected: 0,
                    });
                }

                let result = self.value();
                Ok(Box::new(result))
            }
            unknown_function => {
                return Err(IntrospectError::UnknownMethod(unknown_function.to_string()));
            }
        }
    }

    fn call_method_mut(
        &mut self,
        name: &str,
        args: Vec<Box<dyn Any>>,
    ) -> Result<Box<dyn Any>, IntrospectError> {
        match name {
            "increment" => {
                if args.len() != 0 {
                    return Err(IntrospectError::WrongNumberArguments {
                        have: args.len(),
                        expected: 0,
                    });
                }

                let result = self.increment();
                Ok(Box::new(result))
            }
            "increment_by" => {
                if args.len() != 1 {
                    return Err(IntrospectError::WrongNumberArguments {
                        have: args.len(),
                        expected: 1,
                    });
                }

                let arg1 = match args.get(0).unwrap().downcast_ref::<u32>() {
                    Some(arg) => Ok(arg),
                    None => Err(IntrospectError::IncorrectArgument),
                }?;

                let result = self.increment_by(*arg1);
                Ok(Box::new(result))
            }
            "value" => {
                if args.len() != 0 {
                    return Err(IntrospectError::WrongNumberArguments {
                        have: args.len(),
                        expected: 0,
                    });
                }

                let result = self.value();
                Ok(Box::new(result))
            }
            unknown_function => {
                return Err(IntrospectError::UnknownMethod(unknown_function.to_string()));
            }
        }
    }
}

impl Service for Service1 {}
