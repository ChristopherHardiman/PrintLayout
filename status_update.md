# PrintLayout – Status Update

**Review date:** 2026-10-11
**Last commit:** `6a8370c` (2025-12-03), "Release v0.2.1: CUPS printer capabilities integration"
**Released version:** v0.2.1 (tags `v0.1.0`, `v0.2.0`, `v0.2.1`)
**Current branch:** `main`, up to date with `origin/main`. The Tauri `gui_rework` branch was deleted (local and remote) on 2026-10-11.
**Dependencies:** Iced 0.13.1, `image` 0.25, `rfd` 0.15, `tokio` 1 (`rust-version = "1.75"`)
**GitHub:** no open issues, no PRs

This was written from a review of the docs ([status.md](status.md), [README.md](README.md), [CHANGELOG.md](CHANGELOG.md), [USAGE.md](USAGE.md), [INSTALL.md](INSTALL.md), [AGENTS.md](AGENTS.md), [GEMINI.md](GEMINI.md), [upgrade_plan.md](upgrade_plan.md)) and the code. I also ran the build, formatter, linter and tests.

---

## 1. TL;DR

- **The shipped Iced app (v0.2.1) is feature-complete for its original scope (Phases 1–7).** It builds and runs, and the 2 unit tests pass.
- **Direction decided: 100% Rust with Iced.** The Tauri + web rewrite was dropped on 2026-10-11 and its prototypes, plan docs and branch were removed (section 5). The next step is upgrading from Iced 0.13 to 0.14 (section 9).
- **There are no keyboard shortcuts in the code, although README and GEMINI.md list them.** There is also no undo/redo, multi-page support or PDF export.
- **Quality gates are green again (2026-10-11).** `make check` (fmt, clippy with `-D warnings`, tests) passes. This is the baseline for the Iced upgrade.
- **A trial upgrade to Iced 0.14.0 compiled cleanly after about 9 mechanical edits** (section 9). I did not run the app or change the repo.

---

## 2. What is done (v0.2.1, Iced 0.13)

| Area | State | Notes |
|---|---|---|
| Project/build setup, Makefile, CI | ✅ | [ci.yml](.github/workflows/ci.yml) runs check, fmt, clippy, test and build |
| Layout engine ([layout.rs](src/layout.rs)) | ✅ | Paper sizes (A/B series, Letter, Legal, Tabloid, photo sizes), paper types, margins, orientation, `PlacedImage` with rotation, flip, opacity and a `locked` flag |
| Canvas ([canvas_widget.rs](src/canvas_widget.rs)) | ✅ | Zoom, selection, 8-handle drag-resize, drag-move, transform-keyed image cache |
| Image tools | ✅ | Rotate 90° CW/CCW, flip H/V, width/height with aspect lock, opacity 0–100%, per-image copies |
| Printing ([printing.rs](src/printing.rs)) | ✅ | CUPS via `lpstat`/`lp`, printer discovery, dynamic capabilities (InputSlot, MediaType, ColorModel, print quality), 300 DPI render, async job with progress modal |
| Persistence ([config.rs](src/config.rs)) | ✅ | `.pxl` JSON projects, config in XDG dirs, 30 s auto-save with recovery prompt, 5 rotating backups, 10 recent files, dirty `(*)` marker |
| File dialogs | ✅ | `rfd` with xdg-portal (no GTK3) |
| Packaging | ✅ | RPM spec ([print-layout.spec](packaging/rpm/print-layout.spec)) and [release-rpm.yml](.github/workflows/release-rpm.yml) for tag-driven releases (COPR-friendly Source0) |
| Docs | ✅ | README, INSTALL, USAGE and CHANGELOG exist |

Code size: about 3,800 lines of Rust. [main.rs](src/main.rs) is 1,927 lines, [printing.rs](src/printing.rs) 599, [layout.rs](src/layout.rs) 490, [canvas_widget.rs](src/canvas_widget.rs) 520 and [config.rs](src/config.rs) 295.

