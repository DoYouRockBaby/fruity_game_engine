// Copyright 2018-2021 strongly made by the Deno authors. All rights reserved. MIT license.

use rusty_v8 as v8;

use crate::error::JsError;
use crate::exception::exception_to_err_result;
use crate::module_specifier::resolve_import;
use crate::module_specifier::resolve_url;
use crate::runtime::JsRuntime;
use log::debug;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::future::Future;
use std::rc::Rc;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;
use url::Url;

lazy_static::lazy_static! {
  pub static ref NEXT_LOAD_ID: AtomicI32 = AtomicI32::new(1);
}

pub type ModuleId = i32;
pub type ModuleLoadId = i32;
pub type ModuleSpecifier = Url;

/// EsModule source code that will be loaded into V8.
///
/// Users can implement `Into<ModuleInfo>` for different file types that
/// can be transpiled to valid EsModule.
///
/// Found module URL might be different from specified URL
/// used for loading due to redirections (like HTTP 303).
/// Eg. Both "`https://example.com/a.ts`" and
/// "`https://example.com/b.ts`" may point to "`https://example.com/c.ts`"
/// By keeping track of specified and found URL we can alias modules and avoid
/// recompiling the same code 3 times.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ModuleSource {
  pub code: String,
  pub module_url_specified: String,
  pub module_url_found: String,
}

/// Describes the entrypoint of a recursive module load.
#[derive(Debug)]
enum LoadInit {
  /// Main module specifier.
  Main(String),
  /// Module specifier for side module.
  Side(String),
  /// Dynamic import specifier with referrer.
  DynamicImport(String, String),
}

/// This future is used to implement parallel async module loading.
pub struct RecursiveModuleLoad {
  init: LoadInit,
  // TODO(bartlomieju): in future this value should
  // be randomized
  pub id: ModuleLoadId,
  pub root_module_id: Option<ModuleId>,
  pub module_map_rc: Rc<RefCell<ModuleMap>>,
  // These two fields are copied from `module_map_rc`, but they are cloned ahead
  // of time to avoid already-borrowed errors.
  pub visited: HashSet<ModuleSpecifier>,
}

impl RecursiveModuleLoad {
  /// Starts a new parallel load of the given URL of the main module.
  pub fn main(specifier: &str, module_map_rc: Rc<RefCell<ModuleMap>>) -> Self {
    Self::new(LoadInit::Main(specifier.to_string()), module_map_rc)
  }

  pub fn side(specifier: &str, module_map_rc: Rc<RefCell<ModuleMap>>) -> Self {
    Self::new(LoadInit::Side(specifier.to_string()), module_map_rc)
  }

  pub fn dynamic_import(
    specifier: &str,
    referrer: &str,
    module_map_rc: Rc<RefCell<ModuleMap>>,
  ) -> Self {
    let init = LoadInit::DynamicImport(specifier.to_string(), referrer.to_string());
    Self::new(init, module_map_rc)
  }

  pub fn is_dynamic_import(&self) -> bool {
    matches!(self.init, LoadInit::DynamicImport(..))
  }

  fn new(init: LoadInit, module_map_rc: Rc<RefCell<ModuleMap>>) -> Self {
    let module_id = NEXT_LOAD_ID.fetch_add(1, Ordering::SeqCst);

    let mut load = Self {
      id: module_id,
      root_module_id: Some(module_id),
      init,
      module_map_rc: module_map_rc.clone(),
      visited: HashSet::new(),
    };

    // Ignore the error here, let it be hit in `Stream::poll_next()`.
    if let Ok(root_specifier) = load.resolve_root() {
      if let Some(module_id) = module_map_rc.borrow().get_id(root_specifier.as_str()) {
        load.root_module_id = Some(module_id);
      }
    }
    load
  }

  pub fn resolve_root(&self) -> Result<ModuleSpecifier, JsError> {
    match self.init {
      LoadInit::Main(ref specifier) => resolve_import(specifier, "."),
      LoadInit::Side(ref specifier) => resolve_import(specifier, "."),
      LoadInit::DynamicImport(ref specifier, ref referrer) => resolve_import(specifier, referrer),
    }
  }

