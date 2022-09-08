use crate::components::fields::edit_introspect_fields;
use crate::ui::context::UIContext;
use crate::ui::elements::layout::Empty;
use crate::ui::elements::UIElement;
use crate::ui::elements::UIWidget;
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
use std::ops::DerefMut;
use std::sync::Arc;

#[derive(FruityAny)]
pub struct InspectorService {
    inspect_types: HashMap<
        TypeId,
        Arc<dyn Fn(&mut UIContext, Box<dyn SerializableObject>) -> UIElement + Send + Sync>,
    >,
}

impl InspectorService {
    pub fn new(_resource_container: ResourceContainer) -> Self {
        Self {
            inspect_types: HashMap::new(),
        }
    }

    pub fn register_inspect_type<T: SerializableObject>(
        &mut self,
        inspect: impl Fn(&mut UIContext, &mut T) -> UIElement + Send + Sync + 'static,
    ) {
        self.inspect_types.insert(
            TypeId::of::<T>(),
            Arc::new(
                move |ctx: &mut UIContext, obj: Box<dyn SerializableObject>| match obj
                    .as_any_box()
                    .downcast::<T>()
                {
                    Ok(mut obj) => inspect(ctx, obj.deref_mut()),
                    Err(_) => Empty {}.elem(),
                },
            ),
        );
    }

    pub fn inspect(&self, ctx: &mut UIContext, obj: Box<dyn SerializableObject>) -> UIElement {
        match self.inspect_types.get(&obj.type_id()) {
            Some(inspect) => inspect(ctx, obj),
            None => edit_introspect_fields(ctx, obj),
        }
    }
}

impl Debug for InspectorService {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl IntrospectObject for InspectorService {
    fn get_class_name(&self) -> String {
        "InspectorService".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for InspectorService {}