---

## 3. Build and quality check (run today)

| Check | Result |
|---|---|
| `cargo build` | ✅ passes (about 50 s cold) |
| `cargo test` | ✅ 2 pass. Both are in [printing.rs](src/printing.rs): `test_printer_discovery` and `test_printer_capabilities`. |
| `cargo clippy -- -D warnings` | ✅ passes (10 lints fixed: range `contains` checks, a collapsible `if`, a derivable `Default`, an `Option::map`) |
| `cargo fmt --check` | ✅ passes (the code was reformatted) |
| Test coverage | ⚠️ There are no tests for layout math, serialization, config or canvas. [AGENTS.md](AGENTS.md) asks for them. |

Behaviour is unchanged by these fixes.

---

## 4. Gaps between the docs and the code

1. **Keyboard shortcuts are documented but not implemented.** [README.md](README.md) lists Ctrl+N/O/S/Shift+S, Delete, and Ctrl+/−/0/F. [GEMINI.md](GEMINI.md) lists a much larger set (undo/redo, copy/paste, duplicate, rotate, lock, hide, arrow-key nudge). The code has no `subscription`, keyboard or `on_key` handling.
2. **Several features in [GEMINI.md](GEMINI.md) don't exist.** These are undo/redo, copy/paste/duplicate, multi-select, smart snapping, print preview and DPI quality indicators. `snap_to_grid` exists only as a config field. `locked` exists on `PlacedImage`, and I did not check for any UI for it.
3. **README says users can drag files onto the canvas.** I found no file-drop handling in `src/main.rs`.
4. **README lists a log file at `~/.cache/print_layout/app.log`.** I found no file logging, and `~/.cache/print_layout/` is currently empty.
5. **[status.md](status.md) is stale.** It lists "Undo/redo" and "Multi-page" as planned but does not point to the plan for them. It says v0.2.0 was tagged and that Color Management was a tab, but v0.2.1 removed that tab. Its line counts are wrong: main.rs is 1,927 lines (listed as 1,803) and printing.rs is 599 (listed as 419).
6. **Version numbering is confusing.** [upgrade_plan.md](upgrade_plan.md) is titled "Upgrade Plan v0.2.0 / Q1 2026", but 0.2.0 and 0.2.1 are already released.
7. **[Cargo.toml](Cargo.toml) and [.cargo/config.toml](.cargo/config.toml) disagree on `codegen-units`.** [Cargo.toml](Cargo.toml) sets `lto = "thin"` with `codegen-units = 1`. The [.cargo/config.toml](.cargo/config.toml) setting (which Cargo applies over the manifest) sets `thin` with 4. [status.md](status.md) targets about 7 MB for the binary, but the release binary was not measured.

---

## 5. GUI rework (Tauri + web) – dropped

Decision on 2026-10-11: stay with a 100% Rust implementation on Iced. Removed:
- `Prototype_Rust/` and `Prototype_Web/` (the Phase 0 benchmark apps) and [QUICK_START_PROTOTYPES.md](QUICK_START_PROTOTYPES.md).
- `gui_upgrade.md` and the Tauri `upgrade_plan.md` (these only existed on `gui_rework`).
- The `gui_rework` branch, locally and on `origin`. Its only unique commit, `e32eaca`, was the Tauri docs.

The Phase 0 benchmarks were never run, and no results were saved.

---

## 6. Repository hygiene

Cleaned up on 2026-10-11:
- Deleted the stray `layout.ron`, the duplicate `delayed_upgrade_plan.md` (identical to [upgrade_plan.md](upgrade_plan.md)) and the 9 untracked prototype screenshots in `assets/prototype_images/`.
- Dropped the uncommitted global `*.png` rule from `.gitignore`, so future PNGs (for example README screenshots) are not hidden.
- [.cargo/config.toml](.cargo/config.toml) (thin LTO) and [AGENTS.md](AGENTS.md) were committed earlier.

