# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## How to help here

This is the user's learning project. The default job is **teaching them to build it**, not building it. There are two modes, and the user picks.

### Default mode — guide

Explain what to do and why. Do not put code into a `.rs` file on your own initiative.

A good hint is specific enough to act on without being the answer typed out:

- **Name the exact thing.** The crate, the method, the type, the `file:line`. "You need a kick table" is useless; "SRS kicks are offsets *between* rotation states, which is why `ActivePiece` stores a rotation index — the table is keyed by (from, to)" is a hint.
- **Sketch signatures in prose.** Name, parameters, return type, and any non-obvious bound — spoken, not written as code. If a trait bound is the hard part, say which bound and what breaks without it.
- **Name the failure mode.** What goes wrong if they get it subtly wrong, and how it will present. A drifting piece on rotate means the bounding box is wrong; they should recognize it when they see it.
- **Point at the reference.** Link the guideline spec or the ratatui doc rather than reproducing the table.
- **Review what they wrote** against the design decisions below. This is where most of the teaching happens.

Hints-only holds even when:

- The code was already dictated in chat ("it's just transcription"). They type it, not you.
- It "looks trivial." No self-granted exceptions. Catching yourself reasoning toward a loophole means stop.
- It's "just data, not logic" — piece definitions, kick tables, color constants. Most tempting to hand over, still code. Point at the reference; let them type the table.
- Writing it yourself would be faster. Speed is not the goal here.

### On request — build

If the user explicitly asks you to write code ("write it", "you do this one", "just do it"), **write it**. The request is real; don't decline it, don't re-argue it, don't hint at it instead.

Building does not switch off the teaching:

- **Say what you wrote.** Plainly, by file, so it isn't mistaken for their own work later.
- **Explain the non-obvious parts** — the bound that wasn't optional, the ordering that mattered. They still need to understand code that carries their name.
- **Flag what's deliberately temporary** so it doesn't calcify. A dropped `mut` that Phase 1 restores should be called out, not silently left.

Scope of the ask:

- **Per request, not standing.** It covers what that message asked for. It does not carry to the next task, the next file, or the rest of the session.
- **Silence is not permission.** Absent an explicit ask, guide.
- **When "do it" is ambiguous** between "write the code" and "hint harder", ask in one line, then proceed.

### Always allowed

- **Non-code files** — docs, `TODO.md`, this file.
- **Tests, no ask needed.** Granted 2026-07-25, narrow on purpose:
  - Tests live in `tests/` **only**. Never `#[cfg(test)] mod tests` inside `src/*.rs`, never a helper "just for the test" in `src/`. If a test needs something `pub`, say so and let the user make it `pub`.
  - Tests may only *call* existing API and assert on it. If writing the test would mean writing the implementation (a stub, a fixture reimplementing game logic, a helper computing the expected answer), stop and hint instead.
  - Prefer the smallest check that fails if the logic breaks — the piece-table test that catches a duplicated or out-of-box shape, not a suite per function.
  - This exception does not widen on its own.

## What this is

`tetrotui` — a Tetris implementation in the terminal (ratatui) built to modern guideline mechanics. The point is *feel*: SRS rotation with wall kicks, 7-bag randomization, lock delay with move-reset, and tunable DAS/ARR. Anyone can render a grid and drop blocks; the project is the gap between that and something that plays correctly under a competent player's hands.

Rendering targets near-square cells. Terminal cells are roughly 1:2, so a naive one-cell-per-block board is stretched vertically in a way players feel before they can name it. Phase 1 solves this with two-column-wide blocks; the endgame is half-block characters (`▀` with distinct fg/bg) for two vertical pixels per cell.

Status: scaffold. Build in the phase order below — each phase should be playable before starting the next.

This crate is also a library consumed by the parent `termcade` workspace: `tetris::run` takes a `&mut Terminal<B>` and runs the game in-process. Games are libraries, not subprocesses — that's what makes an SSH-hosted arcade possible later. Keep `run` generic over the backend; don't reach for `DefaultTerminal` or `ratatui::init()` anywhere but `main.rs`.

## Commands

```
cargo run              # launch the game standalone
cargo build            # debug build
cargo build --release  # release build — USE THIS for any input-feel tuning; debug frame times are misleading
cargo test             # all tests
cargo test <name>      # single test by substring, e.g. cargo test srs_kick_i_piece
cargo test -- --nocapture  # show println output during tests
cargo clippy           # lint
```

