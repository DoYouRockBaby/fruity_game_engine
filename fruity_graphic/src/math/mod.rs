use bytemuck::{Pod, Zeroable};
use cgmath::SquareMatrix;
use css_color_parser::Color as CssColor;
use fruity_core::settings::Settings;
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct Matrix4(pub [[f32; 4]; 4]);

impl Matrix4 {
    pub fn identity() -> Matrix4 {
        Matrix4(cgmath::Matrix4::identity().into())
    }

    pub fn from_rect(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Matrix4 {
        Matrix4(cgmath::ortho(left, right, bottom, top, near, far).into())
    }

    pub fn invert(&self) -> Matrix4 {
        Matrix4(cgmath::Matrix4::from(self.0).invert().unwrap().into())
    }
}

impl Into<[[f32; 4]; 4]> for Matrix4 {
    fn into(self) -> [[f32; 4]; 4] {
        self.0
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Color([f32; 4]);

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

impl TryFrom<Settings> for Color {
    type Error = String;

    fn try_from(value: Settings) -> Result<Self, Self::Error> {
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