  pub fn prepare(&self) -> Result<(), JsError> {
    match self.init {
      LoadInit::Main(ref specifier) => {
        resolve_import(specifier, ".")?;
      }
      LoadInit::Side(ref specifier) => {
        resolve_import(specifier, ".")?;
      }
      LoadInit::DynamicImport(ref specifier, ref referrer) => {
        resolve_import(specifier, referrer)?;
      }
    };

    Ok(())
  }

  pub fn is_currently_loading_main_module(&self) -> bool {
    !self.is_dynamic_import() && matches!(self.init, LoadInit::Main(..))
  }

  pub fn register_and_recurse(
    &mut self,
    scope: &mut v8::HandleScope,
    module_source: &ModuleSource,
  ) -> Result<(), JsError> {
    // Register the module in the module map unless it's already there. If the
    // specified URL and the "true" URL are different, register the alias.
    if module_source.module_url_specified != module_source.module_url_found {
      self.module_map_rc.borrow_mut().alias(
        &module_source.module_url_specified,
        &module_source.module_url_found,
      );
    }
    let maybe_module_id = self
      .module_map_rc
      .borrow()
      .get_id(&module_source.module_url_found);
    let module_id = match maybe_module_id {
      Some(id) => {
        debug!(
          "Already-registered module fetched again: {}",
          module_source.module_url_found
        );
        id
      }
      None => self.module_map_rc.borrow_mut().new_module(
        scope,
        self.is_currently_loading_main_module(),
        &module_source.module_url_found,
        &module_source.code,
      )?,
    };

    // Recurse the module's imports. There are two cases for each import:
    // 1. If the module is not in the module map, start a new load for it in
    //    `self.pending`. The result of that load should eventually be passed to
    //    this function for recursion.
    // 2. If the module is already in the module map, queue it up to be
    //    recursed synchronously here.
    // This robustly ensures that the whole graph is in the module map before
    // `LoadState::Done` is set.
    let specifier = resolve_url(&module_source.module_url_found).unwrap();
    let mut already_registered = VecDeque::new();
    already_registered.push_back((module_id, specifier.clone()));
    self.visited.insert(specifier);
    while let Some((module_id, referrer)) = already_registered.pop_front() {
      let imports = self
        .module_map_rc
        .borrow()
        .get_children(module_id)
        .unwrap()
        .clone();
      for specifier in imports {
        if !self.visited.contains(&specifier) {
          if let Some(module_id) = self.module_map_rc.borrow().get_id(specifier.as_str()) {
            already_registered.push_back((module_id, specifier.clone()));
          } else {
            load(&specifier, Some(referrer.clone()), self.is_dynamic_import());
          }
          self.visited.insert(specifier);
        }
      }
    }

    Ok(())
  }

  pub fn module_source(&mut self) -> Result<ModuleSource, JsError> {
    // IMPORTANT: Do not borrow `inner.module_map_rc` here. It may not be
    // available.

    /*match self.state {
    LoadState::Init => {*/
    let module_specifier = match self.resolve_root() {
      Ok(url) => url,
      Err(error) => return Err(error),
    };

    if let Some(_module_id) = self.root_module_id {
      Ok(ModuleSource {
        module_url_specified: module_specifier.to_string(),
        module_url_found: module_specifier.to_string(),
        // The code will be discarded, since this module is already in the
        // module map.
        code: Default::default(),
      })
    } else {
      let maybe_referrer = match self.init {
        LoadInit::DynamicImport(_, ref referrer) => resolve_url(referrer).ok(),
        _ => None,
      };
      load(&module_specifier, maybe_referrer, self.is_dynamic_import())
    }
  }
}

pub struct ModuleInfo {
  pub id: ModuleId,
  // Used in "bindings.rs" for "import.meta.main" property value.
  pub main: bool,
  pub name: String,
  pub import_specifiers: Vec<ModuleSpecifier>,
}

/// A symbolic module entity.
enum SymbolicModule {
  /// This module is an alias to another module.
  /// This is useful such that multiple names could point to
  /// the same underlying module (particularly due to redirects).
  Alias(String),
  /// This module associates with a V8 module by id.
  Mod(ModuleId),
}

