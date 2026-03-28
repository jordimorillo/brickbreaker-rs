/// 2D size/dimensions.
///
/// Value object guaranteeing positive dimensions at construction time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dimensions {
    pub width: f32,
    pub height: f32,
}

impl Dimensions {
    /// Panics in debug builds if width or height are not positive.
    pub fn new(width: f32, height: f32) -> Self {
        debug_assert!(width > 0.0, "Dimensions width must be positive, got {width}");
        debug_assert!(height > 0.0, "Dimensions height must be positive, got {height}");
        Self { width, height }
    }

    pub fn half_width(&self) -> f32 {
        self.width / 2.0
    }

    pub fn half_height(&self) -> f32 {
        self.height / 2.0
    }
}

// ─── TDD: Unit tests ─────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimensions_stores_size() {
        let d = Dimensions::new(100.0, 20.0);
        assert_eq!(d.width, 100.0);
        assert_eq!(d.height, 20.0);
    }

    #[test]
    fn test_half_width() {
        let d = Dimensions::new(80.0, 14.0);
        assert_eq!(d.half_width(), 40.0);
    }

    #[test]
    fn test_half_height() {
        let d = Dimensions::new(80.0, 14.0);
        assert_eq!(d.half_height(), 7.0);
    }

    #[test]
    fn test_equality_by_value() {
        let a = Dimensions::new(10.0, 5.0);
        let b = Dimensions::new(10.0, 5.0);
        assert_eq!(a, b);
    }
}
