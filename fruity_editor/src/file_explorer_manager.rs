use fruity_any::*;
use fruity_core::resource::resources_manager::ResourceIdentifier;
use fruity_core::resource::resources_manager::ResourcesManager;
use fruity_core::service::service::Service;
use fruity_core::service::service_manager::ServiceManager;
use fruity_core::service::service_rwlock::ServiceRwLock;
use fruity_graphic::resources::texture_resource::TextureResource;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;

struct FileTypeEntry {
    get_thumbnail: Arc<dyn Fn(&str) -> Option<Arc<TextureResource>> + Send + Sync>,
    on_selected: Arc<dyn Fn(&str) + Send + Sync>,
}

#[derive(FruityAny)]
pub struct FileExplorerManager {
    resource_manager: ServiceRwLock<ResourcesManager>,
    file_types: HashMap<String, FileTypeEntry>,
}

impl FileExplorerManager {
    pub fn new(service_manager: &Arc<RwLock<ServiceManager>>) -> Self {
        let service_manager = service_manager.read().unwrap();

        FileExplorerManager {
            resource_manager: service_manager.get::<ResourcesManager>().unwrap(),
            file_types: HashMap::new(),
        }
    }

    pub fn register_file_type(
        &mut self,
        file_type: &str,
        get_thumbnail: impl Fn(&str) -> Option<Arc<TextureResource>> + Send + Sync + 'static,
        on_selected: impl Fn(&str) + Send + Sync + 'static,
    ) {
        self.file_types.insert(
            file_type.to_string(),
            FileTypeEntry {
                get_thumbnail: Arc::new(get_thumbnail),
                on_selected: Arc::new(on_selected),
            },
        );
    }

    pub fn get_thumbnail(&self, file_path: &str) -> Arc<TextureResource> {
        match self.inner_get_thumbnail(file_path) {
            Some(thumbnail) => thumbnail,
            None => {
                let resource_manager = self.resource_manager.read().unwrap();
                resource_manager
                    .get_resource::<TextureResource>(ResourceIdentifier(
                        "Editor/Icons/unknown".to_string(),
                    ))
                    .unwrap()
            }
        }
    }

    pub fn notify_selected(&self, file_path: &str) {
        self.inner_notify_selected(file_path);
    }

    // TODO: There should be a way to use the ? without having to do that
    fn inner_notify_selected(&self, file_path: &str) -> Option<()> {
        let file_type = Self::get_file_type_from_path(file_path)?;
        let file_type = self.file_types.get(&file_type)?;
        (file_type.on_selected)(file_path);

        Some(())
    }

    fn inner_get_thumbnail(&self, file_path: &str) -> Option<Arc<TextureResource>> {
        let file_type = Self::get_file_type_from_path(file_path)?;
        let file_type = self.file_types.get(&file_type)?;
        (file_type.get_thumbnail)(file_path)
    }

    fn get_file_type_from_path(file_path: &str) -> Option<String> {
        let path = Path::new(file_path);
        Some(path.extension()?.to_str()?.to_string())
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
