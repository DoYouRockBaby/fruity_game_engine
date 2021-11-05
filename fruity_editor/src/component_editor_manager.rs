use crate::components::component::ComponentFieldEditor;
use crate::ui_element::UIElement;
use fruity_any::*;
use fruity_core::component::component_rwlock::ComponentRwLock;
use fruity_core::service::service::Service;
use fruity_core::service::service_manager::ServiceManager;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(FruityAny)]
pub struct ComponentEditorManager {
    component_field_editors: HashMap<TypeId, fn(ComponentRwLock, &FieldInfo) -> UIElement>,
}

impl ComponentEditorManager {
    pub fn new(_service_manager: &Arc<RwLock<ServiceManager>>) -> Self {
        ComponentEditorManager {
            component_field_editors: HashMap::new(),
        }
    }

    pub fn register_component_field_editor<T>(&mut self)
    where
        T: ComponentFieldEditor + 'static,
    {
        self.component_field_editors
            .insert(TypeId::of::<T>(), T::draw_editor);
    }

    pub fn get_component_field_editor(
        &self,
        type_id: TypeId,
    ) -> Option<fn(ComponentRwLock, &FieldInfo) -> UIElement> {
        self.component_field_editors.get(&type_id).map(|draw| *draw)
    }
}

impl Debug for ComponentEditorManager {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl IntrospectObject for ComponentEditorManager {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Service for ComponentEditorManager {}