/// A collection of JS modules.
pub struct ModuleMap {
  // Handling of specifiers and v8 objects
  ids_by_handle: HashMap<v8::Global<v8::Module>, ModuleId>,
  handles_by_id: HashMap<ModuleId, v8::Global<v8::Module>>,
  info: HashMap<ModuleId, ModuleInfo>,
  by_name: HashMap<String, SymbolicModule>,
  next_module_id: ModuleId,

  // Handling of futures for loading module sources
  pub(crate) dynamic_import_map: HashMap<ModuleLoadId, v8::Global<v8::PromiseResolver>>,
}

impl ModuleMap {
  pub fn new() -> ModuleMap {
    Self {
      ids_by_handle: HashMap::new(),
      handles_by_id: HashMap::new(),
      info: HashMap::new(),
      by_name: HashMap::new(),
      next_module_id: 1,
      dynamic_import_map: HashMap::new(),
    }
  }

  /// Get module id, following all aliases in case of module specifier
  /// that had been redirected.
  pub fn get_id(&self, name: &str) -> Option<ModuleId> {
    let mut mod_name = name;
    loop {
      let symbolic_module = self.by_name.get(mod_name)?;
      match symbolic_module {
        SymbolicModule::Alias(target) => {
          mod_name = target;
        }
        SymbolicModule::Mod(mod_id) => return Some(*mod_id),
      }
    }
  }

  // Create and compile an ES module.
  pub fn new_module(
    &mut self,
    scope: &mut v8::HandleScope,
    main: bool,
    name: &str,
    source: &str,
  ) -> Result<ModuleId, JsError> {
    let name_str = v8::String::new(scope, name).unwrap();
    let source_str = v8::String::new(scope, source).unwrap();

    let origin = module_origin(scope, name_str);
    let source = v8::script_compiler::Source::new(source_str, Some(&origin));

    let tc_scope = &mut v8::TryCatch::new(scope);

    let maybe_module = v8::script_compiler::compile_module(tc_scope, source);

    if tc_scope.has_caught() {
      assert!(maybe_module.is_none());
      let e = tc_scope.exception().unwrap();
      return exception_to_err_result(tc_scope, e);
    }

    let module = maybe_module.unwrap();

    let mut import_specifiers: Vec<ModuleSpecifier> = vec![];
    let module_requests = module.get_module_requests();
    for i in 0..module_requests.length() {
      let module_request =
        v8::Local::<v8::ModuleRequest>::try_from(module_requests.get(tc_scope, i).unwrap())
          .unwrap();
      let import_specifier = module_request
        .get_specifier()
        .to_rust_string_lossy(tc_scope);
      let module_specifier = resolve_import(&import_specifier, name)?;
      import_specifiers.push(module_specifier);
    }

    if main {
      let maybe_main_module = self.info.values().find(|module| module.main);
      if let Some(_main_module) = maybe_main_module {
        return Err(JsError::MainModuleAlreadyExists {
          name: name.to_string(),
        });
      }
    }

    let handle = v8::Global::<v8::Module>::new(tc_scope, module);
    let id = self.next_module_id;
    self.next_module_id += 1;
    self
      .by_name
      .insert(name.to_string(), SymbolicModule::Mod(id));
    self.handles_by_id.insert(id, handle.clone());
    self.ids_by_handle.insert(handle, id);
    self.info.insert(
      id,
      ModuleInfo {
        id,
        main,
        name: name.to_string(),
        import_specifiers,
      },
    );

    Ok(id)
  }

  pub fn get_children(&self, id: ModuleId) -> Option<&Vec<ModuleSpecifier>> {
    self.info.get(&id).map(|i| &i.import_specifiers)
  }

  pub fn is_registered(&self, specifier: &ModuleSpecifier) -> bool {
    self.get_id(specifier.as_str()).is_some()
  }

  pub fn alias(&mut self, name: &str, target: &str) {
    self
      .by_name
      .insert(name.to_string(), SymbolicModule::Alias(target.to_string()));
  }

  pub fn get_handle(&self, id: ModuleId) -> Option<v8::Global<v8::Module>> {
    self.handles_by_id.get(&id).cloned()
  }

  pub fn get_info(&self, global: &v8::Global<v8::Module>) -> Option<&ModuleInfo> {
    if let Some(id) = self.ids_by_handle.get(global) {
      return self.info.get(id);
    }

    None
  }

