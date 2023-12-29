use super::*;

gen_vec!(Vec3f32, f32, x, y, z);
impl_vec!(@cross_vec3 Vec3f32, f32, x, y, z);

impl From<Vec3f64> for Vec3f32 {
    fn from(other: Vec3f64) -> Self {
        Self {
            x: other.x as f32,
            y: other.y as f32,
            z: other.z as f32,
        }
    }
}

impl From<Vec2f32> for Vec3f32 {
    fn from(other: Vec2f32) -> Self {
        Self {
            x: other.x,
            y: other.y,
            z: 0f32,
        }
    }
}

impl From<Vec2f64> for Vec3f32 {
    fn from(other: Vec2f64) -> Self {
        Self {
            x: other.x as f32,
            y: other.y as f32,
            z: 0f32,
        }
    }
}

impl From<Vec4f32> for Vec3f32 {
    fn from(other: Vec4f32) -> Self {
        Self {
            x: other.x,
            y: other.y,
            z: other.z,
        }
    }
}

impl From<Vec4f64> for Vec3f32 {
    fn from(other: Vec4f64) -> Self {
        Self {
            x: other.x as f32,
            y: other.y as f32,
            z: other.z as f32,
        }
    }
}

gen_vec!(Vec3f64, f64, x, y, z);
impl_vec!(@cross_vec3 Vec3f64, f64, x, y, z);

impl From<Vec3f32> for Vec3f64 {
    fn from(other: Vec3f32) -> Self {
        Self {
            x: other.x as f64,
            y: other.y as f64,
            z: other.z as f64,
        }
    }
}

impl From<Vec2f64> for Vec3f64 {
    fn from(other: Vec2f64) -> Self {
        Self {
            x: other.x,
            y: other.y,
            z: 0f64,
        }
    }
}

impl From<Vec2f32> for Vec3f64 {
    fn from(other: Vec2f32) -> Self {
        Self {
            x: other.x as f64,
            y: other.y as f64,
            z: 0f64,
        }
    }
}

impl From<Vec4f64> for Vec3f64 {
    fn from(other: Vec4f64) -> Self {
        Self {
            x: other.x,
            y: other.y,
            z: other.z,
        }
    }
}

impl From<Vec4f32> for Vec3f64 {
    fn from(other: Vec4f32) -> Self {
        Self {
            x: other.x as f64,
            y: other.y as f64,
            z: other.z as f64,
        }
    }
}
