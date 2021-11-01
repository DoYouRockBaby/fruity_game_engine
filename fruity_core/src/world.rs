use crate::initialize;
use crate::platform::PlatformCallback;
use crate::service::service_manager::ServiceManager;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

/// The main container of the ECS
#[derive(Clone)]
pub struct World {
    /// The services container
    pub service_manager: Arc<RwLock<ServiceManager>>,
    platform: Option<PlatformCallback>,
}

impl Debug for World {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        self.service_manager.fmt(formatter)
    }
}

impl<'s> World {
    /// Returns a World
    pub fn new() -> World {
        let service_manager = Arc::new(RwLock::new(ServiceManager::new()));
        initialize(&service_manager);

        World {
            service_manager,
            platform: None,
        }
    }

    /// Run the world
    pub fn run(&self, initialize_callback: fn(&Arc<RwLock<ServiceManager>>)) {
        if let Some(platform) = self.platform {
            platform(&self.service_manager, initialize_callback);
        }
    }

    /// Set the callback that is called when running the world
    ///
    /// # Arguments
    /// * `callback` - The callback that will be called when world is run
    ///
    /// # Arguments
    /// * `F` - The callback closure type
    ///
    pub fn set_platform(&mut self, platform: PlatformCallback) {
        self.platform = Some(platform);
    }
}
