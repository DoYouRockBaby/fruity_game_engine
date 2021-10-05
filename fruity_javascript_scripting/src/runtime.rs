use crate::error::JsError;
use crate::modules::module_resolve_callback;
use crate::modules::ModuleMap;
use crate::modules::ModuleSpecifier;
use crate::value::JsResult;
use rusty_v8 as v8;
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::forget;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

pub type ModuleId = i32;

#[derive(Default, Clone)]
pub struct SharedArrayBufferStore(Arc<Mutex<SharedArrayBufferStoreInner>>);

#[derive(Default)]
pub struct SharedArrayBufferStoreInner {
  buffers: HashMap<u32, v8::SharedRef<v8::BackingStore>>,
  last_id: u32,
}

pub struct JsRuntime {
  v8_isolate: Option<v8::OwnedIsolate>,
}

/// Internal state for JsRuntime which is stored in one of v8::Isolate's
/// embedder slots.
pub(crate) struct JsRuntimeState {
  pub global_context: Option<v8::Global<v8::Context>>,
}

impl JsRuntime {
  pub fn new() -> JsRuntime {
    // Initialize V8
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    // Create isolate
    let params = v8::Isolate::create_params().heap_limits(0, 3 * 1024 * 1024);
    let isolate = v8::Isolate::new(params);
    let mut isolate = JsRuntime::setup_isolate(isolate);

    // Create context
    let global_context = {
      let scope = &mut v8::HandleScope::new(&mut isolate);
      let context = v8::Context::new(scope);
      v8::Global::new(scope, context)
    };

    isolate.set_slot(Rc::new(RefCell::new(JsRuntimeState {
      global_context: Some(global_context),
    })));

    let module_map = ModuleMap::new();
    isolate.set_slot(Rc::new(RefCell::new(module_map)));

    JsRuntime {
      v8_isolate: Some(isolate),
    }
  }

  pub fn handle_scope(&mut self) -> v8::HandleScope {
    let context = self.global_context();
    v8::HandleScope::with_context(self.v8_isolate(), context)
  }

  pub fn global_context(&mut self) -> v8::Global<v8::Context> {
    let state = Self::state(self.v8_isolate());
    let state = state.borrow();
    state.global_context.clone().unwrap()
  }

  pub fn v8_isolate(&mut self) -> &mut v8::OwnedIsolate {
    self.v8_isolate.as_mut().unwrap()
  }

  pub(crate) fn state(isolate: &v8::Isolate) -> Rc<RefCell<JsRuntimeState>> {
    let s = isolate.get_slot::<Rc<RefCell<JsRuntimeState>>>().unwrap();
    s.clone()
  }

  fn setup_isolate(mut isolate: v8::OwnedIsolate) -> v8::OwnedIsolate {
    /*isolate.set_capture_stack_trace_for_uncaught_exceptions(true, 10);
    isolate.set_promise_reject_callback(bindings::promise_reject_callback);
    isolate.set_host_initialize_import_meta_object_callback(
      bindings::host_initialize_import_meta_object_callback,
    );
    isolate.set_host_import_module_dynamically_callback(
      bindings::host_import_module_dynamically_callback,
    );*/
    isolate
  }

  pub fn run_script(&mut self, source: &str) -> Result<JsResult, JsError> {
    // Enter the context for compiling and running the script
    let mut scope = self.handle_scope();

    // Prepare the sources
    let source = v8::String::new(&mut scope, &source).unwrap();

    // Compile and run the script
    let script = if let Some(script) = v8::Script::compile(&mut scope, source, None) {
      script
    } else {
      return Err(JsError::CompileError);
    };

    let result = script.run(&mut scope);
    Ok(JsResult::new(scope, result))
  }

  pub fn module_map(isolate: &v8::Isolate) -> Rc<RefCell<ModuleMap>> {
    let module_map = isolate.get_slot::<Rc<RefCell<ModuleMap>>>().unwrap();
    module_map.clone()
  }

  /// Load specified module and all of its dependencies.
  ///
  /// The module will be marked as "main", and because of that
  /// "import.meta.main" will return true when checked inside that module.
  ///
  /// User must call `JsRuntime::mod_evaluate` with returned `ModuleId`
  /// manually after load is finished.
  pub fn load_main_module(&mut self, specifier: &ModuleSpecifier) -> Result<ModuleId, JsError> {
    let module_map_rc = Self::module_map(self.v8_isolate());
    let mut load = ModuleMap::load_main(module_map_rc.clone(), specifier.as_str())?;

    if let Ok(info) = load.module_source() {
      let scope = &mut self.handle_scope();
      load.register_and_recurse(scope, &info)?;
    }

    let root_id = load.root_module_id.expect("Root module should be loaded");
    self.instantiate_module(root_id)?;
    Ok(root_id)
  }

  pub fn instantiate_module(&mut self, id: ModuleId) -> Result<(), JsError> {
    let module_map_rc = Self::module_map(self.v8_isolate());
    let scope = &mut self.handle_scope();
    let tc_scope = &mut v8::TryCatch::new(scope);

    let module = module_map_rc
      .borrow()
      .get_handle(id)
      .map(|handle| v8::Local::new(tc_scope, handle))
      .expect("ModuleInfo not found");

    // IMPORTANT: No borrows to `ModuleMap` can be held at this point because
    // `module_resolve_callback` will be calling into `ModuleMap` from within
    // the isolate.
    let instantiate_result = module.instantiate_module(tc_scope, module_resolve_callback);

    if instantiate_result.is_none() {
      let exception = module.get_exception();
      let err = JsError::from_v8_exception(tc_scope, exception);
      return Err(err);
    }

    Ok(())
  }

  /// Evaluates an already instantiated ES module.
  ///
  /// This function panics if module has not been instantiated.
  pub fn mod_evaluate(&mut self, id: ModuleId) -> Result<JsResult, JsError> {
    let module_map_rc = Self::module_map(self.v8_isolate());
    let mut scope = self.handle_scope();

    let module = {
      module_map_rc
        .borrow()
        .get_handle(id)
        .map(|handle| v8::Local::new(&mut scope, handle))
        .expect("ModuleInfo not found")
    };

    let result = module.evaluate(&mut scope);
    Ok(JsResult::new(scope, result))
  }

  #[inline(always)]
  pub fn set_func(&mut self, name: &'static str, callback: impl v8::MapFnTo<v8::FunctionCallback>) {
    let mut scope = self.handle_scope();
    let context = v8::Context::new(&mut scope);
    let global = context.global(&mut scope);

    let key = v8::String::new(&mut scope, name).unwrap();
    let tmpl = v8::FunctionTemplate::new(&mut scope, callback);
    let val = tmpl.get_function(&mut scope).unwrap();

    global.set(&mut scope, key.into(), val.into());
  }
}

impl Drop for JsRuntime {
  fn drop(&mut self) {
    // TODO(ry): in rusty_v8, `SnapShotCreator::get_owned_isolate()` returns
    // a `struct OwnedIsolate` which is not actually owned, hence the need
    // here to leak the `OwnedIsolate` in order to avoid a double free and
    // the segfault that it causes.
    let v8_isolate = self.v8_isolate.take().unwrap();
    forget(v8_isolate);

    // Dispose V8
    unsafe {
      v8::V8::dispose();
    }
    v8::V8::shutdown_platform();
  }
}
