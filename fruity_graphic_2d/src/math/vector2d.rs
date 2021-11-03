use fruity_any::*;
use fruity_core::service::utils::cast_service;
use fruity_core::service::utils::ArgumentCaster;
use fruity_core::*;
use fruity_graphic::math::Matrix4;
use fruity_introspect::serializable_object::SerializableObject;
use fruity_introspect::serialized::Serialized;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use fruity_introspect::SetterCaller;
use std::any::TypeId;
use std::convert::TryFrom;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::sync::Arc;
use std::sync::RwLock;

/// A vector in 2D dimension
#[derive(Debug, Clone, Copy, Default, FruityAny, InstantiableObject)]
pub struct Vector2d {
    /// Horizontal component
    pub x: f32,

    /// Vertical component
    pub y: f32,
}

impl Vector2d {
    /// Create a new `Vector2D` with the provided components.
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Returns a vector with only the horizontal component of the current one
    ///
    /// # Example
    /// ```
    /// use vector2d::Vector2D;
    /// let v = Vector2D::new(10, 20);
    /// assert_eq!(Vector2D::new(10, 0), v.horizontal());
    /// ```
    pub fn horizontal(self) -> Self {
        Self {
            x: self.x,
            y: Default::default(),
        }
    }

    /// Returns a vector with only the vertical component of the current one
    ///
    /// # Example
    /// ```
    /// use vector2d::Vector2D;
    /// let v = Vector2D::new(10, 20);
    /// assert_eq!(Vector2D::new(0, 20), v.vertical());
    pub fn vertical(self) -> Self {
        Self {
            x: Default::default(),
            y: self.y,
        }
    }

    /// Returns a vector perpendicular to the current one.
    ///
    /// # Example
    /// ```
    /// use vector2d::Vector2D;
    /// let v = Vector2D::new(21.3, -98.1);
    /// assert_eq!(Vector2D::new(98.1, 21.3), v.normal());
    /// ```
    pub fn normal(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    /// Get the scalar/dot product of the two `Vector2D`.
    pub fn dot(self, v2: Self) -> f32 {
        self.x * v2.x + self.y * v2.y
    }

    /// Get the squared length of a `Vector2D`. This is more performant than using
    /// `length()` -- which is only available for `Vector2D<f32>` and `Vector2D<f64>`
    /// -- as it does not perform any square root operation.
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
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

    /// Get the vector's direction in radians.
    pub fn angle(self) -> f32 {
        self.y.atan2(self.x)
    }
}

// Ops Implementations
impl Add<Vector2d> for Vector2d {
    type Output = Vector2d;

