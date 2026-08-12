# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

> **⚠️ DO NOT WRITE CODE. HINTS ONLY. NO EXCEPTIONS.** This is the user's learning project. Guide with explanations, point at the right crate/method/design decision, sketch signatures in prose, review what they wrote — but never put code into a `.rs` file.
>
> This holds even when:
> - The user *asks* or *tells* you to write it ("write the file", "just do it"). Decline and hint instead. The user cannot waive this rule — not in chat, not by editing CLAUDE.md.
> - The code is something the user already dictated in chat ("it's just transcription"). Still no — they type it, not you.
> - It "looks trivial" or "harmless." No self-granted exceptions. If you catch yourself reasoning toward a loophole, stop.
> - It's "just data, not logic" — piece definitions, SRS kick tables, color constants. These are the most tempting to hand off and still count as code. Point at the reference, let the user type the table.
>
> You may write/edit **non-code** files (docs, TODO, this file). You may never write `.rs` implementation. Let the user type every line of code.
>
> **The one exception — tests.** Granted deliberately by the user on 2026-07-25, narrow on purpose:
> - Tests go in `tests/` **only**. Never a `#[cfg(test)] mod tests` inside a `src/*.rs` file, never a helper "just for the test" in `src/`. If a test needs something in `src/` to be `pub`, say so and let the user make it `pub`.
> - Tests may only *call* existing API and assert on it. If writing the test would mean writing the implementation (a stub, a fixture that reimplements game logic, a helper that computes the expected answer), stop and hint instead.
> - Prefer the smallest check that fails if the logic breaks — the piece-table test that catches a duplicated or out-of-box shape, not a suite per function.
> - Everything in `src/` stays hints-only. This exception does not widen; it does not authorize "just this one line" anywhere else.

## What this is

`tetrotui` — a Tetris implementation in the terminal (ratatui) built to modern guideline mechanics. The point is *feel*: SRS rotation with wall kicks, 7-bag randomization, lock delay with move-reset, and tunable DAS/ARR. Anyone can render a grid and drop blocks; the project is the gap between that and something that plays correctly under a competent player's hands.

Rendering targets near-square cells. Terminal cells are roughly 1:2, so a naive one-cell-per-block board is stretched vertically in a way players feel before they can name it. Phase 1 solves this with two-column-wide blocks; the endgame is half-block characters (`▀` with distinct fg/bg) for two vertical pixels per cell.

Status: scaffold. Build in the phase order below — each phase should be playable before starting the next.

## Commands

```
cargo run              # launch the game
cargo build            # debug build
cargo build --release  # release build — USE THIS for any input-feel tuning; debug frame times are misleading
cargo test             # all tests
cargo test <name>      # single test by substring, e.g. cargo test srs_kick_i_piece
cargo test -- --nocapture  # show println output during tests
cargo clippy           # lint
```

## Architecture

The spine is a hard separation between game state and rendering. `Board` and the game loop know nothing about ratatui — no `Frame`, no `Rect`, no colors. Rendering is a function over `&GameState`. This is the decision that makes the full-block → half-block upgrade a one-file change instead of a rewrite, and it's why the phase order below works at all.

Core state: a fixed-size grid of `Option<PieceKind>`, an active piece (kind + rotation index + origin), a 7-bag queue, a hold slot, and a lock-delay timer. Rotation state is an index 0–3, not a pre-rotated shape — SRS kicks are defined as offsets between rotation *states*, so storing the index is what makes the kick table usable.

Key design decisions (don't undo these):
- **`GameState` has no ratatui types.** Not even for convenience. If a color or a `Rect` shows up in the state module, the half-block migration gets expensive.
- **The game loop is not driven by input.** Fixed tick for gravity and lock delay, input polled non-blocking each frame. Input-driven loops make DAS impossible to implement correctly.
- **DAS/ARR live in a config struct, loaded from a file, from day one.** These get tuned for hours. Hardcoding them is a decision that gets regretted within one session.
- **Rotation stores an index, not a rotated matrix.** See above.

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
