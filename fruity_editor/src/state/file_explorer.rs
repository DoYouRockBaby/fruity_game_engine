use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, FruityAny)]
pub struct FileExplorerState {
    current_dir: String,
}

impl Default for FileExplorerState {
    fn default() -> Self {
        FileExplorerState {
            current_dir: "./assets".to_string(),
        }
    }
}

impl FileExplorerState {
    pub fn get_current_dir(&self) -> String {
        self.current_dir.clone()
    }

    pub fn open_dir(&mut self, path: &str) {
        self.current_dir = path.to_string();
    }

    pub fn get_files(&self) -> Vec<PathBuf> {
        match fs::read_dir(&self.current_dir) {
            Ok(dir) => dir
                .filter_map(|file| file.ok())
                .map(|file| file.path())
                .collect::<Vec<_>>(),
            Err(_) => Vec::new(),
        }
    }
}

// TODO
impl IntrospectObject for FileExplorerState {
    fn get_class_name(&self) -> String {
        "FileExplorerState".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for FileExplorerState {}
