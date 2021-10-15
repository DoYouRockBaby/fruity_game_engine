use crate::bridge::console::configure_console;
use crate::error::JsError;
use crate::exception::exception_to_err_result;
use crate::js_value::object::JsObject;
use crate::module_map::ModuleInfos;
use crate::module_map::ModuleMap;
use crate::normalize_path::normalize_path;
use fruity_any_derive::*;
use fruity_ecs::serialize::serialized::Serialized;
use fruity_ecs::service::service::Service;
use fruity_introspect::IntrospectMethods;
use fruity_introspect::MethodInfo;
use rusty_v8 as v8;
use std::cell::RefCell;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug, FruityAny)]
pub struct JsRuntime {
  pub(crate) handles: Arc<Mutex<JsRuntimeHandles>>,
}

#[derive(Debug, FruityAny)]
pub struct JsRuntimeHandles {
  v8_isolate: Option<v8::OwnedIsolate>,
  global_context: v8::Global<v8::Context>,
}

unsafe impl Send for JsRuntimeHandles {}
unsafe impl Sync for JsRuntimeHandles {}

impl Drop for JsRuntimeHandles {
  fn drop(&mut self) {
    std::mem::forget(self.v8_isolate.take());
  }
}

impl JsRuntimeHandles {
  pub fn handle_scope(&mut self) -> v8::HandleScope {
    let context = self.global_context();
    v8::HandleScope::with_context(self.v8_isolate(), context)
  }

  pub fn global_context(&self) -> v8::Global<v8::Context> {
    self.global_context.clone()
  }

  pub fn v8_isolate(&mut self) -> &mut v8::OwnedIsolate {
    self.v8_isolate.as_mut().unwrap()
  }

  pub fn global_object(&mut self) -> JsObject {
    let global_context = self.global_context();
    let isolate = self.v8_isolate();
    let mut scope = v8::HandleScope::with_context(isolate, global_context.clone());

    let global = global_context.get(&mut scope).global(&mut scope);
    let global = v8::Global::new(&mut scope, global);
    JsObject::from_v8(global)
  }
}

impl Drop for JsRuntime {
  fn drop(&mut self) {
    std::mem::drop(self);

    unsafe {
      v8::V8::dispose();
    }
    v8::V8::shutdown_platform();
  }
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

    isolate.set_slot(Rc::new(RefCell::new(ModuleMap::new())));

    // Create the runtime
    let mut runtime = JsRuntime {
      handles: Arc::new(Mutex::new(JsRuntimeHandles {
        v8_isolate: Some(isolate),
        global_context,
      })),
    };

    configure_console(&mut runtime);

    runtime
  }

  fn setup_isolate(isolate: v8::OwnedIsolate) -> v8::OwnedIsolate {
    isolate
  }

  pub fn run_script(&mut self, source: &str) -> Result<(), JsError> {
    // Enter the context for compiling and running the script
    let mut handles = self.handles.lock().unwrap();
    let mut scope = handles.handle_scope();
    let mut try_catch = v8::TryCatch::new(&mut scope);

    // Prepare the sources
    let source = v8::String::new(&mut try_catch, &source).unwrap();

    // Compile and run the script
    let script = if let Some(script) = v8::Script::compile(&mut try_catch, source, None) {
      script
    } else {
      return Err(JsError::CompileError);
    };

    let result = script.run(&mut try_catch);

    if result.is_none() {
      let exception = try_catch.exception().unwrap();
      return exception_to_err_result(&mut try_catch, exception);
    }

    Ok(())
    //Ok(JsResult::new(scope, result))
  }

  #[allow(unused_must_use)]
  pub fn run_module(&mut self, filepath: &str) -> Result<(), JsError> {
    // Enter the context for compiling and running the script
    let mut handles = self.handles.lock().unwrap();
    let mut scope = handles.handle_scope();
    let mut try_catch = v8::TryCatch::new(&mut scope);

    // Create the module
    let module = compile_module(&mut try_catch, filepath)?;

    // Instantiate the module
    let result = module.instantiate_module(&mut try_catch, module_resolve_callback);
    if result.is_none() {
      let exception = try_catch.exception().unwrap();
      return exception_to_err_result(&mut try_catch, exception);
    }

    // Run the module
    module.evaluate(&mut try_catch);

    // Update status after evaluating.
    let status = module.get_status();

    if status == v8::ModuleStatus::Errored {
      let exception = module.get_exception();
      return exception_to_err_result(&mut try_catch, exception);
    }

    Ok(())
  }

  pub fn module_map(isolate: &v8::Isolate) -> Rc<RefCell<ModuleMap>> {
    let module_map = isolate.get_slot::<Rc<RefCell<ModuleMap>>>().unwrap();
    module_map.clone()
  }
}

