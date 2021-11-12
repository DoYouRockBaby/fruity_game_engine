use crate::settings::Settings;
use crate::ResourceManager;
use std::sync::Arc;

/// A platform implementation, is supposed to basicaly instantiate a Window
/// and to run the engine
///
/// Take as parameter a function that will be called to initialize the engine
///
pub type PlatformCallback =
    fn(service_manager: Arc<ResourceManager>, initialize_engine: Initializer, settings: &Settings);

/// A platform implementation, is supposed to basicaly instantiate a Window
/// and to run the engine
///
/// Take as parameter a function that will be called to initialize the engine
///
pub type Initializer = fn(service_manager: Arc<ResourceManager>, settings: &Settings);
