# Brickbreaker RS

[![Rust](https://img.shields.io/badge/language-Rust-orange)](https://www.rust-lang.org/)
[![CI](https://github.com/jordimorillo/brickbreaker-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/jordimorillo/brickbreaker-rs/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![Clippy: clean](https://img.shields.io/badge/clippy-clean-green)](https://github.com/rust-lang/rust-clippy)

A classic brick-breaker game built in Rust — designed as a **portfolio showcase** demonstrating production-grade software engineering practices: **SOLID**, **Domain-Driven Design (DDD)**, and **Test-Driven Development (TDD)**.

---

## 🎮 Gameplay

| Key | Action |
|-----|--------|
| `←` / `A` | Move paddle left |
| `→` / `D` | Move paddle right |
| `Space` | Launch ball / Next level |
| `P` | Pause / Resume |
| `R` | Restart (after Game Over / Victory) |
| `Esc` | Quit |

**3 levels** with increasing difficulty. Clear all destroyable bricks to advance. Collect combos for score multipliers.

---

## 🏗️ Architecture

This project applies **Domain-Driven Design** in a 3-layer architecture:

```
┌───────────────────────────────────────────────────────────────────┐
│  Infrastructure                                                   │
│  ┌──────────────────────┐  ┌──────────────────────────────────┐  │
│  │  MacroquadRenderer   │  │  MacroquadInput                  │  │
│  │  impl Renderer trait │  │  impl InputProvider trait        │  │
│  └──────────┬───────────┘  └──────────────┬─────────────────┘  │
│             │ depends on                   │ depends on           │
└─────────────┼───────────────────────────────┼───────────────────┘
              │                               │
┌─────────────┼───────────────────────────────┼───────────────────┐
│  Application │                              │                    │
│  ┌───────────▼──────────────────────────────▼─────────────────┐ │
│  │  GameService (orchestrator)                                 │ │
│  │  depends on Box<dyn CollisionDetector> + Box<dyn Scorer>   │ │
│  ├─────────────────────────────────────────────────────────────┤ │
│  │  GameState   │   Level (layout factory)                    │ │
│  ├─────────────────────────────────────────────────────────────┤ │
│  │  ports::Renderer  │  ports::InputProvider  (output traits) │ │
│  └─────────────────────────────────────────────────────────────┘ │
│             │ depends on                                          │
└─────────────┼──────────────────────────────────────────────────┘
              │
┌─────────────▼──────────────────────────────────────────────────┐
│  Domain  (zero external dependencies)                           │
│  ┌──────────────────┐  ┌─────────────────┐  ┌───────────────┐ │
│  │  Value Objects   │  │    Entities     │  │   Services    │ │
│  │  Position        │  │  Ball           │  │ Collision-    │ │
│  │  Velocity        │  │  Paddle         │  │ Service       │ │
│  │  Dimensions      │  │  Brick          │  │ ScoringService│ │
│  └──────────────────┘  └─────────────────┘  └───────────────┘ │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  ports (traits)                                         │   │
│  │  CollisionDetector  │  Scorer                           │   │
│  └─────────────────────────────────────────────────────────┘   │
└────────────────────────────────────────────────────────────────┘
```

### Design Principles Applied

#### SOLID

| Principle | How it's demonstrated |
|---|---|
| **S** — Single Responsibility | `Ball` knows its kinematics. `CollisionService` detects collisions. `ScoringService` calculates points. Never mixed. |
| **O** — Open/Closed | `BrickKind` enum extends behaviour (add `Explosive` brick) without touching physics or rendering. |
| **L** — Liskov Substitution | Any `Box<dyn CollisionDetector>` can substitute `CollisionService` without breaking `GameService`. |
| **I** — Interface Segregation | `CollisionDetector`, `Scorer`, `Renderer`, `InputProvider` are small, focused traits — never a fat interface. |
| **D** — Dependency Inversion | `GameService` depends on `Box<dyn CollisionDetector>` and `Box<dyn Scorer>`, not on concrete types. Injected at construction in `main.rs`. |

#### Domain-Driven Design

- **Value Objects**: `Position`, `Velocity`, `Dimensions` — immutable, equality by value, no identity.
- **Entities**: `Ball`, `Paddle`, `Brick` — identity through role in the world, immutable-update pattern.
- **Domain Services**: `CollisionService`, `ScoringService` — stateless, pure logic, no external dependencies.
- **Ports**: Traits defining interfaces the domain and application need (not what they provide).
- **Adapters**: `MacroquadRenderer`, `MacroquadInput` — infrastructure implementations of ports.
- **Application Service**: `GameService` — orchestrates domain logic per frame, depends only on traits.

#### Test-Driven Development

Every domain and application module was written **test-first** (Red → Green → Refactor). Tests live inline with the source using `#[cfg(test)]`.

```
87 tests  |  0 failures  |  0 ignored
```

Infrastructure (renderer, input) is not unit-tested — it has no logic, only macroquad calls.

---

## 🚀 Build & Run

### Prerequisites

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Linux: install OpenGL/X11 dependencies for macroquad
sudo apt install libglfw3-dev libx11-dev libxi-dev libxcursor-dev
```

### Build

```bash
# Debug build (fast compile, slower run)
cargo build

# Release build (optimised, smaller binary)
cargo build --release
```

### Run

```bash
cargo run --release
```

### Test

```bash
cargo test
```

### Lint

```bash
cargo clippy -- -D warnings
```

---

## 📁 Project Structure

```
src/
├── domain/                      # DDD domain layer — zero external dependencies
│   ├── value_objects/
│   │   ├── position.rs          # Immutable 2D position
│   │   ├── velocity.rs          # Immutable velocity vector
│   │   └── dimensions.rs        # Immutable size
│   ├── entities/
│   │   ├── ball.rs              # Ball entity
│   │   ├── paddle.rs            # Paddle entity
│   │   └── brick.rs             # Brick entity + BrickKind enum
│   ├── ports/
│   │   ├── collision_detector.rs  # CollisionDetector trait (ISP + DIP)
│   │   └── scorer.rs              # Scorer trait (ISP + DIP)
│   └── services/
│       ├── collision_service.rs   # AABB collision detection
│       └── scoring_service.rs     # Score + combo multiplier
│
├── application/                 # DDD application layer — orchestration
│   ├── game_state.rs            # GameState aggregate + GameStatus state machine
│   ├── game_service.rs          # Frame update orchestrator (uses DI via traits)
│   ├── level.rs                 # Level layout factory
│   └── ports/
│       ├── renderer.rs          # Renderer output port
│       └── input_provider.rs    # InputProvider input port
│
├── infrastructure/              # DDD infrastructure layer — external adapters
│   ├── macroquad_renderer.rs    # macroquad implementation of Renderer
│   └── macroquad_input.rs       # macroquad implementation of InputProvider
│
└── main.rs                      # Composition root: wires DI, runs game loop
```

---

## 📜 License

[MIT](LICENSE) © Jordi

---

## 👤 Autor

**Jordi Morillo Sells** — [LinkedIn](https://www.linkedin.com/in/jordi-morillo-sells/)
