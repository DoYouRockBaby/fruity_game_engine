use crate::initialize;
use crate::platform::Initializer;
use crate::platform::PlatformCallback;
use crate::settings::Settings;
use crate::ResourceContainer;
use std::fmt::Debug;

/// The main container of the ECS
#[derive(Clone)]
pub struct World {
    /// The resource container
    pub resource_container: ResourceContainer,
    platform: Option<PlatformCallback>,
}

impl Debug for World {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        self.resource_container.fmt(formatter)
    }
}

impl<'s> World {
    /// Returns a World
    pub fn new() -> World {
        let resource_container = ResourceContainer::new();
        initialize(resource_container.clone());

        World {
            resource_container,
            platform: None,
        }
    }

    /// Run the world
    pub fn run(
        &self,
        ext_initializer: Initializer,
        world_initializer: Initializer,
        settings: &Settings,
    ) {
        puffin::profile_function!();

        if let Some(platform) = self.platform {
            platform(
                self.resource_container.clone(),
                ext_initializer,
                world_initializer,
                settings,
            );
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
