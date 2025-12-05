# GUI Rework Proposal: Tauri + Web Stack

**Author:** PrintLayout Team  \
**Date:** 2025-12-04  \
**Target Version:** 0.3.0 (proposed)  \
**Objective:** Replace the current Iced-based interface with a Tauri + Web front end that more closely matches Canon Professional Print & Layout both visually and functionally.

---

## 1. Purpose & Outcomes
- Deliver a modern, GPU-accelerated UI that mirrors Canon PPL paneling, typography, and workflow.
- Preserve existing Rust business logic (printing pipeline, CUPS integration, layout math) while exposing it to the web layer through Tauri commands/events.
- Establish a front-end foundation that can evolve rapidly (animations, richer styling) without touching core printing logic.

Success Criteria:
- All current features (layout editing, printer selection, CUPS option management, job submission) function through the new UI.
- Canvas interactions (pan/zoom, drag/drop) hit target latency (<16 ms per frame on reference hardware).
- Packaging pipeline (RPM, GitHub release) produces installers that include the Tauri shell with static web assets.

---

## 2. Scope Definition
### In Scope
1. **Architecture & Tooling**
   - Introduce Tauri 2.x with a vanilla (non-framework) front end initially, optionally upgrading to a modern framework later.
   - Define IPC contracts (`#[tauri::command]` functions, event channels) for all UI ↔ Rust interactions.
2. **Rust Backend Refactor**
   - Extract UI-agnostic state management into reusable crates/modules (e.g., `core`, `printing`, `layout`, `history`).
   - Formalize DTOs (serde-serializable structs/enums) shared with the web layer.
3. **Web UI Implementation**
   - Recreate current tabs/panels using HTML/CSS with a Canon-like theme (layered panels, subtle gradients, typographic hierarchy).
   - Implement layout canvas using `<canvas>` + WebGL/Canvas2D with smoothing, rulers, guides.
   - Build component library (panel shells, dropdowns, segmented controls) for reuse.
4. **Dev Experience & Testing**
   - Set up Node/NPM toolchain (Vite or vanilla dev server) with hot reload bridged to Tauri dev mode.
   - Establish lint/test suites for both Rust and web code (ESLint, Playwright snapshot tests for UI flows).
5. **Packaging & Distribution**
   - Update CI workflows to install Node, build front-end assets, and bundle them via `tauri build` before RPM packaging.
   - Ensure signing/notarization steps (if future macOS/Windows support is desired) are documented.

### Out of Scope (for this phase)
- Rewrite of core printing logic or CUPS integrations (these stay in Rust).
- Adding new feature areas (multi-page, PDF export) beyond ensuring their hooks exist; those remain separate roadmap items.
- Broad theming/skin support; focus on one canonical Canon-inspired design.

---

## 3. Assumptions & Dependencies
- Team can install the Node LTS toolchain plus Tauri CLI on build agents and developer machines.
- Existing Rust modules compile to `cdylib`/`rlib` targets that Tauri can link statically.
- Canvas rendering strategy decided up front (pure browser rendering vs. Rust-generated bitmaps streamed via IPC).
- Adequate design references/screenshots of Canon PPL to inform layout decisions.

Dependencies:
- Completion of `history` module (undo/redo) so state diffs can be surfaced cleanly to the web UI.
- Documentation of all current `Message` handlers so equivalent web commands can be created.

---

## 4. Workstreams & Tasks

### 4.1 Architecture & Command Surface
- Inventory existing Iced `Message` variants → map to Tauri commands/events.
- Define JSON schemas (Serde structs) for printer capabilities, layout state, job status.
- Set up shared crate (`crates/ui_bridge`) exporting these DTOs to both Rust backend and TypeScript bindings (via `ts-rs` or manual generation).

### 4.2 Backend Preparation
- Decouple UI state from `Application` struct; expose async functions callable via commands (e.g., `async fn list_printers()`).
- Implement subscription/event emitters for long-running jobs (print progress, render tasks).
- Add error taxonomy so the UI can display actionable messages.

