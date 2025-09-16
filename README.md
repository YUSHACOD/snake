### Snake (Rust, terminal)

A simple, fast, and dependency-light terminal Snake game written in Rust using `crossterm` for rendering and `nanorand` for food placement. It uses an alternate screen, raw mode input, and a double-buffered renderer for smooth updates.

## Features
- **Terminal UI** with bordered play area, live score, and input status line
- **Smooth rendering** via double buffering; only changed cells are redrawn
- **Wrap-around movement** at edges (no walls)
- **Responsive controls** with pause and quit
- **Cross‑platform** (Windows, Linux, macOS) with `crossterm`

## Controls
- **Enter**: Start
- **Space**: Pause/Resume
- **h / Left Arrow**: Move left
- **j / Down Arrow**: Move down
- **k / Up Arrow**: Move up
- **l / Right Arrow**: Move right
- **q**: Quit

## Requirements
- Rust toolchain (1.70+ recommended). Install via `https://rustup.rs`.

## Build
```
cargo build --release
```

## Run
The game takes a single required argument: the frame delay in milliseconds (recommended 80–750 ms).

Examples:
```
cargo run -- 120
cargo run --release -- 200
```

If run without a valid delay or with `help`, usage and controls are printed.

## How it works (high level)
- `main.rs` initializes the terminal, computes the drawable area, spawns the renderer/game loop, and starts a non‑blocking input listener on a channel.
- `event_capturer.rs` reads key events in raw mode and converts them into a compact `Input` enum.
- `game.rs` maintains `GameState` (snake body, direction, score, food), generates frames, detects collisions, and updates the score. It also manages pause/start and end‑of‑game messaging.
- `snake.rs` holds movement rules, direction changes (preventing 180° turns), food placement with `WyRand`, and frame composition.
- `display.rs` provides a double‑buffered matrix and draws only changed cells for efficiency.
- `game_display.rs` renders the outer frame, score box, message box, food, score, and status text.
- `window.rs` sets up the alternate screen, hides/shows cursor, toggles raw mode, and draws generic boxes.

## CLI
```
Usage: cargo run -- <delay_ms>

Arguments:
  <delay_ms>   Frame delay in milliseconds (suggested 80–750). Defaults to 100 if parsing fails.
```

## Tips
- Make the terminal window larger for more play space; the game adapts to current terminal size at launch.
- Use a monospaced font and a terminal that supports ANSI escape sequences.
- On Windows, use a recent Terminal/PowerShell for best results.

## Troubleshooting
- **Weird borders or misalignment**: Ensure your terminal uses a monospaced font and supports box‑drawing characters; try increasing window size.
- **Terminal stuck after crash**: The program attempts to clean up (leave raw mode, show cursor). If something goes wrong, try pressing `Enter`, then `reset` (Unix) or close/reopen the terminal (Windows).
- **No input registered**: Make sure the terminal is focused and raw mode isn’t blocked by another process.

## Project structure
```
src/
  display.rs        # Buffer and efficient drawing
  event_capturer.rs # Input handling (raw mode)
  game.rs           # Game loop, state, rendering orchestration
  game_display.rs   # UI boxes, score, messages, food
  snake.rs          # Movement, collision, food logic
  window.rs         # Terminal setup/teardown and box drawing
```

## Dependencies
- `crossterm = "0.27"`
- `nanorand = "0.7"`