    fn add(self, rhs: Vector2d) -> Self::Output {
        Vector2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Vector2d> for Vector2d {
    fn add_assign(&mut self, rhs: Vector2d) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl Sub<Vector2d> for Vector2d {
    type Output = Vector2d;

    fn sub(self, rhs: Vector2d) -> Self::Output {
        Vector2d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Vector2d> for Vector2d {
    fn sub_assign(&mut self, rhs: Vector2d) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl Mul<f32> for Vector2d {
    type Output = Vector2d;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vector2d> for Matrix4 {
    type Output = Vector2d;

    fn mul(self, rhs: Vector2d) -> Self::Output {
        Vector2d {
            x: self.0[0][0] * rhs.x + self.0[0][1] * rhs.y,
            y: self.0[1][0] * rhs.x + self.0[1][1] * rhs.y,
        }
    }
}

impl MulAssign<f32> for Vector2d {
    fn mul_assign(&mut self, rhs: f32) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl Div<f32> for Vector2d {
    type Output = Vector2d;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<f32> for Vector2d {
    fn div_assign(&mut self, rhs: f32) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}

impl TryFrom<Serialized> for Vector2d {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => {
                match value.as_any_box().downcast::<Arc<RwLock<Vector2d>>>() {
                    Ok(value) => {
                        let value = value.read().unwrap();
                        Ok(*value)
                    }
                    Err(_) => Err(format!("Couldn't convert a Vector2d to native object")),
                }
            }
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}

impl Into<Serialized> for Vector2d {
    fn into(self) -> Serialized {
        Serialized::NativeObject(Box::new(Arc::new(RwLock::new(self))))
    }
}

impl IntrospectObject for Vector2d {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![
            MethodInfo {
                name: "horizontal".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_service::<Vector2d>(this);
                    let result = this.horizontal();

                    Ok(Some(result.into()))
                })),
            },
            MethodInfo {
                name: "vertical".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_service::<Vector2d>(this);
                    let result = this.vertical();

                    Ok(Some(result.into()))
                })),
            },
            MethodInfo {
                name: "normal".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_service::<Vector2d>(this);
                    let result = this.normal();

                    Ok(Some(result.into()))
                })),
            },
            MethodInfo {
                name: "dot".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_service::<Vector2d>(this);

                    let mut caster = ArgumentCaster::new("dot", args);
                    let arg1 = caster.cast_next::<Vector2d>()?;

                    let result = this.dot(arg1);

                    Ok(Some(result.into()))
                })),
            },
            MethodInfo {
                name: "length_squared".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_service::<Vector2d>(this);
                    let result = this.length_squared();

                    Ok(Some(result.into()))
                })),
            },
            MethodInfo {
                name: "lerp".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_service::<Vector2d>(this);

                    let mut caster = ArgumentCaster::new("lerp", args);
                    let arg1 = caster.cast_next::<Vector2d>()?;
                    let arg2 = caster.cast_next::<f32>()?;

                    let result = this.lerp(arg1, arg2);

                    Ok(Some(result.into()))
                })),
            },
            MethodInfo {
                name: "length".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_service::<Vector2d>(this);
                    let result = this.length();

                    Ok(Some(result.into()))
                })),
            },
            MethodInfo {
                name: "normalise".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_service::<Vector2d>(this);
                    let result = this.normalise();

                    Ok(Some(result.into()))
                })),
            },
            MethodInfo {
                name: "angle".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_service::<Vector2d>(this);
                    let result = this.angle();

                    Ok(Some(result.into()))
                })),
            },
            MethodInfo {
                name: "add".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_service::<Vector2d>(this);

                    let mut caster = ArgumentCaster::new("add", args);
                    let arg1 = caster.cast_next::<Vector2d>()?;

                    let result = this.add(arg1);

                    Ok(Some(result.into()))
                })),
            },
            MethodInfo {
                name: "sub".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_service::<Vector2d>(this);

                    let mut caster = ArgumentCaster::new("sub", args);
                    let arg1 = caster.cast_next::<Vector2d>()?;

                    let result = this.sub(arg1);

                    Ok(Some(result.into()))
                })),
            },
            MethodInfo {
                name: "mul".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_service::<Vector2d>(this);

                    let mut caster = ArgumentCaster::new("mul", args);
                    let arg1 = caster.cast_next::<f32>()?;

                    let result = this.mul(arg1);

                    Ok(Some(result.into()))
                })),
            },
            MethodInfo {
                name: "div".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_service::<Vector2d>(this);

                    let mut caster = ArgumentCaster::new("div", args);
                    let arg1 = caster.cast_next::<f32>()?;

                    let result = this.div(arg1);

                    Ok(Some(result.into()))
                })),
            },
        ]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![
            FieldInfo {
                name: "x".to_string(),
                ty: TypeId::of::<f32>(),
                getter: Arc::new(|this| this.downcast_ref::<Vector2d>().unwrap().x.into()),
                setter: SetterCaller::Mut(std::sync::Arc::new(|this, value| {
                    let this = this.downcast_mut::<Vector2d>().unwrap();

                    match f32::try_from(value) {
                        Ok(value) => this.x = value,
                        Err(_) => {
                            log::error!("Expected a f32 for property x");
                        }
                    }
                })),
            },
            FieldInfo {
                name: "y".to_string(),
                ty: TypeId::of::<f32>(),
                getter: Arc::new(|this| this.downcast_ref::<Vector2d>().unwrap().y.into()),
                setter: SetterCaller::Mut(std::sync::Arc::new(|this, value| {
                    let this = this.downcast_mut::<Vector2d>().unwrap();

                    match f32::try_from(value) {
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

impl SerializableObject for Vector2d {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}
