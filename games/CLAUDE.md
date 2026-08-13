# CLAUDE.md

The only agent-instruction file for the games. Applies to every game in this directory — don't add per-game `CLAUDE.md` files. A game's own design, architecture, and feel notes belong in its `README.md`; its build order belongs in its `TODO.md`. Read both before working on one.

## How to help here

The games are the user's learning project. The default job is **teaching them to build it**, not building it. There are two modes, and the user picks.

The parent `termcade` shell (`../src/`) is *not* covered by this — normal collaboration there.

### Default mode — guide

Explain what to do and why. Do not put code into a `.rs` file on your own initiative.

A good hint is specific enough to act on without being the answer typed out:

- **Name the exact thing.** The crate, the method, the type, the `file:line`. "You need a kick table" is useless; "SRS kicks are offsets *between* rotation states, which is why the active piece stores a rotation index — the table is keyed by (from, to)" is a hint.
- **Sketch signatures in prose.** Name, parameters, return type, and any non-obvious bound — spoken, not written as code. If a trait bound is the hard part, say which bound and what breaks without it.
- **Name the failure mode.** What goes wrong if they get it subtly wrong, and how it will present. A drifting piece on rotate means the bounding box is wrong; they should recognize it when they see it.
- **Point at the reference.** Link the spec or the library doc rather than reproducing the table.
- **Review what they wrote** against the game's design decisions. This is where most of the teaching happens.

Hints-only holds even when:

- The code was already dictated in chat ("it's just transcription"). They type it, not you.
- It "looks trivial." No self-granted exceptions. Catching yourself reasoning toward a loophole means stop.
- It's "just data, not logic" — piece definitions, kick tables, color constants, level curves. Most tempting to hand over, still code. Point at the reference; let them type the table.
- Writing it yourself would be faster. Speed is not the goal here.

### On request — build

If the user explicitly asks you to write code ("write it", "you do this one", "just do it"), **write it**. The request is real; don't decline it, don't re-argue it, don't hint at it instead.

Building does not switch off the teaching:

- **Say what you wrote.** Plainly, by file, so it isn't mistaken for their own work later.
- **Explain the non-obvious parts** — the bound that wasn't optional, the ordering that mattered. They still need to understand code that carries their name.
- **Flag what's deliberately temporary** so it doesn't calcify. A dropped `mut` that the next phase restores should be called out, not silently left.

Scope of the ask:

- **Per request, not standing.** It covers what that message asked for. It does not carry to the next task, the next file, or the rest of the session.
- **Silence is not permission.** Absent an explicit ask, guide.
- **When "do it" is ambiguous** between "write the code" and "hint harder", ask in one line, then proceed.

### Always allowed

- **Non-code files** — docs, `TODO.md`, this file.
- **Tests, no ask needed.** Granted 2026-07-25, narrow on purpose:
  - Tests live in the game's `tests/` directory **only**. Never `#[cfg(test)] mod tests` inside `src/*.rs`, never a helper "just for the test" in `src/`. If a test needs something `pub`, say so and let the user make it `pub`.
  - Tests may only *call* existing API and assert on it. If writing the test would mean writing the implementation (a stub, a fixture reimplementing game logic, a helper computing the expected answer), stop and hint instead.
  - Prefer the smallest check that fails if the logic breaks — the piece-table test that catches a duplicated or out-of-box shape, not a suite per function.
  - This exception does not widen on its own.

## Working agreement

- **The game's `TODO.md` is the authority on what to build and in what order.** Read it before answering "what's next" — the answer is the first unchecked item in the current phase. Don't invent a phase order from memory, and don't duplicate the list anywhere else.
- **Don't jump phases.** Guide through the current phase's unchecked items top to bottom. If the user asks for something from a later phase, say which phase it belongs to and let them decide.
- **One item at a time.** Explain the item, let the user write it, review it, then move on.
- **Read the file; don't ask for a paste.** Open it with Read. Never ask the user to copy-paste source you can read yourself — a compiler error with a file and line number is an instruction to go look. Review what's *actually* on disk, not a snippet that may be stale.
- **A box is checked only when it's true in the code.** Verify before calling a phase done; `cargo check` warnings are a useful to-do list (an unused `mut` means the mutation was never written). If a checked item turns out false, say so and uncheck it.
- **The user checks the boxes**, unless they ask you to. Editing `TODO.md` is allowed — it's not code.
- **If a game has no `TODO.md`**, say so plainly rather than improvising a plan. Ask whether they want one built; only write it if they say yes.

## Games are libraries

Every game in this directory is a workspace member that `termcade` links and runs **in-process**. Not a subprocess.

Each game exposes a `run` function taking `&mut Terminal<B>` and returning `color_eyre::Result<()>`, generic over the backend with `B::Error: std::error::Error + Send + Sync + 'static` — ratatui 0.30 made `Backend::Error` associated, and `?` into `color_eyre::Report` needs those bounds. The game's own `main.rs` is a thin wrapper that calls `ratatui::init()`, calls `run`, then restores — so the game still builds standalone.

**Keep `run` generic.** `ratatui::init()` and `DefaultTerminal` belong in `main.rs` and nowhere else. A concrete `Stdout` in a game loop is precisely what would have to be undone to serve the arcade over SSH, where each session has its own writer and its own reported size.

Registering a new game: add the crate to the workspace members and to `termcade`'s dependencies, add a `Game` entry in `../src/games.rs`, and add its arm to `launch`.

## CI

CI runs `--workspace` with clippy at `-D warnings`, so every game's binary and test targets are compiled and linted on each push. `cargo fmt --all --check` gates formatting.
