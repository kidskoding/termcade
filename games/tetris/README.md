# tetris

A sleek and intuitive TUI for Tetris.

Built to modern guideline mechanics. The point is *feel*: SRS rotation with wall kicks, 7-bag randomization, lock delay with move-reset, and tunable DAS/ARR. Anyone can render a grid and drop blocks; the project is the gap between that and something that plays correctly under a competent player's hands.

Status: scaffold. `TODO.md` holds the phase order — each phase ends playable before the next begins.

## Running

Standalone:

```
cargo run              # launch the game
cargo build --release  # USE THIS for input-feel tuning; debug frame times are misleading
cargo test             # all tests
cargo clippy           # lint
```

Or through the arcade: `cargo run` from the workspace root and pick Tetris. The game is a library `termcade` links and runs in-process — `tetris::run` takes a `&mut Terminal<B>` and stays generic over the backend. `main.rs` is a thin standalone wrapper.

## Rendering

Terminal cells are roughly 1:2, so a naive one-cell-per-block board is stretched vertically in a way players feel before they can name it. Phase 1 solves this with two-column-wide blocks; the endgame is half-block characters (`▀` with distinct fg/bg) for two vertical pixels per cell.

## Architecture

The spine is a hard separation between game state and rendering. `Board` and the game loop know nothing about ratatui — no `Frame`, no `Rect`, no colors. Rendering is a function over `&GameState`. This is what makes the full-block → half-block upgrade a one-file change instead of a rewrite, and it's why the phase order works at all.

Core state: a fixed-size grid of `Option<PieceKind>`, an active piece (kind + rotation index + origin), a 7-bag queue, a hold slot, and a lock-delay timer. Rotation state is an index 0–3, not a pre-rotated shape — SRS kicks are defined as offsets between rotation *states*, so storing the index is what makes the kick table usable.

Design decisions that shouldn't be undone:

- **`GameState` has no ratatui types.** Not even for convenience. A color or a `Rect` in the state module makes the half-block migration expensive.
- **The game loop is not driven by input.** Fixed tick for gravity and lock delay, input polled non-blocking each frame. Input-driven loops make DAS impossible to implement correctly.
- **DAS/ARR live in a config struct loaded from a file, from day one.** These get tuned for hours; hardcoding them is regretted within one session.
- **Rotation stores an index, not a rotated matrix.**
- **`run` stays generic over the backend.** A concrete `Stdout` in the loop is what would have to be undone to serve the arcade over SSH.

## Feel specifics (easy to get wrong)

- **Line clear needs a beat.** 150–200ms of flash or collapse before rows vanish. Without it, clears feel like they didn't register — the single most common omission in hobby implementations.
- **Lock delay needs a cap.** Move-reset without a reset limit means infinite stalling. Standard is 15 resets before forced lock.
- **Tetromino colors are convention, not choice.** Cyan I, yellow O, purple T, green S, red Z, blue J, orange L. Players pattern-match on these. Freedom lives in the chrome — frame, HUD, background — and the discipline there is restraint: the pieces should be the only saturated thing on screen.
- **Layout stays tight.** The board at two columns per cell is 20 columns wide. The temptation is to pad side panels to fill an 80-column terminal; don't. Dead space around a dense composition reads arcade. Sprawl reads dashboard.
- **Resize is a feature.** Recompute layout and redraw on resize; below minimum dimensions, render a legible "needs 24×30" message rather than garbage. Most terminal games skip this, and it's what separates software from a project.
- **Spawn orientation and position are specified.** Pieces spawn flat, in the two rows above the visible field. Getting this wrong makes the opening of every game feel off.
