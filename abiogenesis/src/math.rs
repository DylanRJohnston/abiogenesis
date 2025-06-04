use bevy::math::{Rect, Vec2};

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

pub trait TorodialMath {
    fn toroidal_displacement(&self, a: Vec2, b: Vec2) -> Vec2;
    fn toroidal_wrap(&self, pos: Vec2) -> Vec2;
}

impl TorodialMath for Rect {
    fn toroidal_displacement(&self, a: Vec2, b: Vec2) -> Vec2 {
        let width = self.max.x - self.min.x;
        let height = self.max.y - self.min.y;

        let mut dx = b.x - a.x;
        let mut dy = b.y - a.y;

        // Adjust x displacement for wrapping
        if dx > width / 2.0 {
            dx -= width;
        } else if dx < -width / 2.0 {
            dx += width;
        }

        // Adjust y displacement for wrapping
        if dy > height / 2.0 {
            dy -= height;
        } else if dy < -height / 2.0 {
            dy += height;
        }

        Vec2::new(dx, dy)
    }

    fn toroidal_wrap(&self, pos: Vec2) -> Vec2 {
        let width = self.max.x - self.min.x;
        let height = self.max.y - self.min.y;

        let mut x = pos.x;
        let mut y = pos.y;

        // Wrap x coordinate
        while x > self.max.x {
            x -= width;
        }
        while x < self.min.x {
            x += width;
        }

        // Wrap y coordinate
        while y > self.max.y {
            y -= height;
        }
        while y < self.min.y {
            y += height;
        }

        Vec2::new(x, y)
    }
}

#[cfg(test)]
mod test {
    mod toroidal_displacement {

        use bevy::math::{Rect, Vec2};

        use super::super::TorodialMath;

        #[test]
        fn zero() {
            assert_eq!(
                Rect::from_center_size(Vec2::ZERO, Vec2::splat(100.))
                    .toroidal_displacement(Vec2::ZERO, Vec2::ZERO),
                Vec2::ZERO
            );
        }

        #[test]
        fn diagonal() {
            assert_eq!(
                Rect::from_center_size(Vec2::ZERO, Vec2::splat(100.))
                    .toroidal_displacement(Vec2::ZERO, Vec2::splat(25.0)),
                Vec2::splat(25.0)
            );
        }

        #[test]
        fn wrap_x() {
            assert_eq!(
                Rect::from_center_size(Vec2::ZERO, Vec2::splat(100.))
                    .toroidal_displacement(Vec2::ZERO, Vec2::new(75.0, 0.0)),
                Vec2::new(-25.0, 0.0)
            );
        }

        #[test]
        fn wrap_y() {
            assert_eq!(
                Rect::from_center_size(Vec2::ZERO, Vec2::splat(100.))
                    .toroidal_displacement(Vec2::ZERO, Vec2::new(0.0, 75.0)),
                Vec2::new(0.0, -25.0)
            );
        }

        #[test]
        fn wrap_xy() {
            assert_eq!(
                Rect::from_center_size(Vec2::ZERO, Vec2::splat(100.))
                    .toroidal_displacement(Vec2::ZERO, Vec2::new(75.0, 75.0)),
                Vec2::new(-25.0, -25.0)
            );
        }

        #[test]
        fn non_zero_origin() {
            assert_eq!(
                Rect::from_center_size(Vec2::new(50., -25.), Vec2::splat(100.))
                    .toroidal_displacement(Vec2::new(90.0, 23.0), Vec2::new(10.0, -74.0)),
                Vec2::new(20.0, 3.0)
            );
        }
    }

    mod toroidal_wrap {
        use bevy::math::{Rect, Vec2};

        use super::super::TorodialMath;

        #[test]
        fn zero() {
            assert_eq!(
                Rect::from_center_size(Vec2::ZERO, Vec2::splat(100.0)).toroidal_wrap(Vec2::ZERO),
                Vec2::ZERO
            );
        }

        #[test]
        fn within_bounds() {
            assert_eq!(
                Rect::from_center_size(Vec2::ZERO, Vec2::splat(100.0))
                    .toroidal_wrap(Vec2::new(25.0, 25.0)),
                Vec2::new(25.0, 25.0)
            );
        }

        #[test]
        fn within_bounds_negative() {
            assert_eq!(
                Rect::from_center_size(Vec2::ZERO, Vec2::splat(100.0))
                    .toroidal_wrap(Vec2::new(-25.0, -25.0)),
                Vec2::new(-25.0, -25.0)
            );
        }

        #[test]
        fn wrap_x() {
            assert_eq!(
                Rect::from_center_size(Vec2::ZERO, Vec2::splat(100.0))
                    .toroidal_wrap(Vec2::new(75.0, 0.0)),
                Vec2::new(-25.0, 0.0)
            );
        }

        #[test]
        fn wrap_y() {
            assert_eq!(
                Rect::from_center_size(Vec2::ZERO, Vec2::splat(100.0))
                    .toroidal_wrap(Vec2::new(0.0, 75.0)),
                Vec2::new(0.0, -25.0)
            );
        }

        #[test]
        fn wrap_xy() {
            assert_eq!(
                Rect::from_center_size(Vec2::ZERO, Vec2::splat(100.0))
                    .toroidal_wrap(Vec2::new(75.0, 75.0)),
                Vec2::new(-25.0, -25.0)
            );
        }

        #[test]
        fn non_zero_origin_within_bounds() {
            assert_eq!(
                Rect::from_center_size(Vec2::new(100.0, 50.0), Vec2::splat(80.0))
                    .toroidal_wrap(Vec2::new(120.0, 70.0)),
                Vec2::new(120.0, 70.0)
            );
        }

        #[test]
        fn non_zero_origin_outside_bounds() {
            assert_eq!(
                Rect::from_center_size(Vec2::new(100.0, 50.0), Vec2::splat(80.0))
                    .toroidal_wrap(Vec2::new(160.0, 110.0)),
                Vec2::new(80.0, 30.0)
            );
        }
    }
}
