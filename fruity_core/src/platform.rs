use crate::ServiceManager;
use std::sync::Arc;
use std::sync::RwLock;

/// A platform implementation, is supposed to basicaly instantiate a Window
/// and to run the engine
///
/// Take as parameter a function that will be called to initialize the engine
///
pub type PlatformCallback =
    fn(service_manager: &Arc<RwLock<ServiceManager>>, initialize_engine: Initializer);

/// A platform implementation, is supposed to basicaly instantiate a Window
/// and to run the engine
///
/// Take as parameter a function that will be called to initialize the engine
///
pub type Initializer = fn(service_manager: &Arc<RwLock<ServiceManager>>);
