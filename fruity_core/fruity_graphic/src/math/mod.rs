use bytemuck::{Pod, Zeroable};
use css_color_parser::Color as CssColor;
use fruity_core::convert::FruityTryFrom;
use fruity_core::settings::Settings;
use std::str::FromStr;

pub mod material_reference;
pub mod matrix3;
pub mod matrix4;
pub mod vector2d;

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Color(pub [f32; 4]);

impl FromStr for Color {
    type Err = String;

    fn from_str(string: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let css = string
            .parse::<CssColor>()
            .map_err(|_| "Parse color failed".to_string())?;

        Ok(Color([
            css.r as f32 / 255.0,
            css.g as f32 / 255.0,
            css.b as f32 / 255.0,
            css.a,
        ]))
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

pub static WHITE: Color = Color([1.0, 1.0, 1.0, 1.0]);
pub static BLACK: Color = Color([0.0, 0.0, 0.0, 1.0]);
pub static ALPHA: Color = Color([0.0, 0.0, 0.0, 0.0]);
pub static RED: Color = Color([1.0, 0.0, 0.0, 1.0]);
pub static GREEN: Color = Color([0.0, 1.0, 0.0, 1.0]);
pub static BLUE: Color = Color([0.0, 0.0, 1.0, 1.0]);
