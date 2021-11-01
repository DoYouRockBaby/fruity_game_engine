use crate::configure_constructors;
use crate::configure_services;
use crate::error::log_js_error;
use crate::JsRuntime;
use fruity_any::*;
use fruity_core::service::service::Service;
use fruity_core::service::service_manager::ServiceManager;
use fruity_core::service::single_thread_service::SingleThreadService;
use fruity_introspect::serialized::Serialized;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Clone, Copy)]
pub struct CallbackIdentifier(pub i32);

#[derive(Debug, FruityAny)]
pub struct JavascriptEngine {
    single_thread_service: SingleThreadService<JsRuntime>,
}

impl JavascriptEngine {
    pub fn new(service_manager: &Arc<RwLock<ServiceManager>>) -> JavascriptEngine {
        let service_manager = service_manager.clone();
        let single_thread_service = SingleThreadService::<JsRuntime>::start(move || {
            let mut runtime = JsRuntime::new();
            configure_services(&mut runtime, service_manager.clone());
            configure_constructors(&mut runtime, service_manager.clone());

            runtime
        });

        JavascriptEngine {
            single_thread_service,
        }
    }

    pub fn run_script(&self, source: &str) {
        let source = source.to_string();

        self.single_thread_service
            .call(move |runtime| match runtime.run_script(&source) {
                Ok(_) => (),
                Err(err) => log_js_error(&err),
            });
    }

    pub fn run_module(&self, path: &str) {
        let path = path.to_string();

        self.single_thread_service
            .call(move |runtime| match runtime.run_module(&path) {
                Ok(_) => (),
                Err(err) => log_js_error(&err),
            });
    }

    pub fn run_callback(&self, identifier: CallbackIdentifier, args: Vec<Serialized>) {
        self.single_thread_service.call(move |runtime| {
            runtime.run_stored_callback(identifier, args.clone());
        });
    }
}

impl IntrospectObject for JavascriptEngine {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Service for JavascriptEngine {}
