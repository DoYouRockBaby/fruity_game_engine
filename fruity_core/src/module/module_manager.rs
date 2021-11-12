use crate::platform::Initializer;
use crate::settings::Settings;
use crate::ResourceManager;
use hot_reload_lib::load_symbol;
use hot_reload_lib::HotReloadLib;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

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
    resource_manager: Arc<ResourceManager>,
}

impl ModuleManager {
    /// Returns a ModuleManager
    pub fn new(resource_manager: Arc<ResourceManager>) -> ModuleManager {
        ModuleManager {
            libs: HashMap::new(),
            resource_manager: resource_manager.clone(),
        }
    }

    /// Load dynamic modules contained in a folder
    ///
    /// # Arguments
    /// * `world` - The world instance
    /// * `folder` - The folder where the lib is stored
    /// * `lib` - The lib name
    ///
    pub fn load_module(&mut self, folder: &str, lib_name: &str, settings: &Settings) {
        let resource_manager = self.resource_manager.clone();

        let moved_settings = settings.clone();
        let lib = HotReloadLib::new(&folder, &lib_name, move |lib| {
            let resource_manager = resource_manager.clone();
            load_symbol::<Initializer>(&lib, "initialize")(resource_manager, &moved_settings);
        });
        log::debug!("Loaded {}", lib_name);

        lib.load_symbol::<Initializer>("initialize")(self.resource_manager.clone(), settings);

        self.libs.insert(lib_name.to_string(), lib);
    }

    /// Hot reload all loaded modules if needed
    pub fn update_modules(&mut self) {
        self.libs.iter_mut().for_each(|(_, module)| module.update());
    }
}
