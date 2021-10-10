use rusty_v8 as v8;

use std::convert::TryInto;

#[derive(Debug)]
pub enum JsError {
    CompileError,
    ImportModuleWithoutPrefix(String),
    FileNotFound(String),
    RuntimeError { message: String, stack: String },
}

impl JsError {
    pub fn from_v8_exception(scope: &mut v8::HandleScope, exception: v8::Local<v8::Value>) -> Self {
        // The exception is a JS Error object.
        let exception: v8::Local<v8::Object> = exception.try_into().unwrap();

        let message = get_property(scope, exception, "message");
        let message: Option<v8::Local<v8::String>> = message.and_then(|s| s.try_into().ok());
        let message = message.map(|s| s.to_rust_string_lossy(scope)).unwrap();

        let stack = get_property(scope, exception, "stack");
        let stack: Option<v8::Local<v8::String>> = stack.and_then(|s| s.try_into().ok());
        let stack = match stack {
            Some(s) => s.to_rust_string_lossy(scope),
            None => "".to_string(),
        };

        JsError::RuntimeError { message, stack }
    }
}

fn get_property<'a>(
    scope: &mut v8::HandleScope<'a>,
    object: v8::Local<v8::Object>,
    key: &str,
) -> Option<v8::Local<'a, v8::Value>> {
    let key = v8::String::new(scope, key).unwrap();
    let test = object.get(scope, key.into());
    test
}

pub fn log_js_error(err: &JsError) {
    match err {
        JsError::CompileError => log::error!("Failed to compile the module"),
        JsError::ImportModuleWithoutPrefix(module) => log::error!(
            "Wrong module format \"{}\", you should use a prefix like ../, ./ or /",
            module
        ),
        JsError::FileNotFound(file) => log::error!("File not found \"{}\"", file),
        JsError::RuntimeError { message, stack } => {
            log::error!("Javascript Exception: {}\n{:#?}", message, stack)
        }
    }
}
