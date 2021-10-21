use cgmath::SquareMatrix;

#[derive(Debug, Clone, Copy)]
pub struct Matrix4([[f32; 4]; 4]);

impl Matrix4 {
    pub fn identity() -> Matrix4 {
        Matrix4(cgmath::Matrix4::identity().into())
    }

    pub fn from_rect(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Matrix4 {
        Matrix4(cgmath::ortho(left, right, bottom, top, near, far).into())
    }
}

impl Into<[[f32; 4]; 4]> for Matrix4 {
    fn into(self) -> [[f32; 4]; 4] {
        self.0
    }
}
