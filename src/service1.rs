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

/*impl fruity_introspect::introspect::Introspect for Service1 {
    fn methods(&self) -> std::vec::Vec<fruity_introspect::function::Method> {
        vec![
            fruity_introspect::function::Method {
                name: "increment".to_string(),
            },
            fruity_introspect::function::Method {
                name: "increment_by".to_string(),
            },
            fruity_introspect::function::Method {
                name: "value".to_string(),
            },
        ]
    }

    fn call_method(
        &self,
        name: &str,
        args: Vec<Box<dyn std::any::Any>>,
    ) -> Result<Box<dyn std::any::Any>, fruity_introspect::error::IntrospectError> {
        match name {
            "value" => {
                if args.len() != 0 {
                    return Err(
                        fruity_introspect::error::IntrospectError::WrongNumberArguments {
                            have: args.len(),
                            expected: 0,
                        },
                    );
                }

                let result = self.value();
                Ok(Box::new(result))
            }
            unknown_function => {
                return Err(fruity_introspect::error::IntrospectError::UnknownMethod(
                    unknown_function.to_string(),
                ));
            }
        }
    }

    fn call_method_mut(
        &mut self,
        name: &str,
        args: Vec<Box<dyn std::any::Any>>,
    ) -> Result<Box<dyn std::any::Any>, fruity_introspect::error::IntrospectError> {
        match name {
            "increment" => {
                if args.len() != 0 {
                    return Err(
                        fruity_introspect::error::IntrospectError::WrongNumberArguments {
                            have: args.len(),
                            expected: 0,
                        },
                    );
                }

                let result = self.increment();
                Ok(Box::new(result))
            }
            "increment_by" => {
                if args.len() != 1 {
                    return Err(
                        fruity_introspect::error::IntrospectError::WrongNumberArguments {
                            have: args.len(),
                            expected: 1,
                        },
                    );
                }

                let arg1 = match args.get(0).unwrap().downcast_ref::<u32>() {
                    Some(arg) => Ok(arg),
                    None => Err(fruity_introspect::error::IntrospectError::IncorrectArgument),
                }?;

                let result = self.increment_by(*arg1);
                Ok(Box::new(result))
            }
            "value" => {
                if args.len() != 0 {
                    return Err(
                        fruity_introspect::error::IntrospectError::WrongNumberArguments {
                            have: args.len(),
                            expected: 0,
                        },
                    );
                }

                let result = self.value();
                Ok(Box::new(result))
            }
            unknown_function => {
                return Err(fruity_introspect::error::IntrospectError::UnknownMethod(
                    unknown_function.to_string(),
                ));
            }
        }
    }
}*/
