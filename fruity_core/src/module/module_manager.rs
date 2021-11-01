use crate::RunCallback;
use crate::ServiceManager;
use hot_reload_lib::load_symbol;
use hot_reload_lib::HotReloadLib;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

/// An error that can occure during modules loading
#[derive(Debug, Clone)]
pub enum LoadModuleError {
    /// A file couln't not be read
    FileReadFailed(String),
    /// A module has an incorrect format
    IncorrectModule(String),
}

/// A structure to manage module loading, supports hot reload
pub struct ModuleManager {
    libs: HashMap<String, HotReloadLib>,
    service_manager: Arc<RwLock<ServiceManager>>,
}

impl ModuleManager {
    /// Returns a ModuleManager
    pub fn new(service_manager: &Arc<RwLock<ServiceManager>>) -> ModuleManager {
        ModuleManager {
            libs: HashMap::new(),
            service_manager: service_manager.clone(),
        }
    }

    /// Load dynamic modules contained in a folder
    ///
    /// # Arguments
    /// * `world` - The world instance
    /// * `folder` - The folder where the lib is stored
    /// * `lib` - The lib name
    ///
    pub fn load_module(&mut self, folder: &str, lib_name: &str) -> Option<RunCallback> {
        let service_manager = self.service_manager.clone();

        let lib = HotReloadLib::new(&folder, &lib_name, move |lib| {
            let service_manager = service_manager.clone();
            load_symbol::<fn(&Arc<RwLock<ServiceManager>>) -> Option<RunCallback>>(
                &lib,
                "initialize",
            )(&service_manager);
        });
        log::debug!("Loaded {}", lib_name);

        let initialize_result = lib
            .load_symbol::<fn(&Arc<RwLock<ServiceManager>>) -> Option<RunCallback>>("initialize")(
            &self.service_manager,
        );

        self.libs.insert(lib_name.to_string(), lib);

        initialize_result
    }

    /// Hot reload all loaded modules if needed
    pub fn update_modules(&mut self) {
        self.libs.iter_mut().for_each(|(_, module)| module.update());
    }
}
