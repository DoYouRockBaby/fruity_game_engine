use crate::error::JsError;
use rusty_v8 as v8;

pub fn exception_to_err_result<'s, T>(
    scope: &mut v8::HandleScope<'s>,
    exception: v8::Local<v8::Value>,
) -> Result<T, JsError> {
    let is_terminating_exception = scope.is_execution_terminating();
    let mut exception = exception;
    if is_terminating_exception {
        // TerminateExecution was called. Cancel exception termination so that the
        // exception can be created..
        scope.cancel_terminate_execution();

        // Maybe make a new exception object.
        if exception.is_null_or_undefined() {
            let message = v8::String::new(scope, "execution terminated").unwrap();
            exception = v8::Exception::error(scope, message);
        }
    }
    let js_error = JsError::from_v8_exception(scope, exception);
    if is_terminating_exception {
        // Re-enable exception termination.
        scope.terminate_execution();
    }
    Err(js_error)
}
