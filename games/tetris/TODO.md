# TODO — tetrotui

Build in phase order. Each phase ends **playable** before starting next.
Tune input-feel in `--release` only (debug frame times lie).

---

## Phase 0 — Foundation (do before Phase 1)

- [x] Pick crates: `ratatui`, `crossterm` (backend), `rand` (bag), a config loader (`toml` + `serde`).
- [x] Module layout enforcing the state/render split:
  - [x] `state` module — `Board`, `GameState`, pieces. **No ratatui types here, ever.**
  - [x] `render` module — function over `&GameState` → ratatui `Frame`.
  - [x] `input` module — non-blocking poll each frame.
  - [x] `config` module — DAS/ARR + tuning, loaded from file day one.
  - [x] `main` — terminal setup/teardown, the loop.
- [x] Config struct + config file loaded at startup (even if only 2 fields now).
- [x] Fixed-tick game loop: gravity + timers on tick, input polled non-blocking. **Loop not driven by input.**

---

## Phase 1 — Playable core (one evening)

- [x] Grid: fixed-size `Vec`/array of `Option<PieceKind>`. 10 wide × 20 visible + hidden spawn rows above.
- [x] Seven `PieceKind` (I O T S Z J L) with the 7 canonical colors (cyan/yellow/purple/green/red/blue/orange).
- [x] Active piece = kind + rotation index (0–3) + origin. **Store index, not rotated matrix.**
- [x] Full-block `██` rendering (two columns per cell).
  - [x] `color(PieceKind) -> Color` mapping lives in `render.rs`, not `state.rs`.
  - [x] Cell → `Span`, row → `Line`, board → `Vec<Line>`; visible rows only (`HIDDEN_HEIGHT..HEIGHT`).
  - [x] Size guard first: below min → "needs 24×30", early return (avoids `u16` underflow on centering math).
  - [x] Centered `Rect` (`WIDTH * 2` wide × `VISIBLE_HEIGHT` tall) + `frame.render_widget`.
  - [x] Hand-seed cells in `main` to confirm it actually draws.
  - [ ] Border around the playfield: `Block::bordered()` on the board `Paragraph`. **Bump the rect to `WIDTH*2 + 2` × `VISIBLE_HEIGHT + 2`** — a border eats one cell per side and will silently clip 2 columns/rows otherwise. Desaturated border style (`DarkGray`), pieces stay the only saturated thing.
- [x] Piece base shapes: one flat (rotation 0) shape per kind. Box sizes: I = 4×4, O = 2×2, rest = 3×3. **Wrong box size = piece drifts on rotate.**
- [ ] Spawn: flat orientation, in the two rows above visible field. Get position exact.
- [ ] Naive matrix rotation (NO kicks yet). Rotate the offsets, not the origin.
- [ ] Collision check (walls, floor, settled cells).
- [ ] Fixed gravity drop (`GameState.drop_timer` exists, unused).
- [ ] Lock on landing → merge into grid.
- [ ] Line clear detection + row collapse.
- [ ] Game over (spawn blocked).
- [ ] Resize handling: recompute layout + redraw; below min size → "needs 24×30" message.

---

## Phase 2 — Ghost piece (~20 min, huge feel gain)

- [ ] Compute drop landing column-wise.
- [ ] Render dimmed/outlined ghost. **Never filled** (competes with active piece).

---

## Phase 3 — Hard drop

- [ ] Instant drop to landing + lock.
- [ ] Brief visual trace so drop reads as an event.

---

## Phase 4 — Lock delay

- [ ] ~500ms timer on landing, **visible** (piece brightens / border firms during window).
- [ ] Move-reset: successful move/rotate restarts timer.
- [ ] Reset cap: 15 resets → forced lock (prevents infinite stall).

---

## Phase 5 — 7-bag randomizer

- [ ] Shuffle each permutation of the seven pieces. NOT uniform random.
- [ ] Next-queue preview in HUD.

---

## Phase 6 — DAS/ARR (expect longer than it sounds)

- [ ] DAS (delayed auto-shift) + ARR (auto-repeat rate) from config.
- [ ] Wire into the fixed-tick loop.
- [ ] Tune in `--release`.

---

## Phase 7 — SRS wall kicks

- [ ] Kick offset table between rotation states.
- [ ] I-piece own table; J L S T Z share one; O doesn't kick.
- [ ] Apply kicks on rotate; first passing offset wins.
- [ ] Tests: `srs_kick_i_piece` etc.

---

## Phase 8 — Half-block rendering

- [ ] `▀` with distinct fg/bg per cell → two vertical pixels per row.
- [ ] Should be a one-file change (render module only) if state stayed clean.
- [ ] Sub-cell animation: line clears, lock settle.

---

## Feel checklist (easy to miss)

- [ ] Line clear beat: 150–200ms flash/collapse before rows vanish.
- [ ] Lock delay cap wired (see Phase 4).
- [ ] Piece colors exact convention; chrome stays desaturated — pieces only saturated thing.
- [ ] Layout tight — no side-panel padding to fill 80 cols.
- [ ] Hold slot (add when convenient).

---

## Discipline (don't undo)

- [ ] `GameState` has zero ratatui types (no `Rect`, no `Color`).
- [ ] Loop tick-driven, input polled — never input-driven.
- [ ] DAS/ARR in config from file, not hardcoded.
- [ ] Rotation = index, not matrix.
- [ ] Hold slot + lock-delay timer live in state.
