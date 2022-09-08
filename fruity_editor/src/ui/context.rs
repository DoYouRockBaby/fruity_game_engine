use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::RwLock;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone)]
pub struct UIContext {
    pub(crate) shared: Arc<RwLock<SharedUIContext>>,
    pub(crate) local_index: Vec<usize>,
    pub(crate) last_new_child_index: usize,
    pub(crate) local_storage_current: usize,
}

impl UIContext {
    pub fn new(resource_container: ResourceContainer) -> Self {
        Self {
            shared: Arc::new(RwLock::new(SharedUIContext::new(resource_container))),
            local_index: Default::default(),
            last_new_child_index: 0,
            local_storage_current: 0,
        }
    }

    pub fn new_child(&mut self) -> Self {
        let mut local_index = self.local_index.clone();
        local_index.push(self.last_new_child_index);
        self.last_new_child_index += 1;

        Self {
            shared: self.shared.clone(),
            local_index,
            last_new_child_index: 0,
            local_storage_current: 0,
        }
    }

    pub fn resource_container(&self) -> ResourceContainer {
        let context_reader = self.shared.read();
        context_reader.resource_container.clone()
    }

    pub fn local_index(&self) -> Vec<usize> {
        self.local_index.clone()
    }
}

impl Deref for UIContext {
    type Target = Arc<RwLock<SharedUIContext>>;

    fn deref(&self) -> &Self::Target {
        &self.shared
    }
}

pub struct SharedUIContext {
    pub resource_container: ResourceContainer,
    pub global_storage: HashMap<String, Box<dyn Any + Send + Sync>>,
    pub local_storages: HashMap<Vec<usize>, Vec<Box<dyn Any + Send + Sync>>>,
}

impl SharedUIContext {
    pub fn new(resource_container: ResourceContainer) -> Self {
        Self {
            resource_container: resource_container.clone(),
            global_storage: Default::default(),
            local_storages: Default::default(),
        }
    }
}

impl Debug for SharedUIContext {
    fn fmt(&self, _: &mut Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}
