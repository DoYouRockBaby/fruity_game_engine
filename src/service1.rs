use fruity_any_derive::*;
use fruity_core::service::Service;
use fruity_introspect::IntrospectError;
use fruity_introspect::IntrospectMethods;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;

#[derive(Debug, Clone, FruityAny)]
pub struct Service1 {
    incrementer: i32,
}

impl Service1 {
    pub fn new() -> Service1 {
        Service1 { incrementer: 0 }
    }

    pub fn increment(&mut self) {
        self.incrementer += 1;
    }

    pub fn increment_by(&mut self, more: i32) {
        self.incrementer += more;
    }

    pub fn value(&self) -> i32 {
        self.incrementer
    }
}

impl IntrospectMethods for Service1 {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![
            MethodInfo {
                name: "increment".to_string(),
                args: vec![],
                return_type: None,
                call: MethodCaller::Mut(|this, args| {
                    let this = this.downcast_mut::<Service1>().unwrap();

                    if args.len() != 0 {
                        return Err(IntrospectError::WrongNumberArguments {
                            have: args.len(),
                            expected: 0,
                        });
                    }

                    this.increment();
                    Ok(None)
                }),
            },
            MethodInfo {
                name: "increment_by".to_string(),
                args: vec!["u32".to_string()],
                return_type: None,
                call: MethodCaller::Mut(|this, args| {
                    let this = this.downcast_mut::<Service1>().unwrap();

                    if args.len() != 1 {
                        return Err(IntrospectError::WrongNumberArguments {
                            have: args.len(),
                            expected: 0,
                        });
                    }

                    let arg1 = match args.get(0).unwrap().downcast_ref::<i32>() {
                        Some(arg) => Ok(arg),
                        None => Err(IntrospectError::IncorrectArgument),
                    }?;

                    this.increment_by(*arg1);
                    Ok(None)
                }),
            },
            MethodInfo {
                name: "value".to_string(),
                args: vec![],
                return_type: Some("u32".to_string()),
                call: MethodCaller::Const(|this, args| {
                    let this = this.downcast_ref::<Service1>().unwrap();

                    if args.len() != 0 {
                        return Err(IntrospectError::WrongNumberArguments {
                            have: args.len(),
                            expected: 0,
                        });
                    }

                    let result = this.value();
                    Ok(Some(Box::new(result)))
                }),
            },
        ]
    }
}

impl Service for Service1 {}
