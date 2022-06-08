use crate::settings::Settings;
use crate::ResourceContainer;
use std::sync::Arc;

/// A platform implementation, is supposed to basicaly instantiate a Window
/// and to run the engine
///
/// Take as parameter a function that will be called to initialize the engine
///
pub type PlatformCallback = fn(
    resource_container: Arc<ResourceContainer>,
    ext_initializer: Initializer,
    world_initializer: Initializer,
    settings: &Settings,
);

/// A platform implementation, is supposed to basicaly instantiate a Window
/// and to run the engine
///
/// Take as parameter a function that will be called to initialize the engine
///
pub type Initializer = fn(resource_container: Arc<ResourceContainer>, settings: &Settings);