Still open:
- `main` and `master` both exist as branches. CI is configured for `main`/`develop`.
- [GEMINI.md](GEMINI.md) refers to `src/canvas.rs` and `src/ui.rs`, which no longer exist.

---

## 7. Known limitations (carried forward)

- Single-page layouts only.
- No PDF export.
- No undo/redo.
- No RAW or HEIC support.
- The canvas can't pan or scroll for very large pages.
- Regional default paper size is a TODO ([layout.rs](src/layout.rs) line 159: always A4, not Letter for US/Canada).
- There are many `unwrap()` calls (27 across `src/`), which I did not audit.
- The "batch print / copy count" item in [status.md](status.md) is partly there: `copies` is in config and `CopiesChanged`/`ImageCopiesChanged` exist.

---

## 8. Suggested next steps

1. ~~Fix clippy and fmt~~ – done.
2. ~~Clean up uncommitted and stray files~~ – done.
3. ~~Delete prototype images and the `*.png` ignore rule~~ – done.
4. Upgrade to Iced 0.14 (section 9).
5. Implement the documented keyboard shortcuts with `iced::keyboard::listen`, or remove them from the docs.
6. Add unit tests for layout math, `.pxl` serialization and config.
7. Build the planned features per [upgrade_plan.md](upgrade_plan.md): undo/redo first, then multi-page, then PDF export.
8. Refresh [status.md](status.md), for example by merging this file into it.

---

## 9. Outline: upgrading Iced 0.13 → 0.14

**Why:** 0.14.0 was released 2025-12-07. Relevant changes for this app:
- Concurrent image decoding and uploading, which should help the image-heavy canvas.
- A reactive renderer.
- Headless and end-to-end testing support.
- A unified `keyboard::listen` subscription (useful for the missing shortcuts).
- New widgets: `table`, `grid`, `float`, `pin`, `sensor`.
- Animations, and system-theme reactions.

**Trial result:** in a throwaway copy of the project outside this repo, bumping `iced` to `"0.14"` and fixing the errors below gave a clean `cargo check`, with 2/2 tests passing and no new warnings. Clippy reports the same 8 errors as today. The dependency count in `Cargo.lock` went from 584 to 557. The app has **not been run** (no display session), so the runtime checks in step 5 are still to do.

### Step 0 – Prepare
- [x] Cleanup from section 8 is done, so the upgrade diff will be only the upgrade.
- [ ] Create a branch such as `iced-0.14`.
- [ ] Record the baseline: `cargo build --release` size (the 7 MB target in [status.md](status.md) is unmeasured), and a manual smoke test on 0.13 (add images, drag, resize, save/load, one test print).

### Step 1 – Toolchain
- [ ] Iced 0.14 requires **Rust 1.88** (its crates use edition 2024). The project edition can stay 2021.
- [ ] Update `rust-version` in [Cargo.toml](Cargo.toml) (currently 1.75), `BuildRequires: rust >= 1.75` in [print-layout.spec](packaging/rpm/print-layout.spec), and the "Rust 1.70+" in [README.md](README.md) and [GEMINI.md](GEMINI.md).
- [ ] Check that the Fedora/COPR targets in [release-rpm.yml](.github/workflows/release-rpm.yml) ship Rust ≥ 1.88. This is the biggest practical risk.

### Step 2 – Bump the dependency
- [ ] In [Cargo.toml](Cargo.toml): `iced = { version = "0.14", features = ["canvas", "image", "tokio", "debug"] }`. Then `cargo update -p iced`.
- [ ] Optionally add the `crisp` feature (pixel-snapped quads) after the upgrade, if the page border and handles look blurry.

### Step 3 – Code changes found in the trial

