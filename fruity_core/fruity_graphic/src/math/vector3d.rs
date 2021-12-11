use crate::math::matrix4::Matrix4;
use fruity_any::*;
use fruity_core::convert::FruityInto;
use fruity_core::convert::FruityTryFrom;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodCaller;
use fruity_core::introspect::MethodInfo;
use fruity_core::introspect::SetterCaller;
use fruity_core::utils::introspect::cast_introspect_ref;
use fruity_core::utils::introspect::ArgumentCaster;
use fruity_ecs::*;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::sync::Arc;

/// A vector in 3D dimension
#[repr(C)]
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    FruityAny,
    SerializableObject,
    InstantiableObject,
    bytemuck::Pod,
    bytemuck::Zeroable,
)]
pub struct Vector3d {
    /// Horizontal component
    pub x: f32,

    /// Vertical component
    pub y: f32,

    /// Depth component
    pub z: f32,
}

impl Vector3d {
    /// Create a new `Vector3D` with the provided components.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Returns a vector with only the horizontal component of the current one
    ///
    /// # Example
    /// ```
    /// use vector3d::Vector3D;
    /// let v = Vector3D::new(10, 20, 40);
    /// assert_eq!(Vector3D::new(10, 0, 0), v.horizontal());
    /// ```
    pub fn horizontal(self) -> Self {
        Self {
            x: self.x,
            y: Default::default(),
            z: Default::default(),
        }
    }

    /// Returns a vector with only the vertical component of the current one
    ///
    /// # Example
    /// ```
    /// use vector3d::Vector3D;
    /// let v = Vector3D::new(10, 20, 40);
    /// assert_eq!(Vector3D::new(0, 20, 0), v.vertical());
    pub fn vertical(self) -> Self {
        Self {
            x: Default::default(),
            y: self.y,
            z: Default::default(),
        }
    }

    /// Returns a vector with only the depth component of the current one
    ///
    /// # Example
    /// ```
    /// use vector3d::Vector3D;
    /// let v = Vector3D::new(10, 20, 40);
    /// assert_eq!(Vector3D::new(0, 0, 40), v.depth());
    pub fn depth(self) -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: self.z,
        }
    }

    /// Get the scalar/dot product of the two `Vector3D`.
    pub fn dot(self, v2: Self) -> f32 {
        self.x * v2.x + self.y * v2.y + self.z * v2.z
    }

    /// Get the squared length of a `Vector3D`. This is more performant than using
    /// `length()` -- which is only available for `Vector3D<f32>` and `Vector3D<f64>`
    /// -- as it does not perform any square root operation.
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Linearly interpolates between two vectors
    pub fn lerp(self, end: Self, progress: f32) -> Self {
        self + ((end - self) * progress)
    }

    /// Get the length of the vector. If possible, favour `length_squared()` over
    /// this function, as it is more performant.
    pub fn length(self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    /// Get a new vector with the same direction as this vector, but with a length
    /// of 1.0. If the the length of the vector is 0, then the original vector is
    /// returned.
    pub fn normalise(self) -> Self {
        let len = self.length();
        if len == 0.0 {
            self
        } else {
            self / len
        }
    }
}

