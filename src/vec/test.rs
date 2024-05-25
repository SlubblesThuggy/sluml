use super::*;

// Testing of most macro generated functions will currently only be done for Vec4.

#[test]
fn index_mut() {
    let mut v = Vec4::<f32>::new(0f32, 1f32, 2f32, 3f32);

    assert_eq!(v[0], 0f32);
    assert_eq!(v[1], 1f32);
    assert_eq!(v[2], 2f32);
    assert_eq!(v[3], 3f32);

    v[0] = 4f32;

    assert_eq!(v[0], 4f32);
}

#[test]
fn add() {
    assert_eq!(
        Vec4::<f32>::new(0f32, 1f32, 2f32, 3f32) + Vec4::<f32>::new(4f32, 5f32, 6f32, 7f32),
        Vec4::<f32>::new(4f32, 6f32, 8f32, 10f32)
    );
}

#[test]
fn sub() {
    assert_eq!(
        Vec4::<f32>::new(4f32, 5f32, 6f32, 7f32) - Vec4::<f32>::new(3f32, 2f32, 1f32, 0f32),
        Vec4::<f32>::new(1f32, 3f32, 5f32, 7f32)
    );
}

#[test]
fn mul() {
    assert_eq!(
        Vec4::<f32>::new(0f32, 1f32, 2f32, 3f32) * Vec4::<f32>::new(4f32, 5f32, 6f32, 7f32),
        Vec4::<f32>::new(0f32, 5f32, 12f32, 21f32)
    );
}

#[test]
fn div() {
    assert_eq!(
        Vec4::<f32>::new(4f32, 5f32, 6f32, 16f32) / Vec4::<f32>::new(2f32, 1f32, 12f32, 4f32),
        Vec4::<f32>::new(2f32, 5f32, 0.5f32, 4f32)
    );
}

#[test]
fn neg() {
    assert_eq!(
        -Vec4::<f32>::new(1f32, 2f32, 3f32, 4f32),
        Vec4::<f32>::new(-1f32, -2f32, -3f32, -4f32)
    );
}

#[test]
fn dot() {
    assert_eq!(
        Vec4::<f32>::new(4f32, 5f32, 6f32, 16f32).dot(Vec4::<f32>::new(2f32, 1f32, 12f32, 4f32)),
        4f32 * 2f32 + 5f32 * 1f32 + 6f32 * 12f32 + 16f32 * 4f32
    );
}

/*
#[test]
fn magnitude() {
    let v = Vec4f32::new(-4f32, -5f32, 6f32, 16f32);
    let sqm = -4f32 * -4f32 + -5f32 * -5f32 + 6f32 * 6f32 + 16f32 * 16f32;
    let m = sqm.sqrt();

    assert_eq!(v.sq_magnitude(), sqm);
    assert_eq!(v.magnitude(), m);
}

#[test]
fn normalize() {
    assert_eq!(
        Vec4f32::new(0f32, 1f32, 0f32, 1f32).normalize(),
        Vec4f32::new(0f32, 1f32 / SQRT_2, 0f32, 1f32 / SQRT_2)
    );
}

#[test]
fn cross() {
    assert_eq!(
        Vec3f32::new(1f32, 0f32, 0f32).cross(Vec3f32::new(0f32, 1f32, 0f32)),
        Vec3f32::new(0f32, 0f32, 1f32)
    );
    assert_eq!(
        Vec3f32::new(1f32, 0f32, 0f32).cross(Vec3f32::new(-1f32, 0f32, 0f32)),
        Vec3f32::new(0f32, 0f32, 0f32)
    );
}
*/
