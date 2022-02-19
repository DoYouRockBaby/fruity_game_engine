use bytemuck::{Pod, Zeroable};
use css_color_parser::Color as CssColor;
use fruity_any::*;
use fruity_core::convert::FruityInto;
use fruity_core::convert::FruityTryFrom;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::introspect::SetterCaller;
use fruity_core::settings::Settings;
use fruity_ecs::*;
use std::str::FromStr;
use std::sync::Arc;

pub mod matrix3;
pub mod matrix4;
pub mod vector2d;
pub mod vector3d;

#[repr(C)]
#[derive(Debug, FruityAny, SerializableObject, InstantiableObject, Copy, Clone, Pod, Zeroable)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0, 1.0)
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }

    pub fn alpha() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn overlay() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.3)
    }

    pub fn red() -> Self {
        Self::new(1.0, 0.0, 0.0, 1.0)
    }

    pub fn green() -> Self {
        Self::new(0.0, 1.0, 0.0, 1.0)
    }

    pub fn blue() -> Self {
        Self::new(0.0, 0.0, 1.0, 1.0)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::red()
    }
}

impl FromStr for Color {
    type Err = String;

    fn from_str(string: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let css = string
            .parse::<CssColor>()
            .map_err(|_| "Parse color failed".to_string())?;

        Ok(Color::new(
            css.r as f32 / 255.0,
            css.g as f32 / 255.0,
            css.b as f32 / 255.0,
            css.a,
        ))
    }
}

impl FruityTryFrom<Settings> for Color {
    type Error = String;

    fn fruity_try_from(value: Settings) -> Result<Self, Self::Error> {
        match value {
            Settings::String(value) => Color::from_str(&value),
            _ => Err(format!("Couldn't convert {:?} to Color", value)),
        }
    }
}

impl IntrospectObject for Color {
    fn get_class_name(&self) -> String {
        "Color".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![
            FieldInfo {
                name: "r".to_string(),
                serializable: true,
                getter: Arc::new(|this| this.downcast_ref::<Color>().unwrap().r.fruity_into()),
                setter: SetterCaller::Mut(std::sync::Arc::new(|this, value| {
                    let this = this.downcast_mut::<Color>().unwrap();

                    match f32::fruity_try_from(value) {
                        Ok(value) => this.r = value,
                        Err(_) => {
                            log::error!("Expected a f32 for property r");
                        }
                    }
                })),
            },
            FieldInfo {
                name: "g".to_string(),
                serializable: true,
                getter: Arc::new(|this| this.downcast_ref::<Color>().unwrap().g.fruity_into()),
                setter: SetterCaller::Mut(std::sync::Arc::new(|this, value| {
                    let this = this.downcast_mut::<Color>().unwrap();

                    match f32::fruity_try_from(value) {
                        Ok(value) => this.g = value,
                        Err(_) => {
                            log::error!("Expected a f32 for property g");
                        }
                    }
                })),
            },
            FieldInfo {
                name: "b".to_string(),
                serializable: true,
                getter: Arc::new(|this| this.downcast_ref::<Color>().unwrap().b.fruity_into()),
                setter: SetterCaller::Mut(std::sync::Arc::new(|this, value| {
                    let this = this.downcast_mut::<Color>().unwrap();

                    match f32::fruity_try_from(value) {
                        Ok(value) => this.b = value,
                        Err(_) => {
                            log::error!("Expected a f32 for property b");
                        }
                    }
                })),
            },
            FieldInfo {
                name: "a".to_string(),
                serializable: true,
                getter: Arc::new(|this| this.downcast_ref::<Color>().unwrap().a.fruity_into()),
                setter: SetterCaller::Mut(std::sync::Arc::new(|this, value| {
                    let this = this.downcast_mut::<Color>().unwrap();

                    match f32::fruity_try_from(value) {
                        Ok(value) => this.a = value,
                        Err(_) => {
                            log::error!("Expected a f32 for property a");
                        }
                    }
                })),
            },
        ]
    }
}
