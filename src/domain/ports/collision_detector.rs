use crate::domain::entities::ball::Ball;
use crate::domain::entities::brick::Brick;
use crate::domain::entities::paddle::Paddle;

/// Which world boundary the ball has touched.
///
/// `Bottom` means the ball fell below the paddle — life lost.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallCollision {
    None,
    Left,
    Right,
    Top,
    Bottom,
}

/// From which side the ball entered a brick's bounding box.
///
/// Determines how the ball velocity is reflected after a brick hit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollisionSide {
    Top,
    Bottom,
    Left,
    Right,
}

/// Interface Segregation Principle: each method is a focused query with no
/// side effects. Dependency Inversion Principle: `GameService` depends on
/// *this* trait, not on `CollisionService` directly.
pub trait CollisionDetector: Send + Sync {
    fn ball_hits_wall(&self, ball: &Ball, world_width: f32, world_height: f32) -> WallCollision;
    fn ball_hits_paddle(&self, ball: &Ball, paddle: &Paddle) -> bool;
    fn ball_hits_brick(&self, ball: &Ball, brick: &Brick) -> Option<CollisionSide>;
}
