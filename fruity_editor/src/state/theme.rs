use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;

#[derive(Debug, FruityAny, Default)]
pub struct ThemeState {}

// TODO
impl IntrospectObject for ThemeState {
    fn get_class_name(&self) -> String {
        "ThemeState".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for ThemeState {}
