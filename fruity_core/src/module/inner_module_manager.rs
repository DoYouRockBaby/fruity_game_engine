use crate::module::module_manager::LoadModuleError;
use crate::World;
use hot_reload_lib::load_symbol;
use hot_reload_lib::HotReloadLib;
use std::collections::HashMap;

/// A structure to manage module loading, supports hot reload
pub struct InnerModuleManager {
    libs: HashMap<String, HotReloadLib>,
    world: World,
}

impl InnerModuleManager {
    /// Returns a ModuleManager
    pub fn new(world: World) -> InnerModuleManager {
        InnerModuleManager {
            libs: HashMap::new(),
            world: world,
        }
    }

    /// Load dynamic modules contained in a folder
    ///
    /// # Arguments
    /// * `world` - The world instance
    /// * `folder` - The folder where the lib is stored
    /// * `lib` - The lib name
    ///
    pub fn load_module(&mut self, folder: &str, lib_name: &str) -> Result<(), LoadModuleError> {
        let world = self.world.clone();

        let lib = HotReloadLib::new(folder, lib_name, move |lib| {
            let world = world.clone();
            load_symbol::<fn(&World)>(&lib, "initialize")(&world);
        });
        log::debug!("Loaded {}", lib_name);

        self.libs.insert(lib_name.to_string(), lib);

        Ok(())
    }

    /// Hot reload all loaded modules if needed
    pub fn update_modules(&mut self) -> Result<(), LoadModuleError> {
        self.libs.iter_mut().for_each(|(_, module)| module.update());

        Ok(())
    }
}
