use crate::components::fields::edit_introspect_fields;
use crate::ui_element::UIElement;
use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_ecs::component::component_reference::ComponentReference;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(FruityAny)]
pub struct ComponentInspectorService {
    inspect_types: HashMap<String, Arc<dyn Fn(ComponentReference) -> UIElement + Send + Sync>>,
}

impl ComponentInspectorService {
    pub fn new(_resource_container: Arc<ResourceContainer>) -> Self {
        Self {
            inspect_types: HashMap::new(),
        }
    }

    pub fn register_inspect_component(
        &mut self,
        component_identifier: &str,
        inspect: impl Fn(ComponentReference) -> UIElement + Send + Sync + 'static,
    ) {
        self.inspect_types
            .insert(component_identifier.to_string(), Arc::new(inspect));
    }

    pub fn inspect(&self, component: ComponentReference) -> UIElement {
        let component_identifier = {
            let reader = component.read();
            reader.get_class_name()
        };

        match self.inspect_types.get(&component_identifier) {
            Some(inspect) => inspect(component),
            None => edit_introspect_fields(Box::new(component.clone())),
        }
    }
}

impl Debug for ComponentInspectorService {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl IntrospectObject for ComponentInspectorService {
    fn get_class_name(&self) -> String {
        "ComponentInspectorService".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for ComponentInspectorService {}
