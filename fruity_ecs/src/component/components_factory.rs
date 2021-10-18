use crate::component::component::Component;
use crate::serialize::serialized::Serialized;
use crate::service::service::Service;
use crate::service::utils::cast_service;
use crate::service::utils::ArgumentCaster;
use fruity_any_derive::*;
use fruity_introspect::IntrospectMethods;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use std::collections::HashMap;
use std::sync::Arc;

/// Provides a factory for the component types
/// This will be used by the scripting language to expose component creation
#[derive(Debug, FruityAny)]
pub struct ComponentsFactory {
    factories: HashMap<String, fn() -> Box<dyn Component>>,
}

impl ComponentsFactory {
    /// Returns a ComponentFactory
    pub fn new() -> ComponentsFactory {
        ComponentsFactory {
            factories: HashMap::new(),
        }
    }

    /// Add a new component factory
    ///
    /// # Arguments
    /// * `component_type` - The component type identifier
    /// * `entity` - The factory,  return a new default instance of the component
    ///
    pub fn add(&mut self, component_type: &str, factory: fn() -> Box<dyn Component>) {
        self.factories.insert(component_type.to_string(), factory);
    }

    /// Instantiate a component from it's factory
    ///
    /// # Arguments
    /// * `component_type` - The component type identifier
    /// * `serialized` - A serialized value that will populate the new component
    ///
    pub fn instantiate(
        &self,
        component_type: &str,
        serialized: Serialized,
    ) -> Option<Box<dyn Component>> {
        let factory = self.factories.get(component_type)?;
        let mut component = factory();
        let component_fields = component.get_field_infos();

        if let Serialized::Object { fields, .. } = serialized {
            fields.into_iter().for_each(|(key, value)| {
                let field_info = component_fields
                    .iter()
                    .find(|field_info| field_info.name == *key);

                if let Some(field_info) = field_info {
                    (field_info.setter)(component.as_any_mut(), value);
                }
            })
        };

        Some(component)
    }

    /// Iterate over all component factories
    pub fn iter(&self) -> impl Iterator<Item = (&String, &fn() -> Box<dyn Component>)> {
        let test = self.factories.iter();
        test
    }
}

impl IntrospectMethods<Serialized> for ComponentsFactory {
    fn get_method_infos(&self) -> Vec<MethodInfo<Serialized>> {
        vec![MethodInfo {
            name: "instantiate".to_string(),
            call: MethodCaller::Const(Arc::new(move |this, args| {
                let this = cast_service::<ComponentsFactory>(this);

                let mut caster = ArgumentCaster::new("instantiate", args);
                let arg1 = caster.cast_next(|arg| arg.as_string())?;
                let arg2 = caster.cast_next(|arg| Some(arg))?;

                let component = this.instantiate(&arg1, arg2);
                if let Some(component) = component {
                    Ok(Some(Serialized::Component(component)))
                } else {
                    Ok(None)
                }
            })),
        }]
    }
}

impl Service for ComponentsFactory {}
