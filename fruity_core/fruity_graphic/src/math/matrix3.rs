use crate::math::vector2d::Vector2d;
use crate::Matrix4;
use cgmath::Angle;
use cgmath::Rad;
use cgmath::SquareMatrix;
use fruity_any::*;
use fruity_core::convert::FruityInto;
use fruity_core::convert::FruityTryFrom;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodCaller;
use fruity_core::introspect::MethodInfo;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_core::serialize::serialized::Serialized;
use fruity_core::utils::introspect::cast_introspect_ref;
use fruity_ecs::*;
use std::ops::Mul;
use std::sync::Arc;

#[derive(Debug, FruityAny, Clone, Copy, InstantiableObject)]
pub struct Matrix3(pub [[f32; 3]; 3]);

impl Matrix3 {
    pub fn identity() -> Matrix3 {
        Matrix3(cgmath::Matrix3::identity().into())
    }

    pub fn translation(vec: Vector2d) -> Matrix3 {
        Matrix3(
            cgmath::Matrix3::from_translation(cgmath::Vector2::<f32> { x: vec.x, y: vec.y }).into(),
        )
    }

    pub fn rotation(angle: f32) -> Matrix3 {
        let (s, c) = Rad::sin_cos(Rad(angle));
        Matrix3([[c, s, 0.0], [-s, c, 0.0], [0.0, 0.0, 1.0]])
    }

    pub fn scaling(vec: Vector2d) -> Matrix3 {
        Matrix3(cgmath::Matrix3::from_nonuniform_scale(vec.x, vec.y).into())
    }

    pub fn invert(&self) -> Matrix3 {
        if let Some(result) = cgmath::Matrix3::from(self.0).invert() {
            Matrix3(result.into())
        } else {
            Matrix3::identity()
        }
    }
}

impl Into<[[f32; 3]; 3]> for Matrix3 {
    fn into(self) -> [[f32; 3]; 3] {
        self.0
    }
}

impl Into<Matrix4> for Matrix3 {
    fn into(self) -> Matrix4 {
        Matrix4([
            [self.0[0][0], self.0[0][1], 0.0, self.0[0][2]],
            [self.0[1][0], self.0[1][1], 0.0, self.0[1][2]],
            [0.0, 0.0, 1.0, 0.0],
            [self.0[2][0], self.0[2][1], 0.0, self.0[2][2]],
        ])
    }
}

impl Default for Matrix3 {
    fn default() -> Self {
        Matrix3::identity()
    }
}

impl Mul<Matrix3> for Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Self::Output {
        let result = cgmath::Matrix3::from(self.0) * cgmath::Matrix3::from(rhs.0);
        Matrix3(result.into())
    }
}

impl FruityTryFrom<Serialized> for Matrix3 {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => match value.as_any_box().downcast::<Matrix3>() {
                Ok(value) => Ok(*value),
                Err(_) => Err(format!("Couldn't convert a Matrix3 to native object")),
            },
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}

impl FruityInto<Serialized> for Matrix3 {
    fn fruity_into(self) -> Serialized {
        Serialized::NativeObject(Box::new(self))
    }
}

impl IntrospectObject for Matrix3 {
    fn get_class_name(&self) -> String {
        "Matrix3".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![MethodInfo {
            name: "invert".to_string(),
            call: MethodCaller::Const(Arc::new(|this, _args| {
                let this = cast_introspect_ref::<Matrix3>(this);
                let result = this.invert();

                Ok(Some(result.fruity_into()))
            })),
        }]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl SerializableObject for Matrix3 {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}
