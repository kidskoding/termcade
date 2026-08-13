# Design — animated cabinet art + launch transition

Date: 2026-08-13
Status: approved, ready to plan

## Context

`termcade` is a terminal arcade: a menu shell (`src/`) plus games as workspace-member
libraries (`games/`). Today the shell has a logo, a game list, a detail panel with static
ASCII art, a blinking prompt, a footer, and an error dialog. One game is registered
(tetris) and it is mid-Phase-1 in its own `TODO.md` — not yet playable.

The goal for the project is an arcade *cabinet*, not a launcher. The long-term hook is
presence: an SSH-served arcade where other players are visibly there. That is deliberately
deferred. The near-term goal is a local binary that feels like a cabinet, with games built
one at a time to real depth rather than many games built shallow.

## Decisions locked in this session

These constrain the work but are not themselves part of this spec's scope.

- **Local binary first, SSH later.** `termcade` runs locally and stores state under
  `~/.local/share/termcade/`. A future `termcade serve` reuses the same cabinet with one
  session per SSH connection. Shipping a binary people run themselves is the expected
  distribution model and does not conflict with the hosted arcade.
- **No shared `Game` trait yet.** Extracting a common game loop from a single
  implementation is speculative. Tetris needs a fixed tick with DAS/ARR; a future
  turn-based game will not. The trait gets extracted when a second game exists and the
  shared surface is observable. Until then games keep their current
  `run<B: Backend>(&mut Terminal<B>) -> Result<()>` signature.
- **Depth over breadth.** One game reaches "incredibly strong" before the next one starts.
- **Deferred menu features**, revisited once more than one cabinet is registered:
  high-score persistence and the `TOP  score  AAA` line, attract mode, wide-terminal layout
  capping.
- **Rejected outright**: insert-coin/credits, CRT scanline shading, per-game config screens.

## Scope

Two features, in this order:

1. Animated cabinet art in the menu's detail panel.
2. A shutter wipe transition when launching and returning from a game.

Both are parent-shell code (`src/`). The teach-mode rules in `games/CLAUDE.md` cover
`games/` only and do not apply here.

## Feature A — animated cabinet art

### Current state

`src/game.rs` defines `art: &'static [(&'static str, Color)]` — a flat list of colored
rows. `src/render.rs::playfield_lines` wraps it in a box-drawing border, sizing the border
from the widest row.

### Design

`art` becomes a list of animation cels:

```rust
// src/game.rs
pub type Cel = &'static [(&'static str, Color)];

pub struct Game {
    pub name: &'static str,
    pub blurb: &'static str,
    pub id: &'static str,
    pub hint: &'static str,
    pub art: &'static [Cel],
}
```

The displayed cel is derived from a clock rather than stored as animation state:

```
index = art_started.elapsed().as_millis() / CEL_MS % art.len()
```

`CEL_MS` is a constant, initially 200. `Menu` gains one field, `art_started: Instant`,
reset inside `select()` so that changing the highlighted game restarts its animation from
cel 0 rather than joining it mid-cycle.

Nothing about the event loop changes. `Menu::run` already redraws on every 50 ms poll
timeout, which is 20 fps — five redraws per cel.

`playfield_lines` takes a `Cel` instead of a `&Game` and is otherwise unchanged.

### Tetris cel sequence

A short loop that reads as tetris in roughly two seconds: a piece falls three or four
cels, locks into place, a completed row flashes, the row clears. The sequence must return
to its starting state so the loop is seamless.

### Failure mode and its test

Cels of differing dimensions make the panel jitter every 200 ms, and because
`playfield_lines` derives the border width from the widest row of the *current* cel, a
short cel visibly shrinks the box. Every cel of a given cabinet must have the same row
count and the same row width, space-padded.

One test in `tests/`, iterating `GAMES`: for each cabinet, assert all cels have equal row
count, and that every row across every cel has equal display width. This fails the moment
a ragged cel is added.

## Feature B — launch transition

### Current state

`Menu::launch_selected` calls `games::launch`, stores any error, then calls
`terminal.clear()`. The visual result is a hard cut in both directions.

### Design

A shutter wipe: bars close over the menu from the top and bottom edges until they meet,
the game runs, and the bars retract when it returns.

Split along the existing ownership line in the codebase:

- `render::bars(frame, area, covered_rows)` draws the overlay. It is a draw function taking
  `&mut Frame`, so it belongs in `render.rs` with the others.
- `Menu::wipe(terminal, direction)` owns the loop, the terminal, and the sleep, so it
  belongs in `menu.rs` next to `run`. `direction` is a two-variant enum, closing or opening.

No new files.

`launch_selected` becomes: wipe closing → `games::launch` → wipe opening →
`terminal.clear()`.

Timing: approximately 150 ms closing and 120 ms opening, both constants, tuned by feel.

### Edge cases

- **Launch failure.** The bars close, `launch` returns its error immediately, the bars
  retract, and the existing error dialog draws over the menu. No special handling needed;
  the error is stored before the opening wipe as it is today.
- **Terminal below minimum size.** `render::draw` already early-returns with a "needs
  46×18" message. `wipe` skips entirely in that case — there is no menu underneath to wipe
  over.

## Build order

1. `Cel` type; `art` becomes `&'static [Cel]`; `art_started` field on `Menu` reset in
   `select()`; cel index computed in `draw_detail`.
2. The tetris cel sequence.
3. The cel-uniformity test.
4. `render::bars` and `Menu::wipe`, wired into `launch_selected`.

Steps 1 and 2 leave the menu working at every point — step 1 with the existing art as a
single cel is a valid intermediate state.

## Verification

- `cargo test --workspace` — includes the new cel-uniformity test.
- `cargo clippy --workspace -- -D warnings` and `cargo fmt --all --check`, matching CI.
- Manual: run `termcade`, confirm the tetris cabinet animates and loops without the panel
  jittering, then press enter and confirm the wipe closes and retracts.

## Out of scope

High-score persistence, attract mode, wide-terminal layout capping, the shared `Game`
trait, SSH serving, and any change to tetris gameplay or its `TODO.md` phases.
