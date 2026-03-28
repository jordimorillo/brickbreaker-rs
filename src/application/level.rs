use crate::domain::entities::brick::{Brick, BrickKind};
use crate::domain::value_objects::dimensions::Dimensions;
use crate::domain::value_objects::position::Position;

/// Maximum number of levels before the player wins.
pub const MAX_LEVEL: u32 = 3;

/// A level definition containing the initial brick layout.
pub struct Level {
    pub bricks: Vec<Brick>,
}

/// Factory function — creates the brick layout for a given level number.
///
/// Open/Closed Principle: adding level 4 means adding a new arm to the
/// `match` without touching existing level logic.
pub fn create_level(level_num: u32, world_width: f32) -> Level {
    // 10 columns with 2px gaps on each side and between bricks
    let padding     = 2.0;
    let cols        = 10usize;
    let brick_w     = (world_width - padding * (cols as f32 + 1.0)) / cols as f32;
    let brick_h     = 20.0;
    let start_y     = 60.0;

    let bricks = match level_num {
        1 => build_level_1(cols, brick_w, brick_h, padding, start_y),
        2 => build_level_2(cols, brick_w, brick_h, padding, start_y),
        3 => build_level_3(cols, brick_w, brick_h, padding, start_y),
        n => build_level_endless(n, cols, brick_w, brick_h, padding, start_y),
    };

    Level { bricks }
}

// ── Level builders ────────────────────────────────────────────────────────────

/// Level 1 — 4 rows of normal bricks. Classic Breakout layout.
fn build_level_1(cols: usize, bw: f32, bh: f32, pad: f32, sy: f32) -> Vec<Brick> {
    grid(cols, 4, bw, bh, pad, sy, |_row, _col| BrickKind::Normal)
}

/// Level 2 — top row indestructible, next 2 rows tough, bottom normal.
fn build_level_2(cols: usize, bw: f32, bh: f32, pad: f32, sy: f32) -> Vec<Brick> {
    grid(cols, 5, bw, bh, pad, sy, |row, _col| match row {
        0 => BrickKind::Indestructible,
        1 | 2 => BrickKind::Tough,
        _ => BrickKind::Normal,
    })
}

/// Level 3 — checkerboard of Tough and Normal bricks.
fn build_level_3(cols: usize, bw: f32, bh: f32, pad: f32, sy: f32) -> Vec<Brick> {
    grid(cols, 6, bw, bh, pad, sy, |row, col| {
        if (row + col) % 2 == 0 { BrickKind::Tough } else { BrickKind::Normal }
    })
}

/// Endless mode — grows the grid and adds more indestructible/tough rows.
fn build_level_endless(level: u32, cols: usize, bw: f32, bh: f32, pad: f32, sy: f32) -> Vec<Brick> {
    let rows = ((level + 2) as usize).min(10);
    grid(cols, rows, bw, bh, pad, sy, |row, _col| match row {
        0 => BrickKind::Indestructible,
        r if r < 3 => BrickKind::Tough,
        _ => BrickKind::Normal,
    })
}

// ── Grid helper ───────────────────────────────────────────────────────────────

fn grid(
    cols: usize,
    rows: usize,
    brick_w: f32,
    brick_h: f32,
    padding: f32,
    start_y: f32,
    kind_fn: impl Fn(usize, usize) -> BrickKind,
) -> Vec<Brick> {
    let mut bricks = Vec::with_capacity(cols * rows);
    for row in 0..rows {
        for col in 0..cols {
            let x = padding + col as f32 * (brick_w + padding);
            let y = start_y + row as f32 * (brick_h + padding);
            let kind = kind_fn(row, col);
            bricks.push(Brick::new(
                Position::new(x, y),
                Dimensions::new(brick_w, brick_h),
                kind,
            ));
        }
    }
    bricks
}

// ─── TDD: Unit tests ─────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    const WORLD_W: f32 = 800.0;

    #[test]
    fn test_level_1_has_40_bricks() {
        let lvl = create_level(1, WORLD_W);
        assert_eq!(lvl.bricks.len(), 40); // 10 cols × 4 rows
    }

    #[test]
    fn test_level_1_all_normal_bricks() {
        let lvl = create_level(1, WORLD_W);
        assert!(lvl.bricks.iter().all(|b| b.kind == BrickKind::Normal));
    }

    #[test]
    fn test_level_2_has_50_bricks() {
        let lvl = create_level(2, WORLD_W);
        assert_eq!(lvl.bricks.len(), 50); // 10 cols × 5 rows
    }

    #[test]
    fn test_level_2_first_row_is_indestructible() {
        let lvl = create_level(2, WORLD_W);
        let first_row: Vec<_> = lvl.bricks.iter().take(10).collect();
        assert!(first_row.iter().all(|b| b.kind == BrickKind::Indestructible));
    }

    #[test]
    fn test_level_3_has_60_bricks() {
        let lvl = create_level(3, WORLD_W);
        assert_eq!(lvl.bricks.len(), 60); // 10 cols × 6 rows
    }

    #[test]
    fn test_bricks_do_not_overlap_left_wall() {
        let lvl = create_level(1, WORLD_W);
        assert!(lvl.bricks.iter().all(|b| b.left() >= 0.0));
    }

    #[test]
    fn test_bricks_do_not_overlap_right_wall() {
        let lvl = create_level(1, WORLD_W);
        assert!(lvl.bricks.iter().all(|b| b.right() <= WORLD_W + 0.01));
    }

    #[test]
    fn test_endless_level_grows_rows() {
        let lvl4 = create_level(4, WORLD_W).bricks.len();
        let lvl5 = create_level(5, WORLD_W).bricks.len();
        assert!(lvl5 >= lvl4);
    }
}
