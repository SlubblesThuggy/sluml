use super::*;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    sluml_derive::IndexMut,
    sluml_derive::Add,
    sluml_derive::Sub,
    sluml_derive::Mul,
    sluml_derive::Div,
    sluml_derive::Neg,
    sluml_derive::Dot,
)]
pub struct Vec4<T: VecInner> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: VecInner> Vec4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

impl<T: VecInner> SqMagnitude<T> for Vec4<T> {}
impl<T: VecInner + Float> Magnitude<T> for Vec4<T> {
    fn magnitude(self) -> T {
        self.sq_magnitude().sqrt()
    }
}

impl<T: VecInner + Float> Normalize<T> for Vec4<T> {}
