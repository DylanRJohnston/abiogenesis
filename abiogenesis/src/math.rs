#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[inline]
pub fn inverse_lerp(a: f32, b: f32, value: f32) -> f32 {
    (value - a) / (b - a)
}

#[inline]
pub fn remap(value: f32, a: f32, b: f32, c: f32, d: f32) -> f32 {
    lerp(c, d, inverse_lerp(a, b, value))
}
