/// A serialized value
#[derive(Debug, Clone)]
pub enum Serialized {
    /// i8 value
    I8(i8),

    /// i16 value
    I16(i16),

    /// i32 value
    I32(i32),

    /// i64 value
    I64(i64),

    /// isize value
    ISize(isize),

    /// u8 value
    U8(u8),

    /// u16 value
    U16(u16),

    /// u32 value
    U32(u32),

    /// u64 value
    U64(u64),

    /// usize value
    USize(usize),

    /// f32 value
    F32(f32),

    /// f64 value
    F64(f64),

    /// bool value
    Bool(bool),

    /// String value
    String(String),
}

impl Serialized {
    /// Convert as i8
    #[allow(dead_code)]
    pub fn as_i8(&self) -> Option<i8> {
        match self {
            Serialized::I8(value) => Some(*value),
            _ => None,
        }
    }

    /// Convert as i16
    #[allow(dead_code)]
    pub fn as_i16(&self) -> Option<i16> {
        match self {
            Serialized::I8(value) => Some(*value as i16),
            Serialized::I16(value) => Some(*value),
            Serialized::U8(value) => Some(*value as i16),
            _ => None,
        }
    }

    /// Convert as i32
    #[allow(dead_code)]
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Serialized::I8(value) => Some(*value as i32),
            Serialized::I16(value) => Some(*value as i32),
            Serialized::I32(value) => Some(*value as i32),
            Serialized::ISize(value) => Some(*value as i32),
            Serialized::U8(value) => Some(*value as i32),
            Serialized::U16(value) => Some(*value as i32),
            Serialized::F32(value) => Some(*value as i32),
            _ => None,
        }
    }

    /// Convert as i64
    #[allow(dead_code)]
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Serialized::I8(value) => Some(*value as i64),
            Serialized::I16(value) => Some(*value as i64),
            Serialized::I32(value) => Some(*value as i64),
            Serialized::I64(value) => Some(*value as i64),
            Serialized::ISize(value) => Some(*value as i64),
            Serialized::U8(value) => Some(*value as i64),
            Serialized::U16(value) => Some(*value as i64),
            Serialized::U32(value) => Some(*value as i64),
            Serialized::USize(value) => Some(*value as i64),
            Serialized::F32(value) => Some(*value as i64),
            Serialized::F64(value) => Some(*value as i64),
            _ => None,
        }
    }

    /// Convert as isize
    #[allow(dead_code)]
    pub fn as_isize(&self) -> Option<isize> {
        match self {
            Serialized::I8(value) => Some(*value as isize),
            Serialized::I16(value) => Some(*value as isize),
            Serialized::I32(value) => Some(*value as isize),
            Serialized::ISize(value) => Some(*value as isize),
            Serialized::U8(value) => Some(*value as isize),
            Serialized::U16(value) => Some(*value as isize),
            Serialized::F32(value) => Some(*value as isize),
            _ => None,
        }
    }

    /// Convert as u8
    #[allow(dead_code)]
    pub fn as_u8(&self) -> Option<u8> {
        match self {
            Serialized::U8(value) => Some(*value),
            _ => None,
        }
    }

    /// Convert as u16
    #[allow(dead_code)]
    pub fn as_u16(&self) -> Option<u16> {
        match self {
            Serialized::U8(value) => Some(*value as u16),
            Serialized::U16(value) => Some(*value as u16),
            _ => None,
        }
    }

    /// Convert as u32
    #[allow(dead_code)]
    pub fn as_u32(&self) -> Option<u32> {
        match self {
            Serialized::U8(value) => Some(*value as u32),
            Serialized::U16(value) => Some(*value as u32),
            Serialized::U32(value) => Some(*value as u32),
            Serialized::USize(value) => Some(*value as u32),
            _ => None,
        }
    }

    /// Convert as u64
    #[allow(dead_code)]
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Serialized::U8(value) => Some(*value as u64),
            Serialized::U16(value) => Some(*value as u64),
            Serialized::U32(value) => Some(*value as u64),
            Serialized::USize(value) => Some(*value as u64),
            Serialized::U64(value) => Some(*value as u64),
            _ => None,
        }
    }

    /// Convert as usize
    #[allow(dead_code)]
    pub fn as_usize(&self) -> Option<usize> {
        match self {
            Serialized::U8(value) => Some(*value as usize),
            Serialized::U16(value) => Some(*value as usize),
            Serialized::U32(value) => Some(*value as usize),
            Serialized::USize(value) => Some(*value as usize),
            _ => None,
        }
    }

    /// Convert as f32
    #[allow(dead_code)]
    pub fn as_f32(&self) -> Option<f32> {
        match self {
            Serialized::I8(value) => Some(*value as f32),
            Serialized::I16(value) => Some(*value as f32),
            Serialized::I32(value) => Some(*value as f32),
            Serialized::ISize(value) => Some(*value as f32),
            Serialized::U8(value) => Some(*value as f32),
            Serialized::U16(value) => Some(*value as f32),
            Serialized::U32(value) => Some(*value as f32),
            Serialized::USize(value) => Some(*value as f32),
            Serialized::F32(value) => Some(*value as f32),
            _ => None,
        }
    }

    /// Convert as f64
    #[allow(dead_code)]
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Serialized::I8(value) => Some(*value as f64),
            Serialized::I16(value) => Some(*value as f64),
            Serialized::I32(value) => Some(*value as f64),
            Serialized::I64(value) => Some(*value as f64),
            Serialized::ISize(value) => Some(*value as f64),
            Serialized::U8(value) => Some(*value as f64),
            Serialized::U16(value) => Some(*value as f64),
            Serialized::U32(value) => Some(*value as f64),
            Serialized::USize(value) => Some(*value as f64),
            Serialized::U64(value) => Some(*value as f64),
            Serialized::F32(value) => Some(*value as f64),
            Serialized::F64(value) => Some(*value as f64),
            _ => None,
        }
    }

    /// Convert as bool
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Serialized::Bool(value) => Some(*value),
            _ => None,
        }
    }

    /// Convert as bool
    #[allow(dead_code)]
    pub fn as_string(&self) -> Option<String> {
        match self {
            Serialized::String(value) => Some(value.clone()),
            _ => None,
        }
    }
}
