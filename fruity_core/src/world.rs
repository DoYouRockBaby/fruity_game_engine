use crate::service::service_manager::ServiceManager;
use std::fmt::Debug;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

/// The main container of the ECS
#[derive(Clone)]
pub struct World {
    /// The services container
    pub service_manager: Arc<RwLock<ServiceManager>>,
    run_callback: Arc<Mutex<Option<Box<dyn Fn() + Sync + Send + 'static>>>>,
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
        World {
            service_manager: Arc::new(RwLock::new(ServiceManager::new())),
            run_callback: Arc::new(Mutex::new(None)),
        }
    }

    /// Run the world
    pub fn run(&self) {
        let run_callback = self.run_callback.lock().unwrap();
        if let Some(run_callback) = run_callback.as_ref() {
            run_callback();
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
    pub fn set_run_callback<F>(&self, callback: F)
    where
        F: Fn() + Sync + Send + 'static,
    {
        let mut run_callback = self.run_callback.lock().unwrap();
        let run_callback = run_callback.deref_mut();
        *run_callback = Some(Box::new(callback));
    }
}
