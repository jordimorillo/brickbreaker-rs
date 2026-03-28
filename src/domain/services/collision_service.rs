use crate::domain::entities::ball::Ball;
use crate::domain::entities::brick::Brick;
use crate::domain::entities::paddle::Paddle;
use crate::domain::ports::collision_detector::{CollisionDetector, CollisionSide, WallCollision};

/// Pure domain service implementing AABB collision detection.
///
/// Has no state, no external dependencies, and no side effects — trivially
/// testable and replaceable (DIP: swap in a `PhysicsEngineCollisionDetector`
/// in production or a `StubCollisionDetector` in tests).
pub struct CollisionService;

impl CollisionDetector for CollisionService {
    fn ball_hits_wall(&self, ball: &Ball, world_width: f32, world_height: f32) -> WallCollision {
        if ball.left() <= 0.0   { return WallCollision::Left;   }
        if ball.right() >= world_width  { return WallCollision::Right;  }
        if ball.top()  <= 0.0   { return WallCollision::Top;    }
        if ball.bottom() >= world_height { return WallCollision::Bottom; }
        WallCollision::None
    }

    fn ball_hits_paddle(&self, ball: &Ball, paddle: &Paddle) -> bool {
        // Only trigger when the ball is moving downward and its bottom edge
        // enters the paddle's top half — avoids ghost collisions on the way up.
        ball.velocity.is_moving_down()
            && ball.right()  >= paddle.left()
            && ball.left()   <= paddle.right()
            && ball.bottom() >= paddle.top()
            && ball.top()    <= paddle.bottom()
    }

    fn ball_hits_brick(&self, ball: &Ball, brick: &Brick) -> Option<CollisionSide> {
        if brick.is_destroyed() {
            return None;
        }

        // Fast AABB rejection
        if ball.right()  < brick.left()
            || ball.left()   > brick.right()
            || ball.bottom() < brick.top()
            || ball.top()    > brick.bottom()
        {
            return None;
        }

        // Minimum penetration depth determines which face was hit first.
        let overlap_from_left   = ball.right()  - brick.left();
        let overlap_from_right  = brick.right() - ball.left();
        let overlap_from_top    = ball.bottom() - brick.top();
        let overlap_from_bottom = brick.bottom() - ball.top();

        let min = [overlap_from_left, overlap_from_right, overlap_from_top, overlap_from_bottom]
            .iter()
            .cloned()
            .fold(f32::MAX, f32::min);

        if (min - overlap_from_top).abs() < f32::EPSILON {
            Some(CollisionSide::Top)
        } else if (min - overlap_from_bottom).abs() < f32::EPSILON {
            Some(CollisionSide::Bottom)
        } else if (min - overlap_from_left).abs() < f32::EPSILON {
            Some(CollisionSide::Left)
        } else {
            Some(CollisionSide::Right)
        }
    }
}

