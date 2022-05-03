use crate::components::fields::edit_introspect_fields;
use crate::ui_element::UIElement;
use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::object_factory_service::ObjectFactoryService;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::serialize::serialized::Serialized;
use fruity_ecs::component::component::AnyComponent;
use fruity_ecs::component::component_reference::ComponentReference;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

lazy_static! {
    pub static ref DEFAULT_INSPECTOR: Arc<dyn Fn(ComponentReference) -> UIElement + Send + Sync> =
        Arc::new(|component| edit_introspect_fields(Box::new(component)));
}

#[derive(FruityAny)]
pub struct RegisterComponentParams {
    pub inspector: Arc<dyn Fn(ComponentReference) -> UIElement + Send + Sync>,
    pub dependencies: Vec<String>,
}

impl Default for RegisterComponentParams {
    fn default() -> Self {
        Self {
            inspector: DEFAULT_INSPECTOR.clone(),
            dependencies: Vec::new(),
        }
    }
}

#[derive(FruityAny)]
pub struct EditorComponentService {
    components: HashMap<String, RegisterComponentParams>,
    object_factory_service: ResourceReference<ObjectFactoryService>,
}

impl EditorComponentService {
    pub fn new(resource_container: Arc<ResourceContainer>) -> Self {
        Self {
            components: HashMap::new(),
            object_factory_service: resource_container.require::<ObjectFactoryService>(),
        }
    }

    pub fn register_component(
        &mut self,
        component_identifier: &str,
        params: RegisterComponentParams,
    ) {
        self.components
            .insert(component_identifier.to_string(), params);
    }

    pub fn inspect(&self, component: ComponentReference) -> UIElement {
        let component_identifier = component.get_class_name();

        match self.components.get(&component_identifier) {
            Some(params) => (params.inspector)(component),
            None => edit_introspect_fields(Box::new(component)),
        }
    }

    pub fn instantiate(&self, component_identifier: &str) -> Option<Vec<AnyComponent>> {
        let object_factory_service = self.object_factory_service.read();
        let component_params = self.components.get(component_identifier)?;
        let instance = object_factory_service.instantiate(component_identifier, vec![])?;
        let instance = if let Serialized::NativeObject(instance) = instance {
            instance
        } else {
            return None;
        };
        let instance = instance.as_any_box().downcast::<AnyComponent>().ok()?;

        let mut result = vec![*instance];
        let mut dependencies = component_params
            .dependencies
            .iter()
            .filter_map(|dependency| self.instantiate(dependency))
            .flatten()
            .collect::<Vec<_>>();
        result.append(&mut dependencies);

        Some(result)
    }

    pub fn search(&self, search: &str) -> impl Iterator<Item = String> + '_ {
        let search = search.to_string();
        self.components
            .keys()
            .filter(move |key| key.to_lowercase().contains(&search.to_lowercase()))
            .map(|key| key.clone())
    }
}

impl Debug for EditorComponentService {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

// TODO: Complete that
impl IntrospectObject for EditorComponentService {
    fn get_class_name(&self) -> String {
        "EditorComponentService".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for EditorComponentService {}
