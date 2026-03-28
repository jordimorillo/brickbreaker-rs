/// 2D velocity vector.
///
/// Immutable value object. Reflection operations model ball physics in a
/// pure, side-effect-free way — ideal for TDD.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
}

impl Velocity {
    pub fn new(vx: f32, vy: f32) -> Self {
        Self { vx, vy }
    }

    /// Reflects the horizontal component (bounce off left/right walls).
    pub fn reflect_horizontal(&self) -> Self {
        Self { vx: -self.vx, vy: self.vy }
    }

    /// Reflects the vertical component (bounce off top wall or bricks).
    pub fn reflect_vertical(&self) -> Self {
        Self { vx: self.vx, vy: -self.vy }
    }

    /// Scalar speed (magnitude of the vector).
    pub fn speed(&self) -> f32 {
        (self.vx * self.vx + self.vy * self.vy).sqrt()
    }

    /// Scales both components by `factor`, used to enforce minimum ball speed.
    pub fn scale(&self, factor: f32) -> Self {
        Self { vx: self.vx * factor, vy: self.vy * factor }
    }

    /// Returns true if the ball is moving downward (positive Y axis = down).
    pub fn is_moving_down(&self) -> bool {
        self.vy > 0.0
    }
}

// ─── TDD: Unit tests ─────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflect_horizontal_negates_vx() {
        let v = Velocity::new(3.0, -4.0);
        let r = v.reflect_horizontal();
        assert_eq!(r.vx, -3.0);
        assert_eq!(r.vy, -4.0);
    }

    #[test]
    fn test_reflect_vertical_negates_vy() {
        let v = Velocity::new(3.0, -4.0);
        let r = v.reflect_vertical();
        assert_eq!(r.vx, 3.0);
        assert_eq!(r.vy, 4.0);
    }

    #[test]
    fn test_reflect_does_not_mutate_original() {
        let v = Velocity::new(1.0, 2.0);
        let _ = v.reflect_horizontal();
        assert_eq!(v.vx, 1.0);
    }

    #[test]
    fn test_speed_computes_magnitude() {
        let v = Velocity::new(3.0, 4.0);
        assert_eq!(v.speed(), 5.0);
    }

    #[test]
    fn test_speed_of_zero_vector() {
        let v = Velocity::new(0.0, 0.0);
        assert_eq!(v.speed(), 0.0);
    }

    #[test]
    fn test_is_moving_down_when_vy_positive() {
        let v = Velocity::new(0.0, 1.0);
        assert!(v.is_moving_down());
    }

    #[test]
    fn test_is_not_moving_down_when_vy_negative() {
        let v = Velocity::new(0.0, -1.0);
        assert!(!v.is_moving_down());
    }

    #[test]
    fn test_double_reflect_horizontal_is_identity() {
        let v = Velocity::new(5.0, -3.0);
        assert_eq!(v.reflect_horizontal().reflect_horizontal(), v);
    }

    #[test]
    fn test_scale_preserves_direction() {
        let v = Velocity::new(3.0, 4.0); // speed = 5
        let scaled = v.scale(2.0);
        assert_eq!(scaled.speed(), 10.0);
    }
}