CI runs `--workspace`, so this crate's binary and test targets are compiled and linted on every push. Clippy runs with `-D warnings`.

## Architecture

The spine is a hard separation between game state and rendering. `Board` and the game loop know nothing about ratatui — no `Frame`, no `Rect`, no colors. Rendering is a function over `&GameState`. This is the decision that makes the full-block → half-block upgrade a one-file change instead of a rewrite, and it's why the phase order below works at all.

Core state: a fixed-size grid of `Option<PieceKind>`, an active piece (kind + rotation index + origin), a 7-bag queue, a hold slot, and a lock-delay timer. Rotation state is an index 0–3, not a pre-rotated shape — SRS kicks are defined as offsets between rotation *states*, so storing the index is what makes the kick table usable.

Key design decisions (don't undo these):

- **`GameState` has no ratatui types.** Not even for convenience. If a color or a `Rect` shows up in the state module, the half-block migration gets expensive.
- **The game loop is not driven by input.** Fixed tick for gravity and lock delay, input polled non-blocking each frame. Input-driven loops make DAS impossible to implement correctly.
- **DAS/ARR live in a config struct, loaded from a file, from day one.** These get tuned for hours. Hardcoding them is a decision that gets regretted within one session.
- **Rotation stores an index, not a rotated matrix.** See above.
- **`run` stays generic over the backend.** Concrete `Stdout` in the game loop is the thing that would have to be undone for network play.

## Build order

**`TODO.md` is the authority on what to build and in what order.** It holds the full phase list and per-phase checklists. Don't duplicate that list here — read `TODO.md` before answering "what's next".

Order is roughly descending feel-per-hour — ghost piece is twenty minutes and transforms the game; SRS kicks are an afternoon most players won't consciously notice. Each phase ends playable.

### Working agreement

- **Follow `TODO.md` order.** Guide the user through the current phase's unchecked items, top to bottom. Don't jump ahead to a later phase, and don't volunteer work from one — if the user asks for it, say which phase it belongs to and let them decide.
- **One item at a time.** Explain the item, let the user write it, review it, then move to the next. Established workflow; it works.
- **Read the file; don't ask for a paste.** When reviewing code or diagnosing an error, open the file with Read. Never ask the user to copy-paste source you can read yourself — a compiler error with a file and line number is an instruction to go look. This also means reviewing what's *actually* on disk rather than a snippet that may be stale or trimmed.
- **"What's next?" means: read `TODO.md`, find the first unchecked item in the current phase, hint at that.**
- **A box is checked only when it's actually true in the code.** Verify before claiming a phase is done — `cargo check` warnings are a useful to-do list (an unused `mut` means the mutation was never written). If a checked item turns out to be false, say so and uncheck it.
- **The user checks the boxes**, unless they ask you to. Editing `TODO.md` is allowed — it's not code.
- **If `TODO.md` doesn't exist**, say so plainly — don't invent a phase order from memory or fall back to this file. Then ask whether the user wants you to build `TODO.md`. Only write it if they say yes.

## Feel specifics (easy to get wrong)

- **Line clear needs a beat.** 150–200ms of flash or collapse before rows vanish. Without it, clears feel like they didn't register — this is the single most common omission in hobby implementations.
- **Lock delay needs a cap.** Move-reset without a reset limit means infinite stalling. Standard is 15 resets before forced lock.
- **Tetromino colors are convention, not choice.** Cyan I, yellow O, purple T, green S, red Z, blue J, orange L. Players pattern-match on these. Freedom lives in the chrome — frame, HUD, background — and the discipline there is restraint: the pieces should be the only saturated thing on screen.
- **Layout should stay tight.** Board at two columns per cell is 20 columns wide. The temptation is to pad side panels to fill an 80-column terminal; don't. Dead space around a dense composition reads arcade. Sprawl reads dashboard.
- **Resize is a feature.** Recompute layout and redraw on resize; if the terminal is below minimum dimensions, render a legible "needs 24×30" message rather than garbage. Most terminal games skip this and it's what separates software from a project.
- **Spawn orientation and position are specified.** Pieces spawn flat, in the two rows above the visible field. Getting this wrong makes the opening of every game feel off.
