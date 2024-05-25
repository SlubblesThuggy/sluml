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

pub trait VecInner:
    Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
{
}

impl VecInner for f32 {}
impl VecInner for f64 {}

pub mod vec4;
pub use vec4::*;

#[cfg(test)]
mod test;
