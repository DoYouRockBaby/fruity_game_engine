use crate::component::component::Component;
use crate::entity::archetype::component_array::ComponentArray;
use crate::entity::archetype::component_collection::ComponentCollection;
use fruity_any::FruityAny;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::introspect::SetterCaller;
use fruity_core::serialize::serialized::Serialized;
use std::collections::HashMap;
use std::sync::Arc;

/// A wrapper for components that come from scripting languages as serialized
#[derive(Debug, Clone, FruityAny)]
pub struct SerializedComponent {
    class_name: String,
    fields: HashMap<String, Serialized>,
}

impl SerializedComponent {
    /// Returns a SerializedComponent
    pub fn new(class_name: String, fields: HashMap<String, Serialized>) -> SerializedComponent {
        SerializedComponent { class_name, fields }
    }
}

impl Component for SerializedComponent {
    fn get_collection(&self, components_per_entity: usize) -> Box<dyn ComponentCollection> {
        Box::new(ComponentArray::<SerializedComponent>::new(
            components_per_entity,
        ))
    }

    fn duplicate(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl IntrospectObject for SerializedComponent {
    fn get_class_name(&self) -> String {
        self.class_name.clone()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        self.fields
            .iter()
            .map(|(key, _)| {
                let key1 = key.clone();
                let key2 = key.clone();

                FieldInfo {
                    name: key.clone(),
                    serializable: true,
                    getter: Arc::new(move |this| {
                        let this = this.downcast_ref::<SerializedComponent>().unwrap();
                        this.fields.get(&key1).unwrap().clone()
                    }),
                    setter: SetterCaller::Mut(Arc::new(move |this, value| {
                        let this = this.downcast_mut::<SerializedComponent>().unwrap();
                        this.fields.insert(key2.clone(), value);
                    })),
                }
            })
            .collect::<Vec<_>>()
    }
}
