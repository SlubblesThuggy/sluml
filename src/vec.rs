use super::*;

pub trait Dot<Rhs = Self> {
    type Output;

    fn dot(self, rhs: Rhs) -> Self::Output;
}

pub trait Cross<Rhs = Self> {
    type Output;

    fn cross(self, rhs: Rhs) -> Self::Output;
}

pub trait SqMagnitude<T>: Copy + Dot<Output = T> {
    fn sq_magnitude(self) -> T {
        self.dot(self)
    }
}

pub trait Magnitude<T>: SqMagnitude<T> {
    fn magnitude(self) -> T;
}

pub trait Normalize<T>: Div<T, Output = Self> + Magnitude<T> {
    fn normalize(self) -> Self {
        self / self.magnitude()
    }
}

impl<V: Copy + Dot<Output = T>, T> SqMagnitude<T> for V {}
impl<V: SqMagnitude<T>, T: Float> Magnitude<T> for V {
    fn magnitude(self) -> T {
        self.sq_magnitude().sqrt()
    }
}

impl<V: Magnitude<T> + Div<T, Output = Self>, T: Float> Normalize<T> for V {}

pub mod vec4;
pub use vec4::*;

#[cfg(test)]
mod test;
