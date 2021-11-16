use fruity_any::*;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_editor::dialog_service::DialogService;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use std::sync::Arc;
use tinyfiledialogs::open_file_dialog;
use tinyfiledialogs::save_file_dialog_with_filter;

#[derive(Debug, FruityAny)]
pub struct WgpuDialogService {}

impl WgpuDialogService {
    pub fn new(_resource_container: Arc<ResourceContainer>) -> WgpuDialogService {
        WgpuDialogService {}
    }
}

impl DialogService for WgpuDialogService {
    fn save(&self, file_types: &[&str]) -> Option<String> {
        save_file_dialog_with_filter("Save", "scene.scene", file_types, "Save your file")
    }

    fn open(&self, file_types: &[&str]) -> Option<String> {
        open_file_dialog("Save", ".", None)
    }
}

impl IntrospectObject for WgpuDialogService {
    fn get_class_name(&self) -> String {
        "DialogService".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for WgpuDialogService {}
