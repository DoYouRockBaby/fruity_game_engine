use crate::bridge::constructors::configure_constructors;
use crate::bridge::resource_container::configure_resource_container;
use crate::error::log_js_error;
use crate::JsRuntime;
use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::serialize::serialized::Serialized;
use fruity_core::utils::single_thread_wrapper::SingleThreadWrapper;

#[derive(Clone, Copy)]
pub struct CallbackIdentifier(pub i32);

#[derive(Debug, FruityAny)]
pub struct JavascriptService {
    single_thread_wrapper: SingleThreadWrapper<JsRuntime, Option<Serialized>>,
}

impl JavascriptService {
    pub fn new(resource_container: ResourceContainer) -> JavascriptService {
        let single_thread_wrapper =
            SingleThreadWrapper::<JsRuntime, Option<Serialized>>::start(move || {
                let mut runtime = JsRuntime::new();
                configure_constructors(&mut runtime, resource_container.clone());
                configure_resource_container(&mut runtime, resource_container.clone());

                runtime
            });

        JavascriptService {
            single_thread_wrapper,
        }
    }

    pub fn run_script(&self, source: &str) {
        let source = source.to_string();

        self.single_thread_wrapper
            .call(move |runtime| match runtime.run_script(&source) {
                Ok(_) => None,
                Err(err) => {
                    log_js_error(&err);
                    None
                }
            });
    }

    pub fn run_module(&self, path: &str) {
        let path = path.to_string();

        self.single_thread_wrapper
            .call(move |runtime| match runtime.run_module(&path) {
                Ok(_) => None,
                Err(err) => {
                    log_js_error(&err);
                    None
                }
            });
    }

    pub fn run_callback(
        &self,
        identifier: CallbackIdentifier,
        args: Vec<Serialized>,
    ) -> Option<Serialized> {
        self.single_thread_wrapper
            .call(move |runtime| runtime.run_stored_callback(identifier, args.clone()))
    }

    pub fn reset(&self) {
        self.single_thread_wrapper.call(move |runtime| {
            runtime.reset();
            None
        });
    }
}

impl IntrospectObject for JavascriptService {
    fn get_class_name(&self) -> String {
        "JavascriptService".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for JavascriptService {}