| # | Where | 0.13 | 0.14 |
|---|---|---|---|
| 1 | [main.rs](src/main.rs) `main()` | `application(title, update, view).run_with(new)` | `application(new, update, view).title(...).run()` (boot function first, `run_with` removed, title moved to a builder method) |
| 2 | [main.rs](src/main.rs) imports and view | `horizontal_rule(1)`, `vertical_rule(1)` | `rule::horizontal(1)`, `rule::vertical(1)` (`rule` module) |
| 3 | [main.rs](src/main.rs) view (about 20 uses) | `Space::with_width(w)`, `Space::with_height(h)` | `Space::new().width(w)`, `Space::new().height(h)` |
| 4 | [main.rs](src/main.rs) (2 checkboxes) | `checkbox("label", checked)` | `checkbox(checked).label("label")` |
| 5 | [main.rs](src/main.rs) print progress modal | `progress_bar(..).width(..).height(..)` | `.length(..)` and `.girth(..)` |
| 6 | [main.rs](src/main.rs) `theme()` | `Theme::default()` | removed. Use `Theme::Light` to keep today's look, or react to the system theme. |
| 7 | [canvas_widget.rs](src/canvas_widget.rs) `Program::update` | `event: canvas::Event`, returns `(event::Status, Option<Msg>)` | `event: &iced::Event`, returns `Option<canvas::Action<Msg>>` |
| 8 | [canvas_widget.rs](src/canvas_widget.rs) event matches | `canvas::Event::Mouse(..)` | `iced::Event::Mouse(..)` |
| 9 | [canvas_widget.rs](src/canvas_widget.rs) return values | `(Status::Captured, Some(msg))` / `(Status::Ignored, None)` | `Some(canvas::Action::publish(msg).and_capture())` / `None` |

Everything else compiled unchanged, including `Task::perform`/`batch`/`done`, `Cache`, `Frame` image drawing, `image::Handle::from_rgba`, `pick_list`, `text_input`, `scrollable`, `mouse_area`, `opaque` and `center`.

### Step 4 – Verify (automated)
- [ ] `cargo build`, `cargo test` (2 tests), `cargo clippy -- -D warnings` and `cargo fmt --check`. These should match or beat the pre-upgrade baseline.
- [ ] `cargo build --release`, then compare the binary size and link memory against the baseline (the thin-LTO change in [.cargo/config.toml](.cargo/config.toml) was made for memory).

### Step 5 – Verify (manual, run on Wayland and X11)
- [ ] Add several large images and compare canvas pan/zoom and window-resize smoothness with 0.13 (the reason for the thumbnail and source-image caches).
- [ ] Drag, move and the 8 resize handles, including selection and deselection on empty clicks (this is the rewritten `update`).
- [ ] Rotate, flip and opacity on the canvas, versus the printed output.
- [ ] Settings tabs, `pick_list` dropdowns, margin inputs, the new checkbox layout, the print progress modal and the recovery dialog.
- [ ] Save/load `.pxl`, auto-save, recent files, and an actual print job.
- [ ] File dialogs through xdg-portal still work.

### Step 6 – Ship
- [ ] Update [CHANGELOG.md](CHANGELOG.md) and [status.md](status.md), and bump the version (for example 0.2.2, or 0.3.0 together with new features).
- [ ] Merge to `main` and tag. Check that the CI and RPM workflows still build with the newer Rust.

### Optional follow-ups the upgrade makes easier
- Keyboard shortcuts via `keyboard::listen` (currently documented but missing).
- Use `float`/`pin`/`grid`/`table` and `Column::wrap` to simplify the settings and thumbnail panels.
- Headless end-to-end tests for the main flows (the project has almost no tests).
- Animations for panel transitions, and following the system theme.

### Risks
- **Rust 1.88 requirement** may break RPM builds on older Fedora/EL releases.
- **Rendering differences.** 0.14 changed the layout rules (`Shrink` now takes priority over `Fill`), the quad shader and the theme palette generation, so spacing and colours may shift slightly.
- **The canvas `update` rewrite** is the one behavioural change. Event handling for drag and resize needs the most manual testing.
- Iced is pre-1.0, so a later 0.15 upgrade will again have breaking changes.
