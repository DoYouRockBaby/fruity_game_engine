use fruity_any_derive::*;
use fruity_ecs::serialize::serialized::Serialized;
use fruity_ecs::service::service::Service;
use fruity_ecs::service::utils::cast_service;
use fruity_ecs::service::utils::cast_service_mut;
use fruity_ecs::service::utils::ArgumentCaster;
use fruity_introspect::IntrospectMethods;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use std::sync::Arc;

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

impl IntrospectMethods<Serialized> for Service1 {
    fn get_method_infos(&self) -> Vec<MethodInfo<Serialized>> {
        vec![
            MethodInfo {
                name: "increment".to_string(),
                call: MethodCaller::Mut(Arc::new(|this, _args| {
                    let this = cast_service_mut::<Service1>(this);
                    this.increment();
                    Ok(None)
                })),
            },
            MethodInfo {
                name: "increment_by".to_string(),
                call: MethodCaller::Mut(Arc::new(|this, args| {
                    let this = cast_service_mut::<Service1>(this);

                    let mut caster = ArgumentCaster::new("increment_by", args);
                    let arg1 = caster.cast_next(|arg| arg.as_i32())?;

                    this.increment_by(arg1);
                    Ok(None)
                })),
            },
            MethodInfo {
                name: "value".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_service::<Service1>(this);
                    let result = this.value();
                    Ok(Some(Serialized::I32(result)))
                })),
            },
        ]
    }
}

impl Service for Service1 {}
