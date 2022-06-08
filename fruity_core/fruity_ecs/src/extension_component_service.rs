use crate::component::component::AnyComponent;
use crate::component::component::Component;
use crate::component::component::StaticComponent;
use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::sync::Arc;

/// A service to store components extensions
/// When a component is created, if an extension is registered, an other component with a given
/// type is created, this can be use if ou want to extend already existing components with other
/// attributes. This is for example used into the physic engine implementations.
///
/// Warning: The same extension type cannot be shared across multiple based component types
#[derive(FruityAny)]
pub struct ExtensionComponentService {
    extension_constructors: HashMap<String, Vec<Box<dyn Fn() -> AnyComponent + Send + Sync>>>,
}

impl ExtensionComponentService {
    /// Returns an ExtensionComponentService
    pub fn new(_resource_container: Arc<ResourceContainer>) -> Self {
        Self {
            extension_constructors: HashMap::new(),
        }
    }

    /// Register a component extension
    pub fn register<T: StaticComponent, E: Component + Default>(&mut self) {
        let constructor = Box::new(|| AnyComponent::new(E::default()));
        match self.extension_constructors.get_mut(T::get_component_name()) {
            Some(constructors) => {
                constructors.push(constructor);
            }
            None => {
                self.extension_constructors
                    .insert(T::get_component_name().to_string(), vec![constructor]);
            }
        }
    }

    /// Create extensions from a component
    pub fn get_component_extension(&self, component: &dyn Component) -> Vec<AnyComponent> {
        match self.extension_constructors.get(&component.get_class_name()) {
            Some(constructors) => constructors
                .iter()
                .map(|constructor| constructor())
                .collect::<Vec<_>>(),
            None => {
                vec![]
            }
        }
    }
}

impl Debug for ExtensionComponentService {
    fn fmt(&self, _: &mut Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

// TODO: Complete that
impl IntrospectObject for ExtensionComponentService {
    fn get_class_name(&self) -> String {
        "ExtensionComponentService".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for ExtensionComponentService {}
