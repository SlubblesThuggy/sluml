use super::*;

pub trait Dot<Rhs = Self>: Sized {
    type Output;

    fn dot(self, rhs: Rhs) -> Self::Output;
}

pub trait Cross<Rhs = Self>: Sized {
    type Output;

    fn cross(self, rhs: Rhs) -> Self::Output;
}

pub trait Magnitude<T>: Clone + Copy + Dot<Output = T> {
    fn sq_magnitude(self) -> T {
        self.dot(self)
    }

    fn magnitude(self) -> T;
}

pub trait Normalize<T>: Sized + Div<T, Output = Self> + Magnitude<T> {
    fn normalize(self) -> Self {
        self / self.magnitude()
    }
}

mod gen;
use gen::*;

pub mod vec2;
pub use vec2::*;

pub mod vec3;
pub use vec3::*;

pub mod vec4;
pub use vec4::*;

#[cfg(test)]
mod test;