/// Called by V8 during `JsRuntime::instantiate_module`.
///
/// This function borrows `ModuleMap` from the isolate slot,
/// so it is crucial to ensure there are no existing borrows
/// of `ModuleMap` when `JsRuntime::instantiate_module` is called.
pub fn module_resolve_callback<'s>(
  context: v8::Local<'s, v8::Context>,
  specifier: v8::Local<'s, v8::String>,
  _import_assertions: v8::Local<'s, v8::FixedArray>,
  referrer: v8::Local<'s, v8::Module>,
) -> Option<v8::Local<'s, v8::Module>> {
  let scope = &mut unsafe { v8::CallbackScope::new(context) };

  // Get included module path
  let referrer_directory = get_referrer_directory(scope, referrer);
  let included_module_path = get_specifier_filename(scope, specifier, &referrer_directory);

  // Create the module
  let module = match included_module_path {
    Ok(filepath) => match compile_module(scope, &filepath) {
      Ok(module) => Some(module),
      Err(_err) => None,
    },
    Err(_err) => None,
  };

  // Return the newly created module
  module
}

pub fn compile_module<'a>(
  scope: &mut v8::HandleScope<'a>,
  filepath: &str,
) -> Result<v8::Local<'a, v8::Module>, JsError> {
  let module_map_rc = JsRuntime::module_map(scope);
  let mut module_map = module_map_rc.borrow_mut();

  // Check if the filepath is already registered, if yes return the associated module
  if let Some((referrer_global, _)) = module_map.find_by_filepath(filepath) {
    return Ok(v8::Local::new(scope, referrer_global));
  }

  // Prepare sources
  let source_str = match std::fs::read_to_string(filepath) {
    Ok(code) => Ok(code),
    Err(_) => Err(JsError::FileNotFound(filepath.to_string())),
  }?;

  let source = v8::String::new(scope, &source_str).unwrap();

  let filepath_v8 = v8::String::new(scope, filepath).unwrap();
  let origin = module_origin(scope, filepath_v8);
  let source = v8::script_compiler::Source::new(source, Some(&origin));

  // Create the module
  let module = match v8::script_compiler::compile_module(scope, source) {
    Some(module) => Ok(module),
    None => Err(JsError::CompileError),
  }?;

  // Store in referrer hashmap
  let referrer_global = v8::Global::new(scope, module);
  module_map.insert(
    referrer_global,
    ModuleInfos {
      filepath: filepath.to_string(),
    },
  );

  Ok(module)
}

pub fn module_origin<'a>(
  scope: &mut v8::HandleScope<'a>,
  resource_name: v8::Local<'a, v8::String>,
) -> v8::ScriptOrigin<'a> {
  let source_map_url = v8::String::new(scope, "").unwrap();
  v8::ScriptOrigin::new(
    scope,
    resource_name.into(),
    0,
    0,
    false,
    123,
    source_map_url.into(),
    true,
    false,
    true,
  )
}

pub fn get_referrer_directory<'a>(
  scope: &mut v8::HandleScope<'a>,
  referrer: v8::Local<'a, v8::Module>,
) -> String {
  let module_map_rc = JsRuntime::module_map(scope);
  let module_map = module_map_rc.borrow();
  let referrer_global = v8::Global::new(scope, referrer);
  let module_info = module_map.get(&referrer_global).unwrap();
  let referrer_path = PathBuf::from(&module_info.filepath);
  let referrer_directory = referrer_path
    .parent()
    .unwrap()
    .to_str()
    .unwrap()
    .to_string();

  referrer_directory
}

pub fn get_specifier_filename<'a>(
  scope: &mut v8::HandleScope<'a>,
  specifier: v8::Local<'a, v8::String>,
  base: &str,
) -> Result<String, JsError> {
  let specifier = specifier.to_rust_string_lossy(scope);

  if specifier.starts_with("./") || specifier.starts_with("../") {
    let filepath = Path::new(base)
      .join(specifier)
      .to_str()
      .unwrap()
      .to_string();

    Ok(normalize_path(filepath).to_str().unwrap().to_string())
  } else {
    Err(JsError::ImportModuleWithoutPrefix(specifier.to_string()))
  }
}

impl IntrospectMethods<Serialized> for JsRuntime {
  fn get_method_infos(&self) -> Vec<MethodInfo<Serialized>> {
    vec![]
  }
}

impl Service for JsRuntime {}
