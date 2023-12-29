use super::*;

gen_vec!(Vec2f32, f32, x, y);

impl From<Vec2f64> for Vec2f32 {
    fn from(other: Vec2f64) -> Self {
        Self {
            x: other.x as f32,
            y: other.y as f32,
        }
    }
}

impl From<Vec3f32> for Vec2f32 {
    fn from(other: Vec3f32) -> Self {
        Self {
            x: other.x,
            y: other.y,
        }
    }
}

impl From<Vec3f64> for Vec2f32 {
    fn from(other: Vec3f64) -> Self {
        Self {
            x: other.x as f32,
            y: other.y as f32,
        }
    }
}

impl From<Vec4f32> for Vec2f32 {
    fn from(other: Vec4f32) -> Self {
        Self {
            x: other.x,
            y: other.y,
        }
    }
}

impl From<Vec4f64> for Vec2f32 {
    fn from(other: Vec4f64) -> Self {
        Self {
            x: other.x as f32,
            y: other.y as f32,
        }
    }
}

gen_vec!(Vec2f64, f64, x, y);

impl From<Vec2f32> for Vec2f64 {
    fn from(other: Vec2f32) -> Self {
        Self {
            x: other.x as f64,
            y: other.y as f64,
        }
    }
}

impl From<Vec3f64> for Vec2f64 {
    fn from(other: Vec3f64) -> Self {
        Self {
            x: other.x,
            y: other.y,
        }
    }
}

impl From<Vec3f32> for Vec2f64 {
    fn from(other: Vec3f32) -> Self {
        Self {
            x: other.x as f64,
            y: other.y as f64,
        }
    }
}

impl From<Vec4f64> for Vec2f64 {
    fn from(other: Vec4f64) -> Self {
        Self {
            x: other.x,
            y: other.y,
        }
    }
}

impl From<Vec4f32> for Vec2f64 {
    fn from(other: Vec4f32) -> Self {
        Self {
            x: other.x as f64,
            y: other.y as f64,
        }
    }
}
