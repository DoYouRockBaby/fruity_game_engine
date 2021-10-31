use crate::module::inner_module_manager::InnerModuleManager;
use crate::service::service::Service;
use crate::World;
use fruity_any::*;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use std::fmt::Debug;
use std::sync::mpsc;
use std::thread;

/// An error that can occure during modules loading
#[derive(Debug, Clone)]
pub enum LoadModuleError {
    /// A file couln't not be read
    FileReadFailed(String),
    /// A module has an incorrect format
    IncorrectModule(String),
}

#[derive(Debug, Clone)]
enum ModuleEvent {
    LoadModule {
        folder: String,
        lib_name: String,
        notify_done_sender: mpsc::Sender<Result<(), LoadModuleError>>,
    },
    UpdateModules {
        notify_done_sender: mpsc::Sender<Result<(), LoadModuleError>>,
    },
}

/// A structure to manage module loading, supports hot reload
#[derive(Debug, FruityAny)]
pub struct ModuleManager {
    channel_sender: mpsc::SyncSender<ModuleEvent>,
}

impl ModuleManager {
    /// Returns a ModuleManager
    pub fn new(world: &World) -> ModuleManager {
        // TODO: think about a good number for sync channel
        let (sender, receiver) = mpsc::sync_channel::<ModuleEvent>(10);
        let (loading_sender, loading_receiver) = mpsc::channel::<()>();

        // Create a thread that will be dedicated to the module reload
        // An event channel will be used to send instruction to the module load thread
        let world = world.clone();
        thread::spawn(move || {
            let mut runtime = InnerModuleManager::new(world);
            loading_sender.send(()).unwrap();

            for received in receiver {
                match received {
                    ModuleEvent::LoadModule {
                        folder,
                        lib_name,
                        notify_done_sender,
                    } => {
                        let result = runtime.load_module(&folder, &lib_name);
                        notify_done_sender.send(result).unwrap();
                    }
                    ModuleEvent::UpdateModules { notify_done_sender } => {
                        let result = runtime.update_modules();
                        notify_done_sender.send(result).unwrap();
                    }
                };
            }
        });

        loading_receiver.recv().unwrap();

        ModuleManager {
            channel_sender: sender,
        }
    }

    /// Load dynamic modules contained in a folder
    ///
    /// # Arguments
    /// * `world` - The world instance
    /// * `folder` - The folder where the lib is stored
    /// * `lib` - The lib name
    ///
    pub fn load_module(&self, folder: &str, lib_name: &str) -> Result<(), LoadModuleError> {
        let (notify_done_sender, notify_done_receiver) =
            mpsc::channel::<Result<(), LoadModuleError>>();

        self.channel_sender
            .send(ModuleEvent::LoadModule {
                folder: folder.to_string(),
                lib_name: lib_name.to_string(),
                notify_done_sender,
            })
            .unwrap();

        notify_done_receiver.recv().unwrap()
    }

    /// Hot reload all loaded modules if needed
    pub fn update_modules(&self) -> Result<(), LoadModuleError> {
        let (notify_done_sender, notify_done_receiver) =
            mpsc::channel::<Result<(), LoadModuleError>>();

        self.channel_sender
            .send(ModuleEvent::UpdateModules { notify_done_sender })
            .unwrap();

        notify_done_receiver.recv().unwrap()
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