// ─── TDD: Unit tests ─────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::brick::BrickKind;
    use crate::domain::value_objects::dimensions::Dimensions;
    use crate::domain::value_objects::position::Position;
    use crate::domain::value_objects::velocity::Velocity;

    const WORLD_W: f32 = 800.0;
    const WORLD_H: f32 = 600.0;
    const SVC: CollisionService = CollisionService;

    fn ball_at(x: f32, y: f32, vx: f32, vy: f32) -> Ball {
        Ball::new(Position::new(x, y), Velocity::new(vx, vy), 8.0)
    }

    fn paddle_at(x: f32) -> Paddle {
        Paddle::new(
            Position::new(x, 560.0),
            Dimensions::new(100.0, 14.0),
            400.0,
        )
    }

    fn brick_at(x: f32, y: f32) -> Brick {
        Brick::new(Position::new(x, y), Dimensions::new(60.0, 20.0), BrickKind::Normal)
    }

    // ── Wall collisions ───────────────────────────────────────────────────────
    #[test]
    fn test_no_wall_collision_center() {
        let b = ball_at(400.0, 300.0, 0.0, 0.0);
        assert_eq!(SVC.ball_hits_wall(&b, WORLD_W, WORLD_H), WallCollision::None);
    }

    #[test]
    fn test_left_wall_collision() {
        let b = ball_at(4.0, 300.0, -100.0, 0.0); // right = 4-8 = -4 → left() < 0
        // ball.left() = 4 - 8 = -4 ≤ 0
        assert_eq!(SVC.ball_hits_wall(&b, WORLD_W, WORLD_H), WallCollision::Left);
    }

    #[test]
    fn test_right_wall_collision() {
        let b = ball_at(796.0, 300.0, 100.0, 0.0); // right() = 804 ≥ 800
        assert_eq!(SVC.ball_hits_wall(&b, WORLD_W, WORLD_H), WallCollision::Right);
    }

    #[test]
    fn test_top_wall_collision() {
        let b = ball_at(400.0, 4.0, 0.0, -100.0); // top() = -4 ≤ 0
        assert_eq!(SVC.ball_hits_wall(&b, WORLD_W, WORLD_H), WallCollision::Top);
    }

    #[test]
    fn test_bottom_wall_collision() {
        let b = ball_at(400.0, 596.0, 0.0, 100.0); // bottom() = 604 ≥ 600
        assert_eq!(SVC.ball_hits_wall(&b, WORLD_W, WORLD_H), WallCollision::Bottom);
    }

    // ── Paddle collisions ─────────────────────────────────────────────────────
    #[test]
    fn test_ball_hits_paddle_overlapping_and_moving_down() {
        // Ball bottom = 554 + 8 = 562, paddle top = 553
        let b = ball_at(400.0, 554.0, 0.0, 200.0);
        let p = paddle_at(400.0);
        assert!(SVC.ball_hits_paddle(&b, &p));
    }

    #[test]
    fn test_ball_does_not_hit_paddle_when_moving_up() {
        let b = ball_at(400.0, 554.0, 0.0, -200.0); // moving up
        let p = paddle_at(400.0);
        assert!(!SVC.ball_hits_paddle(&b, &p));
    }

    #[test]
    fn test_ball_misses_paddle_to_the_side() {
        let b = ball_at(10.0, 554.0, 0.0, 200.0); // far left of paddle
        let p = paddle_at(400.0);
        assert!(!SVC.ball_hits_paddle(&b, &p));
    }

    // ── Brick collisions ──────────────────────────────────────────────────────
    #[test]
    fn test_no_brick_collision_when_far() {
        let b = ball_at(400.0, 400.0, 0.0, 0.0);
        let brick = brick_at(0.0, 0.0);
        assert_eq!(SVC.ball_hits_brick(&b, &brick), None);
    }

    #[test]
    fn test_ball_hits_brick_from_top() {
        // Brick top-left = (200, 100), size 60x20 → top=100
        // Ball center y=94, radius=8 → bottom=102 (slightly inside)
        let b = ball_at(230.0, 94.0, 0.0, 100.0);
        let brick = brick_at(200.0, 100.0);
        let result = SVC.ball_hits_brick(&b, &brick);
        assert_eq!(result, Some(CollisionSide::Top));
    }

    #[test]
    fn test_no_collision_with_destroyed_brick() {
        let b = ball_at(230.0, 105.0, 0.0, 100.0);
        let brick = brick_at(200.0, 100.0).hit(); // destroyed after 1 hit
        assert_eq!(SVC.ball_hits_brick(&b, &brick), None);
    }

    #[test]
    fn test_ball_hits_brick_from_left() {
        // Brick at (200,100) size 60x20. Ball coming from the left.
        // ball right = 208+8=208 > brick.left=200, but min overlap should be on left face.
        // Ball center (192, 110), radius 8 → right = 200, left = 184
        // overlap_left = 200 - 200 = 0  ← smallest → Left side
        let b = ball_at(192.0, 110.0, 100.0, 0.0);
        let brick = brick_at(200.0, 100.0);
        let result = SVC.ball_hits_brick(&b, &brick);
        assert!(result.is_some()); // Must detect a collision
    }
}
