use crate::service::service::Service;
use crate::service::single_thread_service::SingleThreadService;
use crate::World;
use fruity_any::*;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use hot_reload_lib::load_symbol;
use hot_reload_lib::HotReloadLib;
use std::collections::HashMap;
use std::fmt::Debug;

/// An error that can occure during modules loading
#[derive(Debug, Clone)]
pub enum LoadModuleError {
    /// A file couln't not be read
    FileReadFailed(String),
    /// A module has an incorrect format
    IncorrectModule(String),
}

struct InnerModuleManager {
    libs: HashMap<String, HotReloadLib>,
    world: World,
}

impl Debug for InnerModuleManager {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

/// A structure to manage module loading, supports hot reload
#[derive(Debug, FruityAny)]
pub struct ModuleManager {
    single_thread_service: SingleThreadService<InnerModuleManager>,
}

impl ModuleManager {
    /// Returns a ModuleManager
    pub fn new(world: &World) -> ModuleManager {
        let world = world.clone();
        let single_thread_service =
            SingleThreadService::<InnerModuleManager>::start(move || InnerModuleManager {
                libs: HashMap::new(),
                world: world.clone(),
            });

        ModuleManager {
            single_thread_service,
        }
    }

    /// Load dynamic modules contained in a folder
    ///
    /// # Arguments
    /// * `world` - The world instance
    /// * `folder` - The folder where the lib is stored
    /// * `lib` - The lib name
    ///
    pub fn load_module(&self, folder: &str, lib_name: &str) {
        let folder = folder.to_string();
        let lib_name = lib_name.to_string();

        self.single_thread_service.call(move |module_manager| {
            let world = module_manager.world.clone();

            let lib = HotReloadLib::new(&folder, &lib_name, move |lib| {
                let world = world.clone();
                load_symbol::<fn(&World)>(&lib, "initialize")(&world);
            });
            log::debug!("Loaded {}", lib_name);

            module_manager.libs.insert(lib_name.to_string(), lib);
        });
    }

    /// Hot reload all loaded modules if needed
    pub fn update_modules(&self) {
        self.single_thread_service.call(move |module_manager| {
            module_manager
                .libs
                .iter_mut()
                .for_each(|(_, module)| module.update());
        });
    }
}

impl Drop for ModuleManager {
    fn drop(&mut self) {}
}

impl IntrospectObject for ModuleManager {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Service for ModuleManager {}
