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
use crate::mutations::set_field_mutation::SetFieldMutation;
use crate::ui::context::UIContext;
use crate::ui::elements::display::Text;
use crate::ui::elements::input::Button;
use crate::ui::elements::layout::Collapsible;
use crate::ui::elements::layout::Column;
use crate::ui::elements::UIAlign;
use crate::ui::elements::UIElement;
use crate::ui::elements::UIWidget;
use crate::ui::hooks::use_read_service;
use crate::ui::hooks::use_write_service;
use crate::IntrospectEditorService;
use crate::MutationService;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_core::serialize::serialized::Serialized;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

pub mod primitive;

pub fn edit_introspect_fields(
    ctx: &mut UIContext,
    introspect_object: Box<dyn SerializableObject>,
) -> UIElement {
    let fields_edit = introspect_object
        .deref()
        .get_field_infos()
        .into_iter()
        .map(move |field_info| {
            let field_value = (field_info.getter)(introspect_object.deref().as_any_ref());
            let introspect_object = introspect_object.duplicate();

            let name = field_info.name.clone();
            field_editor(
                ctx,
                &name.clone(),
                field_value.clone(),
                Box::new(move |ctx, new_value| {
                    let mut mutation_service = use_write_service::<MutationService>(ctx);

                    mutation_service.push_action(SetFieldMutation {
                        target: introspect_object.clone(),
                        field: name.clone(),
                        previous_value: field_value.clone(),
                        new_value,
                    });
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
    ctx: &mut UIContext,
    name: &str,
    value: Serialized,
    on_update: Box<dyn Fn(&UIContext, Serialized) + Send + Sync>,
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
            let introspect_editor_service = use_read_service::<IntrospectEditorService>(ctx);

            let type_id = value.deref().type_id();
            if let Some(field_editor) = introspect_editor_service.get_field_editor(type_id) {
                field_editor(
                    ctx,
                    name,
                    value,
                    Box::new(move |ctx, value| on_update(ctx, Serialized::NativeObject(value))),
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
                    ctx,
                    &field_name,
                    fields.get(&field_name).unwrap().clone(),
                    Box::new(move |ctx, value| {
                        let mut fields = fields_2.clone();
                        fields.insert(field_name_2.clone(), value);
                        on_update(
                            ctx,
                            Serialized::SerializedObject {
                                fields,
                                class_name: class_name.clone(),
                            },
                        );
                    }),
                ))
            }

            Collapsible {
                key: name.to_string(),
                title: name.to_string(),
                child: Column {
                    children,
                    align: UIAlign::Start,
                }
                .elem(),
                ..Default::default()
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
                    ctx,
                    &index.to_string(),
                    value.clone(),
                    Box::new(move |ctx, value| {
                        let mut elems = elems.clone();
                        let _ = std::mem::replace(&mut elems[index], value);
                        on_update(ctx, Serialized::Array(elems));
                    }),
                ))
            }

            let on_update = Arc::new(on_update);
            children.push(
                Button {
                    label: "+".to_string(),
                    on_click: Arc::new(move |ctx| {
                        let mut current_value = elems_2.clone();
                        current_value.push(Serialized::SerializedObject {
                            class_name: "unknown".to_string(),
                            fields: HashMap::new(),
                        });

                        on_update(ctx, Serialized::Array(current_value));
                    }),
                    ..Default::default()
                }
                .elem(),
            );

            Collapsible {
                key: name.to_string(),
                title: name.to_string(),
                child: Column {
                    children,
                    align: UIAlign::Start,
                }
                .elem(),
                ..Default::default()
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
