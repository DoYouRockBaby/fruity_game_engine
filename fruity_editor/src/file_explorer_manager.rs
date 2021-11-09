use fruity_any::*;
use fruity_core::service::service::Service;
use fruity_core::service::service_manager::ServiceManager;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

pub type FileTypeAction = dyn Fn(&str) + Send + Sync;

#[derive(FruityAny)]
pub struct FileExplorerManager {
    file_type_actions: HashMap<String, Arc<FileTypeAction>>,
}

impl FileExplorerManager {
    pub fn new(_service_manager: &Arc<RwLock<ServiceManager>>) -> Self {
        FileExplorerManager {
            file_type_actions: HashMap::new(),
        }
    }

    pub fn register_file_type_action<F>(&mut self, file_type: &str, action: F)
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        self.file_type_actions
            .insert(file_type.to_string(), Arc::new(action));
    }

    pub fn get_file_type_action(&self, file_type: &str) -> Option<Arc<FileTypeAction>> {
        self.file_type_actions
            .get(file_type)
            .map(|draw| draw.clone())
    }
}

impl Debug for FileExplorerManager {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl IntrospectObject for FileExplorerManager {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Service for FileExplorerManager {}
