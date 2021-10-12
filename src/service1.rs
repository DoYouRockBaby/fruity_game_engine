use fruity_any_derive::*;
use fruity_core::service::Service;
use fruity_core::utils::assert_argument_count;
use fruity_core::utils::cast_argument;
use fruity_core::utils::cast_service;
use fruity_core::utils::cast_service_mut;
use fruity_introspect::IntrospectMethods;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use fruity_serialize::serialize::serialize_any;

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
                    let this = cast_service_mut::<Service1>(this);
                    assert_argument_count(0, &args)?;

                    this.increment();
                    Ok(None)
                }),
            },
            MethodInfo {
                name: "increment_by".to_string(),
                args: vec!["i32".to_string()],
                return_type: None,
                call: MethodCaller::Mut(|this, args| {
                    let this = cast_service_mut::<Service1>(this);
                    assert_argument_count(1, &args)?;

                    let arg1 = cast_argument(0, &args, |arg| arg.as_i32())?;

                    this.increment_by(arg1);
                    Ok(None)
                }),
            },
            MethodInfo {
                name: "value".to_string(),
                args: vec![],
                return_type: Some("i32".to_string()),
                call: MethodCaller::Const(|this, args| {
                    let this = cast_service::<Service1>(this);
                    assert_argument_count(0, &args)?;

                    let result = this.value();
                    Ok(serialize_any(&result))
                }),
            },
        ]
    }
}

impl Service for Service1 {}
