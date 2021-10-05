use crate::error::JsError;
use rusty_v8 as v8;

pub fn report_exceptions(mut try_catch: v8::TryCatch<v8::HandleScope>) {
    let exception = try_catch.exception().unwrap();
    let exception_string = exception
        .to_string(&mut try_catch)
        .unwrap()
        .to_rust_string_lossy(&mut try_catch);
    
    let message = if let Some(message) = try_catch.message() {
        message
    } else {
        eprintln!("{}", exception_string);
        return;
    };

    // Print (filename):(line number): (message).
    let filename = message
        .get_script_resource_name(&mut try_catch)
        .map_or_else(
            || "(unknown)".into(),
            |s| {
                s.to_string(&mut try_catch)
                .unwrap()
                .to_rust_string_lossy(&mut try_catch)
            },
        );
    
    let line_number = message.get_line_number(&mut try_catch).unwrap_or_default();

    eprintln!("{}:{}: {}", filename, line_number, exception_string);

    // Print line of source code.
    let source_line = message
        .get_source_line(&mut try_catch)
        .map(|s| {
            s.to_string(&mut try_catch)
            .unwrap()
            .to_rust_string_lossy(&mut try_catch)
        })
        .unwrap();
    
    eprintln!("{}", source_line);

    // Print wavy underline (GetUnderline is deprecated).
    let start_column = message.get_start_column();
    let end_column = message.get_end_column();

    for _ in 0..start_column {
        eprint!(" ");
    }

    for _ in start_column..end_column {
        eprint!("^");
    }

    eprintln!();

    // Print stack trace
    let stack_trace = if let Some(stack_trace) = try_catch.stack_trace() {
        stack_trace
    } else {
        return;
    };

    let stack_trace = unsafe { v8::Local::<v8::String>::cast(stack_trace) };
    let stack_trace = stack_trace
        .to_string(&mut try_catch)
        .map(|s| s.to_rust_string_lossy(&mut try_catch));

    if let Some(stack_trace) = stack_trace {
        eprintln!("{}", stack_trace);
    }
}

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