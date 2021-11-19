use crate::ui_element::display::Text;
use crate::ui_element::input::Checkbox;
use crate::ui_element::input::FloatInput;
use crate::ui_element::input::Input;
use crate::ui_element::input::IntegerInput;
use crate::ui_element::layout::Row;
use crate::ui_element::layout::RowItem;
use crate::ui_element::UIElement;
use crate::ui_element::UISize;
use crate::ui_element::UIWidget;
use fruity_core::convert::FruityInto;
use fruity_core::convert::FruityTryFrom;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::SetterCaller;
use fruity_core::serialize::serialized::SerializableObject;
use std::sync::Arc;

macro_rules! impl_int_for_editable_component {
    ( $fn_name:ident, $type:ident ) => {
        pub fn $fn_name(
            introspect: Box<dyn SerializableObject>,
            field_info: &FieldInfo,
        ) -> UIElement {
            let value = (field_info.getter)(introspect.as_any_ref());
            let value = if let Ok(value) = $type::fruity_try_from(value) {
                value
            } else {
                $type::default()
            };

            let field_info = field_info.clone();
            Row {
                children: vec![
                    RowItem {
                        size: UISize::Units(40.0),
                        child: Text {
                            text: field_info.name.to_string(),
                            ..Default::default()
                        }
                        .elem(),
                    },
                    RowItem {
                        size: UISize::Fill,
                        child: IntegerInput {
                            value: value as i64,
                            on_change: Arc::new(move |value| {
                                match &field_info.setter {
                                    SetterCaller::Const(setter) => setter(
                                        introspect.as_any_ref(),
                                        (value as $type).fruity_into(),
                                    ),
                                    SetterCaller::Mut(_) => (),
                                    SetterCaller::None => (),
                                };
                            }),
                        }
                        .elem(),
                    },
                ],
                ..Default::default()
            }
            .elem()
        }
    };
}

impl_int_for_editable_component!(draw_editor_u8, u8);
impl_int_for_editable_component!(draw_editor_u16, u16);
impl_int_for_editable_component!(draw_editor_u32, u32);
impl_int_for_editable_component!(draw_editor_u64, u64);
impl_int_for_editable_component!(draw_editor_usize, usize);
impl_int_for_editable_component!(draw_editor_i8, i8);
impl_int_for_editable_component!(draw_editor_i16, i16);
impl_int_for_editable_component!(draw_editor_i32, i32);
impl_int_for_editable_component!(draw_editor_i64, i64);
impl_int_for_editable_component!(draw_editor_isize, isize);

macro_rules! impl_float_for_editable_component {
    ( $fn_name:ident, $type:ident ) => {
        pub fn $fn_name(
            introspect: Box<dyn SerializableObject>,
            field_info: &FieldInfo,
        ) -> UIElement {
            let value = (field_info.getter)(introspect.as_any_ref());
            let value = if let Ok(value) = $type::fruity_try_from(value) {
                value
            } else {
                $type::default()
            };

            let field_info = field_info.clone();
            Row {
                children: vec![
                    RowItem {
                        size: UISize::Units(40.0),
                        child: Text {
                            text: field_info.name.to_string(),
                            ..Default::default()
                        }
                        .elem(),
                    },
                    RowItem {
                        size: UISize::Fill,
                        child: FloatInput {
                            value: value as f64,
                            on_change: Arc::new(move |value| {
                                match &field_info.setter {
                                    SetterCaller::Const(setter) => setter(
                                        introspect.as_any_ref(),
                                        (value as $type).fruity_into(),
                                    ),
                                    SetterCaller::Mut(_) => (),
                                    SetterCaller::None => (),
                                };
                            }),
                        }
                        .elem(),
                    },
                ],
                ..Default::default()
            }
            .elem()
        }
    };
}

impl_float_for_editable_component!(draw_editor_f32, f32);
impl_float_for_editable_component!(draw_editor_f64, f64);

pub fn draw_editor_bool(
    introspect: Box<dyn SerializableObject>,
    field_info: &FieldInfo,
) -> UIElement {
    let value = (field_info.getter)(introspect.as_any_ref());
    let value = if let Ok(value) = bool::fruity_try_from(value) {
        value
    } else {
        bool::default()
    };

    let field_info = field_info.clone();
    Checkbox {
        label: field_info.name.to_string(),
        value: value,
        on_change: Arc::new(move |value| {
            match &field_info.setter {
                SetterCaller::Const(setter) => setter(introspect.as_any_ref(), value.fruity_into()),
                SetterCaller::Mut(_) => (),
                SetterCaller::None => (),
            };
        }),
    }
    .elem()
}

pub fn draw_editor_string(
    introspect: Box<dyn SerializableObject>,
    field_info: &FieldInfo,
) -> UIElement {
    let value = (field_info.getter)(introspect.as_any_ref());
    let value = if let Ok(value) = String::fruity_try_from(value) {
        value
    } else {
        String::default()
    };

    let field_info = field_info.clone();
    Row {
        children: vec![
            RowItem {
                size: UISize::Units(40.0),
                child: Text {
                    text: field_info.name.to_string(),
                    ..Default::default()
                }
                .elem(),
            },
            RowItem {
                size: UISize::Fill,
                child: Input {
                    placeholder: "".to_string(),
                    value: value,
                    on_change: Arc::new(move |value: &str| {
                        match &field_info.setter {
                            SetterCaller::Const(setter) => {
                                setter(introspect.as_any_ref(), value.to_string().fruity_into())
                            }
                            SetterCaller::Mut(_) => (),
                            SetterCaller::None => (),
                        };
                    }),
                }
                .elem(),
            },
        ],
        ..Default::default()
    }
    .elem()
}
