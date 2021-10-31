use crate::components::component::EditableComponent;
use crate::ui_element::input::Checkbox;
use crate::ui_element::input::FloatInput;
use crate::ui_element::input::Input;
use crate::ui_element::input::IntegerInput;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use fruity_core::component::component_rwlock::ComponentRwLock;
use fruity_introspect::serialized::Serialized;
use fruity_introspect::FieldInfo;
use fruity_introspect::SetterCaller;
use std::any::TypeId;
use std::convert::TryFrom;
use std::sync::Arc;

macro_rules! impl_int_for_editable_component {
    ( $type:ident ) => {
        impl EditableComponent for $type {
            fn type_id() -> TypeId {
                TypeId::of::<$type>()
            }

            fn render_edit(component: ComponentRwLock, field_info: &FieldInfo) -> UIElement {
                let reader = component.read();
                let value = (field_info.getter)(reader.as_any_ref());
                let value = if let Ok(value) = $type::try_from(value) {
                    value
                } else {
                    $type::default()
                };

                let field_info = field_info.clone();
                let component = component.clone();
                IntegerInput {
                    label: field_info.name.to_string(),
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
                .elem()
            }
        }
    };
}

impl_int_for_editable_component!(u8);
impl_int_for_editable_component!(u16);
impl_int_for_editable_component!(u32);
impl_int_for_editable_component!(u64);
impl_int_for_editable_component!(usize);
impl_int_for_editable_component!(i8);
impl_int_for_editable_component!(i16);
impl_int_for_editable_component!(i32);
impl_int_for_editable_component!(i64);
impl_int_for_editable_component!(isize);

macro_rules! impl_float_for_editable_component {
    ( $type:ident ) => {
        impl EditableComponent for $type {
            fn type_id() -> TypeId {
                TypeId::of::<$type>()
            }
            fn render_edit(component: ComponentRwLock, field_info: &FieldInfo) -> UIElement {
                let reader = component.read();
                let value = (field_info.getter)(reader.as_any_ref());
                let value = if let Ok(value) = $type::try_from(value) {
                    value
                } else {
                    $type::default()
                };

                let field_info = field_info.clone();
                let component = component.clone();
                FloatInput {
                    label: field_info.name.to_string(),
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
                .elem()
            }
        }
    };
}

impl_float_for_editable_component!(f32);
impl_float_for_editable_component!(f64);

impl EditableComponent for bool {
    fn type_id() -> TypeId {
        TypeId::of::<bool>()
    }
    fn render_edit(component: ComponentRwLock, field_info: &FieldInfo) -> UIElement {
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
}

impl EditableComponent for String {
    fn type_id() -> TypeId {
        TypeId::of::<String>()
    }
    fn render_edit(component: ComponentRwLock, field_info: &FieldInfo) -> UIElement {
        let reader = component.read();
        let value = (field_info.getter)(reader.as_any_ref());
        let value = if let Ok(value) = String::try_from(value) {
            value
        } else {
            String::default()
        };

        let field_info = field_info.clone();
        let component = component.clone();
        Input {
            label: field_info.name.to_string(),
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
        .elem()
    }
}
