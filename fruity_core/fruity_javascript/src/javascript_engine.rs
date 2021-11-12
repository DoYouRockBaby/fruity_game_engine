use crate::bridge::constructors::configure_constructors;
use crate::bridge::resource_manager::configure_resource_manager;
use crate::error::log_js_error;
use crate::JsRuntime;
use fruity_any::*;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_manager::ResourceManager;
use fruity_core::utils::single_thread_wrapper::SingleThreadWrapper;
use fruity_introspect::serialized::Serialized;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use std::sync::Arc;

#[derive(Clone, Copy)]
pub struct CallbackIdentifier(pub i32);

#[derive(Debug, FruityAny)]
pub struct JavascriptEngine {
    single_thread_wrapper: SingleThreadWrapper<JsRuntime>,
}

impl JavascriptEngine {
    pub fn new(resource_manager: Arc<ResourceManager>) -> JavascriptEngine {
        let single_thread_wrapper = SingleThreadWrapper::<JsRuntime>::start(move || {
            let mut runtime = JsRuntime::new();
            configure_constructors(&mut runtime, resource_manager.clone());
            configure_resource_manager(&mut runtime, resource_manager.clone());

            runtime
        });

        JavascriptEngine {
            single_thread_wrapper,
        }
    }

    pub fn run_script(&self, source: &str) {
        let source = source.to_string();

        self.single_thread_wrapper
            .call(move |runtime| match runtime.run_script(&source) {
                Ok(_) => (),
                Err(err) => log_js_error(&err),
            });
    }

    pub fn run_module(&self, path: &str) {
        let path = path.to_string();

        self.single_thread_wrapper
            .call(move |runtime| match runtime.run_module(&path) {
                Ok(_) => (),
                Err(err) => log_js_error(&err),
            });
    }

    pub fn run_callback(&self, identifier: CallbackIdentifier, args: Vec<Serialized>) {
        self.single_thread_wrapper.call(move |runtime| {
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

impl Resource for JavascriptEngine {}
