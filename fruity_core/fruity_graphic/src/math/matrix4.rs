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
pub struct Matrix4(pub [[f32; 4]; 4]);

impl Matrix4 {
    pub fn identity() -> Matrix4 {
        Matrix4(cgmath::Matrix4::identity().into())
    }

    pub fn from_rect(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Matrix4 {
        Matrix4(cgmath::ortho(left, right, bottom, top, near, far).into())
    }

    pub fn invert(&self) -> Matrix4 {
        if let Some(result) = cgmath::Matrix4::from(self.0).invert() {
            Matrix4(result.into())
        } else {
            Matrix4::identity()
        }
    }
}

impl Into<[[f32; 4]; 4]> for Matrix4 {
    fn into(self) -> [[f32; 4]; 4] {
        self.0
    }
}

impl Default for Matrix4 {
    fn default() -> Self {
        Matrix4::identity()
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        let result = cgmath::Matrix4::from(self.0) * cgmath::Matrix4::from(rhs.0);
        Matrix4(result.into())
    }
}

impl FruityTryFrom<Serialized> for Matrix4 {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => match value.as_any_box().downcast::<Matrix4>() {
                Ok(value) => Ok(*value),
                Err(_) => Err(format!("Couldn't convert a Matrix4 to native object")),
            },
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}

impl FruityInto<Serialized> for Matrix4 {
    fn fruity_into(self) -> Serialized {
        Serialized::NativeObject(Box::new(self))
    }
}

impl IntrospectObject for Matrix4 {
    fn get_class_name(&self) -> String {
        "Matrix4".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![MethodInfo {
            name: "invert".to_string(),
            call: MethodCaller::Const(Arc::new(|this, _args| {
                let this = cast_introspect_ref::<Matrix4>(this);
                let result = this.invert();

                Ok(Some(result.fruity_into()))
            })),
        }]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl SerializableObject for Matrix4 {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}
