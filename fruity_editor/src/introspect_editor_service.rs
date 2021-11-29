use crate::ui_element::UIElement;
use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::serialize::serialized::SerializableObject;
use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

pub type IntrospectFieldEditor = Arc<
    dyn Fn(
            &str,
            Box<dyn SerializableObject>,
            Box<dyn Fn(Box<dyn SerializableObject>) + Send + Sync + 'static>,
        ) -> UIElement
        + Send
        + Sync
        + 'static,
>;

#[derive(FruityAny)]
pub struct IntrospectEditorService {
    component_field_editors: HashMap<TypeId, IntrospectFieldEditor>,
}

impl IntrospectEditorService {
    pub fn new(_resource_container: Arc<ResourceContainer>) -> Self {
        IntrospectEditorService {
            component_field_editors: HashMap::new(),
        }
    }

    pub fn register_field_editor<T, F>(&mut self, editor: F)
    where
        T: 'static,
        F: Fn(
                &str,
                Box<dyn SerializableObject>,
                Box<dyn Fn(Box<dyn SerializableObject>) + Send + Sync + 'static>,
            ) -> UIElement
            + Send
            + Sync
            + 'static,
    {
        let editor = Arc::new(editor);
        self.component_field_editors
            .insert(TypeId::of::<T>(), editor.clone());
    }

    pub fn get_field_editor(&self, type_id: TypeId) -> Option<IntrospectFieldEditor> {
        self.component_field_editors
            .get(&type_id)
            .map(|draw| draw.clone())
    }
}

impl Debug for IntrospectEditorService {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl IntrospectObject for IntrospectEditorService {
    fn get_class_name(&self) -> String {
        "IntrospectEditorService".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for IntrospectEditorService {}
