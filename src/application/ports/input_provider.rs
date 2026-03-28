/// Snapshot of all player inputs for a single frame.
///
/// Using a value snapshot (not polling inside the domain) decouples the
/// application logic from specific input libraries and enables easy testing.
#[derive(Debug, Clone, Default)]
pub struct InputSnapshot {
    pub move_left:  bool,
    pub move_right: bool,
    pub pause:      bool,
    pub launch:     bool,
    pub quit:       bool,
}

/// Input port: reads player input for the current frame.
///
/// Concrete implementations are in the infrastructure layer (macroquad, SDL2,
/// test stubs). The application layer depends only on this trait.
pub trait InputProvider: Send + Sync {
    fn snapshot(&self) -> InputSnapshot;
}