// Ops Implementations
impl Add<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: Vector3d) -> Self::Output {
        Vector3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<Vector3d> for Vector3d {
    fn add_assign(&mut self, rhs: Vector3d) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl Sub<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn sub(self, rhs: Vector3d) -> Self::Output {
        Vector3d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign<Vector3d> for Vector3d {
    fn sub_assign(&mut self, rhs: Vector3d) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

impl Mul<f32> for Vector3d {
    type Output = Vector3d;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3d {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vector3d> for Matrix4 {
    type Output = Vector3d;

    fn mul(self, rhs: Vector3d) -> Self::Output {
        let cgmath_vec = cgmath::Vector4::new(rhs.x, rhs.y, rhs.z, 1.0);
        let cgmath_matrix = cgmath::Matrix4::from(self.0);
        let cgmath_result = cgmath_matrix * cgmath_vec;

        Vector3d {
            x: cgmath_result.x,
            y: cgmath_result.y,
            z: cgmath_result.z,
        }
    }
}

impl MulAssign<f32> for Vector3d {
    fn mul_assign(&mut self, rhs: f32) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl Div<f32> for Vector3d {
    type Output = Vector3d;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f32> for Vector3d {
    fn div_assign(&mut self, rhs: f32) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

impl IntrospectObject for Vector3d {
    fn get_class_name(&self) -> String {
        "Vector3d".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![
            MethodInfo {
                name: "horizontal".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_introspect_ref::<Vector3d>(this);
                    let result = this.horizontal();

                    Ok(Some(result.fruity_into()))
                })),
            },
            MethodInfo {
                name: "vertical".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_introspect_ref::<Vector3d>(this);
                    let result = this.vertical();

                    Ok(Some(result.fruity_into()))
                })),
            },
            MethodInfo {
                name: "depth".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_introspect_ref::<Vector3d>(this);
                    let result = this.depth();

                    Ok(Some(result.fruity_into()))
                })),
            },
            MethodInfo {
                name: "dot".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_introspect_ref::<Vector3d>(this);

                    let mut caster = ArgumentCaster::new("dot", args);
                    let arg1 = caster.cast_next::<Vector3d>()?;

                    let result = this.dot(arg1);

                    Ok(Some(result.fruity_into()))
                })),
            },
            MethodInfo {
                name: "length_squared".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_introspect_ref::<Vector3d>(this);
                    let result = this.length_squared();

                    Ok(Some(result.fruity_into()))
                })),
            },
            MethodInfo {
                name: "lerp".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_introspect_ref::<Vector3d>(this);

                    let mut caster = ArgumentCaster::new("lerp", args);
                    let arg1 = caster.cast_next::<Vector3d>()?;
                    let arg2 = caster.cast_next::<f32>()?;

                    let result = this.lerp(arg1, arg2);

                    Ok(Some(result.fruity_into()))
                })),
            },
            MethodInfo {
                name: "length".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_introspect_ref::<Vector3d>(this);
                    let result = this.length();

                    Ok(Some(result.fruity_into()))
                })),
            },
            MethodInfo {
                name: "normalise".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_introspect_ref::<Vector3d>(this);
                    let result = this.normalise();

                    Ok(Some(result.fruity_into()))
                })),
            },
            MethodInfo {
                name: "add".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_introspect_ref::<Vector3d>(this);

                    let mut caster = ArgumentCaster::new("add", args);
                    let arg1 = caster.cast_next::<Vector3d>()?;

                    let result = this.add(arg1);

                    Ok(Some(result.fruity_into()))
                })),
            },
            MethodInfo {
                name: "sub".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_introspect_ref::<Vector3d>(this);

                    let mut caster = ArgumentCaster::new("sub", args);
                    let arg1 = caster.cast_next::<Vector3d>()?;

                    let result = this.sub(arg1);

                    Ok(Some(result.fruity_into()))
                })),
            },
            MethodInfo {
                name: "mul".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_introspect_ref::<Vector3d>(this);

                    let mut caster = ArgumentCaster::new("mul", args);
                    let arg1 = caster.cast_next::<f32>()?;

                    let result = this.mul(arg1);

                    Ok(Some(result.fruity_into()))
                })),
            },
            MethodInfo {
                name: "div".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_introspect_ref::<Vector3d>(this);

                    let mut caster = ArgumentCaster::new("div", args);
                    let arg1 = caster.cast_next::<f32>()?;

                    let result = this.div(arg1);

                    Ok(Some(result.fruity_into()))
                })),
            },
        ]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![
            FieldInfo {
                name: "x".to_string(),
                serializable: true,
                getter: Arc::new(|this| this.downcast_ref::<Vector3d>().unwrap().x.fruity_into()),
                setter: SetterCaller::Mut(std::sync::Arc::new(|this, value| {
                    let this = this.downcast_mut::<Vector3d>().unwrap();

                    match f32::fruity_try_from(value) {
                        Ok(value) => this.x = value,
                        Err(_) => {
                            log::error!("Expected a f32 for property x");
                        }
                    }
                })),
            },
            FieldInfo {
                name: "y".to_string(),
                serializable: true,
                getter: Arc::new(|this| this.downcast_ref::<Vector3d>().unwrap().y.fruity_into()),
                setter: SetterCaller::Mut(std::sync::Arc::new(|this, value| {
                    let this = this.downcast_mut::<Vector3d>().unwrap();

                    match f32::fruity_try_from(value) {
                        Ok(value) => this.y = value,
                        Err(_) => {
                            log::error!("Expected a f32 for property y");
                        }
                    }
                })),
            },
        ]
    }
}
