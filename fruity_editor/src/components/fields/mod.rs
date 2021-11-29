use crate::components::fields::primitive::draw_editor_bool;
use crate::components::fields::primitive::draw_editor_f32;
use crate::components::fields::primitive::draw_editor_f64;
use crate::components::fields::primitive::draw_editor_i16;
use crate::components::fields::primitive::draw_editor_i32;
use crate::components::fields::primitive::draw_editor_i64;
use crate::components::fields::primitive::draw_editor_i8;
use crate::components::fields::primitive::draw_editor_isize;
use crate::components::fields::primitive::draw_editor_string;
use crate::components::fields::primitive::draw_editor_u16;
use crate::components::fields::primitive::draw_editor_u32;
use crate::components::fields::primitive::draw_editor_u64;
use crate::components::fields::primitive::draw_editor_u8;
use crate::components::fields::primitive::draw_editor_usize;
use crate::hooks::use_global;
use crate::state::world::WorldState;
use crate::ui_element::display::Text;
use crate::ui_element::input::Button;
use crate::ui_element::layout::Collapsible;
use crate::ui_element::layout::Column;
use crate::ui_element::UIAlign;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use crate::IntrospectEditorService;
use fruity_core::introspect::SetterCaller;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_core::serialize::serialized::Serialized;
use std::collections::HashMap;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;

pub mod primitive;

pub fn edit_introspect_fields(introspect_object: Box<dyn SerializableObject>) -> UIElement {
    let fields_edit = introspect_object
        .deref()
        .get_field_infos()
        .into_iter()
        .map(|field_info| {
            let field_value = (field_info.getter)(introspect_object.deref().as_any_ref());
            let introspect_object = introspect_object.duplicate();

            let name = field_info.name.clone();
            field_editor(
                &name,
                field_value,
                Box::new(move |new_value| {
                    match &field_info.setter {
                        SetterCaller::Const(call) => {
                            call(introspect_object.deref().as_any_ref(), new_value)
                        }
                        SetterCaller::Mut(call) => {
                            let mut introspect_object = introspect_object.duplicate();
                            call(introspect_object.deref_mut().as_any_mut(), new_value)
                        }
                        SetterCaller::None => {}
                    };
                }),
            )
        })
        .collect::<Vec<_>>();

    Column {
        children: fields_edit,
        align: UIAlign::Start,
    }
    .elem()
}

pub fn field_editor(
    name: &str,
    value: Serialized,
    on_update: Box<dyn Fn(Serialized) + Send + Sync + 'static>,
) -> UIElement {
    match value {
        Serialized::U8(value) => draw_editor_u8(name, Serialized::U8(value), on_update),
        Serialized::U16(value) => draw_editor_u16(name, Serialized::U16(value), on_update),
        Serialized::U32(value) => draw_editor_u32(name, Serialized::U32(value), on_update),
        Serialized::U64(value) => draw_editor_u64(name, Serialized::U64(value), on_update),
        Serialized::USize(value) => draw_editor_usize(name, Serialized::USize(value), on_update),
        Serialized::I8(value) => draw_editor_i8(name, Serialized::I8(value), on_update),
        Serialized::I16(value) => draw_editor_i16(name, Serialized::I16(value), on_update),
        Serialized::I32(value) => draw_editor_i32(name, Serialized::I32(value), on_update),
        Serialized::I64(value) => draw_editor_i64(name, Serialized::I64(value), on_update),
        Serialized::ISize(value) => draw_editor_isize(name, Serialized::ISize(value), on_update),
        Serialized::F32(value) => draw_editor_f32(name, Serialized::F32(value), on_update),
        Serialized::F64(value) => draw_editor_f64(name, Serialized::F64(value), on_update),
        Serialized::Bool(value) => draw_editor_bool(name, Serialized::Bool(value), on_update),
        Serialized::String(value) => draw_editor_string(name, Serialized::String(value), on_update),
        Serialized::NativeObject(value) => {
            let world_state = use_global::<WorldState>();

            let resource_container = world_state.resource_container.clone();
            let introspect_editor_service = resource_container.require::<IntrospectEditorService>();
            let introspect_editor_service = introspect_editor_service.read();

            let type_id = value.deref().type_id();
            if let Some(field_editor) = introspect_editor_service.get_field_editor(type_id) {
                field_editor(
                    name,
                    value,
                    Box::new(move |value| on_update(Serialized::NativeObject(value))),
                )
            } else {
                Text {
                    text: name.to_string(),
                    ..Default::default()
                }
                .elem()
            }
        }
        Serialized::SerializedObject { fields, class_name } => {
            let mut children = Vec::new();
            let mut field_names = fields
                .iter()
                .map(|(key, _)| key.clone())
                .collect::<Vec<_>>();
            field_names.sort_by(|a, b| a.cmp(b));

            let on_update = Arc::new(on_update);
            for field_name in field_names {
                let class_name = class_name.clone();

                let fields_2 = fields.clone();
                let field_name_2 = field_name.clone();
                let on_update = on_update.clone();
                children.push(field_editor(
                    &field_name,
                    fields.get(&field_name).unwrap().clone(),
                    Box::new(move |value| {
                        let mut fields = fields_2.clone();
                        fields.insert(field_name_2.clone(), value);
                        on_update(Serialized::SerializedObject {
                            fields,
                            class_name: class_name.clone(),
                        });
                    }),
                ))
            }

            Collapsible {
                title: name.to_string(),
                on_click: None,
                child: Column {
                    children,
                    align: UIAlign::Start,
                }
                .elem(),
            }
        }
        .elem(),
        Serialized::Array(elems) => {
            let mut children = Vec::new();
            let elems_2 = elems.clone();

            let on_update = Arc::new(on_update);
            for (index, value) in elems.iter().enumerate() {
                let elems = elems.clone();
                let on_update = on_update.clone();
                children.push(field_editor(
                    &index.to_string(),
                    value.clone(),
                    Box::new(move |value| {
                        let mut elems = elems.clone();
                        let _ = std::mem::replace(&mut elems[index], value);
                        on_update(Serialized::Array(elems));
                    }),
                ))
            }

            let on_update = Arc::new(on_update);
            children.push(
                Button {
                    label: "+".to_string(),
                    on_click: Arc::new(move || {
                        let mut current_value = elems_2.clone();
                        current_value.push(Serialized::SerializedObject {
                            class_name: "unknown".to_string(),
                            fields: HashMap::new(),
                        });

                        on_update(Serialized::Array(current_value));
                    }),
                    ..Default::default()
                }
                .elem(),
            );

            Collapsible {
                title: name.to_string(),
                on_click: None,
                child: Column {
                    children,
                    align: UIAlign::Start,
                }
                .elem(),
            }
            .elem()
        }
        _ => Text {
            text: name.to_string(),
            ..Default::default()
        }
        .elem(),
    }
}
