use crate::initialize;
use crate::platform::Initializer;
use crate::platform::PlatformCallback;
use crate::settings::Settings;
use crate::ResourceManager;
use std::fmt::Debug;
use std::sync::Arc;

/// The main container of the ECS
#[derive(Clone)]
pub struct World {
    /// The resource container
    pub resource_manager: Arc<ResourceManager>,
    platform: Option<PlatformCallback>,
}

impl Debug for World {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        self.resource_manager.fmt(formatter)
    }
}

impl<'s> World {
    /// Returns a World
    pub fn new() -> World {
        let resource_manager = Arc::new(ResourceManager::new());
        initialize(resource_manager.clone());

        World {
            resource_manager,
            platform: None,
        }
    }

    /// Run the world
    pub fn run(&self, initializer: Initializer, settings: &Settings) {
        if let Some(platform) = self.platform {
            platform(self.resource_manager.clone(), initializer, settings);
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
