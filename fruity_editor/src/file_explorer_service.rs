use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::utils::string::get_file_type_from_path;
use fruity_graphic::resources::texture_resource::TextureResource;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

struct FileTypeEntry {
    get_thumbnail:
        Arc<dyn Fn(&str) -> Option<ResourceReference<dyn TextureResource>> + Send + Sync>,
    on_selected: Arc<dyn Fn(&str) + Send + Sync>,
}

#[derive(FruityAny)]
pub struct FileExplorerService {
    resource_container: Arc<ResourceContainer>,
    file_types: HashMap<String, FileTypeEntry>,
}

impl FileExplorerService {
    pub fn new(resource_container: Arc<ResourceContainer>) -> Self {
        FileExplorerService {
            resource_container,
            file_types: HashMap::new(),
        }
    }

    pub fn register_file_type(
        &mut self,
        file_type: &str,
        get_thumbnail: impl Fn(&str) -> Option<ResourceReference<dyn TextureResource>>
            + Send
            + Sync
            + 'static,
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

    pub fn get_thumbnail(&self, file_path: &str) -> ResourceReference<dyn TextureResource> {
        match self.inner_get_thumbnail(file_path) {
            Some(thumbnail) => thumbnail,
            None => self
                .resource_container
                .get::<dyn TextureResource>("Editor/Icons/unknown")
                .unwrap(),
        }
    }

    pub fn notify_selected(&self, file_path: &str) {
        self.inner_notify_selected(file_path);
    }

    // TODO: There should be a way to use the ? without having to do that
    fn inner_notify_selected(&self, file_path: &str) -> Option<()> {
        let file_type = get_file_type_from_path(file_path)?;
        let file_type = self.file_types.get(&file_type)?;
        (file_type.on_selected)(file_path);

        Some(())
    }

    fn inner_get_thumbnail(
        &self,
        file_path: &str,
    ) -> Option<ResourceReference<dyn TextureResource>> {
        let file_type = get_file_type_from_path(file_path)?;
        let file_type = self.file_types.get(&file_type)?;
        (file_type.get_thumbnail)(file_path)
    }
}

impl Debug for FileExplorerService {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl IntrospectObject for FileExplorerService {
    fn get_class_name(&self) -> String {
        "FileExplorerService".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for FileExplorerService {}
