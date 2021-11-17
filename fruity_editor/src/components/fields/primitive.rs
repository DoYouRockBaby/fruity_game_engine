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
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::SetterCaller;
use fruity_core::serialize::serialized::Serialized;
use fruity_ecs::component::component_rwlock::ComponentRwLock;
use std::convert::TryFrom;
use std::sync::Arc;

macro_rules! impl_int_for_editable_component {
    ( $fn_name:ident, $type:ident ) => {
        pub fn $fn_name(component: ComponentRwLock, field_info: &FieldInfo) -> UIElement {
            let reader = component.read();
            let value = (field_info.getter)(reader.as_any_ref());
            let value = if let Ok(value) = $type::try_from(value) {
                value
            } else {
                $type::default()
            };

            let field_info = field_info.clone();
            let component = component.clone();
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
                                let mut writer = component.write();
                                match &field_info.setter {
                                    SetterCaller::Const(setter) => setter(
                                        writer.as_any_ref(),
                                        Serialized::try_from(value as $type).unwrap(),
                                    ),
                                    SetterCaller::Mut(setter) => setter(
                                        writer.as_any_mut(),
                                        Serialized::try_from(value as $type).unwrap(),
                                    ),
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
        pub fn $fn_name(component: ComponentRwLock, field_info: &FieldInfo) -> UIElement {
            let reader = component.read();
            let value = (field_info.getter)(reader.as_any_ref());
            let value = if let Ok(value) = $type::try_from(value) {
                value
            } else {
                $type::default()
            };

            let field_info = field_info.clone();
            let component = component.clone();
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
                                let component = component.clone();
                                let mut writer = component.write();
                                match &field_info.setter {
                                    SetterCaller::Const(setter) => setter(
                                        writer.as_any_ref(),
                                        Serialized::try_from(value as $type).unwrap(),
                                    ),
                                    SetterCaller::Mut(setter) => setter(
                                        writer.as_any_mut(),
                                        Serialized::try_from(value as $type).unwrap(),
                                    ),
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

pub fn draw_editor_bool(component: ComponentRwLock, field_info: &FieldInfo) -> UIElement {
    let reader = component.read();
    let value = (field_info.getter)(reader.as_any_ref());
    let value = if let Ok(value) = bool::try_from(value) {
        value
    } else {
        bool::default()
    };

    let field_info = field_info.clone();
    let component = component.clone();
    Checkbox {
        label: field_info.name.to_string(),
        value: value,
        on_change: Arc::new(move |value| {
            let mut writer = component.write();

            match &field_info.setter {
                SetterCaller::Const(setter) => {
                    setter(writer.as_any_ref(), Serialized::try_from(value).unwrap())
                }
                SetterCaller::Mut(setter) => {
                    setter(writer.as_any_mut(), Serialized::try_from(value).unwrap())
                }
                SetterCaller::None => (),
            };
        }),
    }
    .elem()
}

pub fn draw_editor_string(component: ComponentRwLock, field_info: &FieldInfo) -> UIElement {
    let reader = component.read();
    let value = (field_info.getter)(reader.as_any_ref());
    let value = if let Ok(value) = String::try_from(value) {
        value
    } else {
        String::default()
    };

    let field_info = field_info.clone();
    let component = component.clone();
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
                        let mut writer = component.write();

                        match &field_info.setter {
                            SetterCaller::Const(setter) => setter(
                                writer.as_any_ref(),
                                Serialized::try_from(value.to_string()).unwrap(),
                            ),
                            SetterCaller::Mut(setter) => setter(
                                writer.as_any_mut(),
                                Serialized::try_from(value.to_string()).unwrap(),
                            ),
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
