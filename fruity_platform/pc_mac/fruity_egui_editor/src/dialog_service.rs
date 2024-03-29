use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_editor::dialog_service::DialogService;
use tinyfiledialogs::open_file_dialog;
use tinyfiledialogs::save_file_dialog_with_filter;

#[derive(Debug, FruityAny)]
pub struct WgpuDialogService {}

impl WgpuDialogService {
    pub fn new(_resource_container: ResourceContainer) -> WgpuDialogService {
        WgpuDialogService {}
    }
}

impl DialogService for WgpuDialogService {
    fn save(&self, default_filename: &str, file_types: &[&str]) -> Option<String> {
        save_file_dialog_with_filter("Save", default_filename, file_types, "Save your file")
    }

    fn open(&self, file_types: &[&str]) -> Option<String> {
        let _filters = file_types
            .get(0)
            .map(|first_filter| (file_types, *first_filter));

        // TODO: Fix filters
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