### 4.3 Tauri Project Bootstrapping
- Run `cargo tauri init --frontend vanilla` inside repo (possibly `/app` folder) and configure workspace members.
- Configure `tauri.conf.json` with required capabilities (filesystem access for opening images, printing privileges where applicable).
- Integrate existing logging/tracing with Tauri’s logging bridge for unified diagnostics.

### 4.4 Front-End Implementation
- Build base layout: top toolbar, left asset column, center canvas, right settings panel, bottom status strip.
- Style panels with CSS variables mirroring Canon palette; implement responsive rules for 1080p/4K.
- Canvas subsystem:
  - Decide between WebGL scene graph vs. Canvas2D; prototype both for performance.
  - Implement zoom/pan, selection outlines, ruler overlays, drop targets.
  - Bridge input events (drag, resize handles) back to Rust via debounced command calls.
- Forms & dropdowns: wrap Tauri invoke calls with a lightweight state store (plain JS or minimal state library).

### 4.5 Dev Tooling & Testing
- Add npm scripts for `dev`, `build`, `lint`, `test`.
- Configure ESLint + Prettier (or equivalents) and integrate with CI.
- Write integration tests:
  - Web: use Playwright to automate key flows (load printers, change options, start print job).
  - Rust: add contract tests ensuring commands return expected payloads consumed by the UI.

### 4.6 Packaging & Deployment
- Update GitHub Actions workflow to install Node, run `npm ci`, build assets, then call `cargo tauri build --bundles none` to produce the combined binary before RPM packaging.
- Embed static assets in Tauri bundle and configure CSP to limit external access.
- Document installer size/memory deltas vs. Iced build.

### 4.7 Validation & Rollout
- Performance profiling sessions on reference hardware to compare canvas latency vs. current release.
- Beta flag: allow running either Iced UI or Tauri UI via feature flag/env var until parity confirmed.
- Collect user feedback from test build; plan incremental UI polish sprints.

---

## 5. Timeline (Rough)
| Phase | Duration | Key Outputs |
| --- | --- | --- |
| Preparation & architecture | 1 week | Command map, DTO definitions, workspace layout |
| Backend refactor for commands | 1-2 weeks | Tauri-ready async APIs, event emitters |
| Front-end scaffold & styling | 2 weeks | Canon-inspired shell, basic forms, nav |
| Canvas implementation | 2 weeks | High-performance preview with interactions |
| Integration & feature parity | 1 week | All existing workflows functional |
| QA & packaging updates | 1 week | Automated tests, CI builds, beta release |

Total estimated effort: **8 weeks** (overlapping where possible).

---

## 6. Risks & Mitigations
- **WebView performance below expectations** → Prototype canvas early; fall back to Rust-rendered bitmaps if WebGL proves costly.
- **Tooling learning curve (Node/NPM)** → Adopt vanilla setup first; add frameworks incrementally; document common commands in `CONTRIBUTING.md`.
- **State drift between Rust and Web UI** → Centralize state transitions in Rust, send patches/diffs instead of duplicating logic client-side.
- **Packaging complexity** → Extend CI gradually, keep Iced build pipeline until Tauri release stabilizes.
- **Accessibility & localization gaps** → Use semantic HTML, plan for future i18n by structuring copy in resource files from day one.

---

## 7. Resource Needs
- 1 Rust engineer (backend refactor, Tauri command layer).
- 1 Front-end engineer/designer comfortable with CSS/layout to recreate Canon look.
- Shared QA resource for cross-platform smoke tests.
- Build infrastructure updates: Node LTS, Tauri CLI, WebKitGTK dev packages on CI runners.

---

## 8. Next Steps
1. Approve this scope and allocate owners per workstream.
2. Spin up `feature/tauri-ui` branch and initialize Tauri workspace.
3. Document command contracts and begin backend extraction.
4. Produce UI mockups that match Canon PPL to guide CSS implementation.
5. Schedule prototype review at end of the scaffold phase.

Once these steps are underway, the team can iterate toward a full Tauri-based release while keeping current Iced builds available for production users until parity is achieved.