  pub fn get_info_by_id(&self, id: &ModuleId) -> Option<&ModuleInfo> {
    self.info.get(id)
  }

  pub fn load_main(
    module_map_rc: Rc<RefCell<ModuleMap>>,
    specifier: &str,
  ) -> Result<RecursiveModuleLoad, JsError> {
    let load = RecursiveModuleLoad::main(specifier, module_map_rc.clone());
    load.prepare()?;
    Ok(load)
  }

  pub fn load_side(
    module_map_rc: Rc<RefCell<ModuleMap>>,
    specifier: &str,
  ) -> Result<RecursiveModuleLoad, JsError> {
    let load = RecursiveModuleLoad::side(specifier, module_map_rc.clone());
    load.prepare()?;
    Ok(load)
  }

  // Initiate loading of a module graph imported using `import()`.
  pub fn load_dynamic_import(
    module_map_rc: Rc<RefCell<ModuleMap>>,
    specifier: &str,
    referrer: &str,
    resolver_handle: v8::Global<v8::PromiseResolver>,
  ) -> Result<(), JsError> {
    let load = RecursiveModuleLoad::dynamic_import(specifier, referrer, module_map_rc.clone());
    module_map_rc
      .borrow_mut()
      .dynamic_import_map
      .insert(load.id, resolver_handle);
    let resolve_result = resolve_import(specifier, referrer);

    match resolve_result {
      Ok(module_specifier) => {
        if !module_map_rc.borrow().is_registered(&module_specifier) {
          load.prepare();
          Ok(())
        } else {
          Ok(())
        }
      }
      Err(error) => Err(error),
    }
  }

  /// Called by `module_resolve_callback` during module instantiation.
  pub fn resolve_callback<'s>(
    &self,
    scope: &mut v8::HandleScope<'s>,
    specifier: &str,
    referrer: &str,
  ) -> Option<v8::Local<'s, v8::Module>> {
    let resolved_specifier =
      resolve_import(specifier, referrer).expect("Module should have been already resolved");

    if let Some(id) = self.get_id(resolved_specifier.as_str()) {
      if let Some(handle) = self.get_handle(id) {
        return Some(v8::Local::new(scope, handle));
      }
    }

    None
  }
}

pub fn module_origin<'a>(
  s: &mut v8::HandleScope<'a>,
  resource_name: v8::Local<'a, v8::String>,
) -> v8::ScriptOrigin<'a> {
  let source_map_url = v8::String::new(s, "").unwrap();
  v8::ScriptOrigin::new(
    s,
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

  let module_map_rc = JsRuntime::module_map(scope);
  let module_map = module_map_rc.borrow();

  let referrer_global = v8::Global::new(scope, referrer);

  let referrer_info = module_map
    .get_info(&referrer_global)
    .expect("ModuleInfo not found");
  let referrer_name = referrer_info.name.to_string();

  let specifier_str = specifier.to_rust_string_lossy(scope);

  let maybe_module = module_map.resolve_callback(scope, &specifier_str, &referrer_name);
  if let Some(module) = maybe_module {
    return Some(module);
  }

  let msg = format!(
    r#"Cannot resolve module "{}" from "{}""#,
    specifier_str, referrer_name
  );

  throw_type_error(scope, msg);
  None
}

fn throw_type_error(scope: &mut v8::HandleScope, message: impl AsRef<str>) {
  let message = v8::String::new(scope, message.as_ref()).unwrap();
  let exception = v8::Exception::type_error(scope, message);
  scope.throw_exception(exception);
}

fn load(
  module_specifier: &ModuleSpecifier,
  _maybe_referrer: Option<ModuleSpecifier>,
  _is_dynamic: bool,
) -> Result<ModuleSource, JsError> {
  let module_specifier = module_specifier.clone();

  let path = module_specifier
    .to_file_path()
    .map_err(|_| JsError::ImportFormat(module_specifier.to_string()))?;

  let code = match std::fs::read_to_string(path) {
    Ok(code) => Ok(code),
    Err(_) => Err(JsError::FileNotFound(module_specifier.to_string())),
  }?;

  let module = ModuleSource {
    code,
    module_url_specified: module_specifier.to_string(),
    module_url_found: module_specifier.to_string(),
  };

  Ok(module)
}
