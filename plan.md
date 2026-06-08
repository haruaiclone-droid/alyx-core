# Alyx PR #4 Remediation Plan

## Rules
- Only mark `[x]` after implementation is complete and verified.
- Keep unfinished items as `[ ]` and include why they remain.
- Preserve backward compatibility and avoid introducing new crates or deps unless necessary.
- Keep changes additive and minimal; avoid broad redesigns.

## Tasks

### 1. Capture baseline PR diff and supporting files
- Status: [x]
- Files:
  - `Cargo.toml`
  - `README.md`
  - `docs/alyx-overview.md`
  - `.github/workflows/ci.yml`
  - `crates/**/*`
  - `examples/**/*`
  - `crates/**/tests/**/*`
- Completion criteria:
  - PR branch (`pr4-head`) and base (`origin/main`) diff reviewed for added/removed files.
  - All new/changed crates, examples, docs, CI, and tests have been enumerated before code edits.
- Verification:
  - `git diff --stat origin/main..pr4-head`
  - `git diff --name-status origin/main..pr4-head`
  - Manual read-through of `README.md`, `docs/alyx-overview.md`, `.github/workflows/ci.yml`, all `Cargo.toml`, crates, examples, and tests.

### 2. Confirm compatibility impact before modifying behavior
- Status: [x]
- Files:
  - `Cargo.toml`
  - `crates/**/*.rs` (diff scope)
  - `crates/alyx-core/src/lib.rs`
  - `crates/alyx-host/src/lib.rs`
  - `crates/alyx-web/src/lib.rs`
  - `crates/alyx-widgets/src/lib.rs`
- Completion criteria:
  - New symbols are additive and do not remove existing public APIs.
  - CLI behavior remains backward-compatible with previous invocation forms used in PR #4.
  - No new crate dependencies beyond the PR scope are introduced.
- Verification:
  - `git diff origin/main..pr4-head --stat`
  - `git diff origin/main..pr4-head -- crates/alyx-core/src/lib.rs crates/alyx-cli/src/main.rs`

### 3. Fix broken generated HTML in web export
- Status: [x]
- Files:
  - `crates/alyx-web/src/lib.rs`
- Completion criteria:
  - `export_static_html_with_endpoint` emits valid nested script tags (no stray closing tag).
- Verification:
  - `git diff origin/main..pr4-head -- crates/alyx-web/src/lib.rs`
  - Visual inspection of the generated markup in `export_static_html_with_endpoint` output.

### 4. Ensure static dist shape includes required Alyx contract files
- Status: [x]
- Files:
  - `crates/alyx-host/src/lib.rs`
  - `crates/alyx-core/tests/phase_integration.rs`
  - `crates/alyx-core/tests` helper test set
- Completion criteria:
  - `build_static_dist`/`build_static_dist_with_bridge` write `index.html`, `manifest.json`, `alyx-manifest.json`, `app.wasm`, `alyx-loader.js`.
  - `alyx-manifest.json` includes `entry` and `renderer` fields.
- Verification:
  - `git diff origin/main..pr4-head -- crates/alyx-host/src/lib.rs`
  - `git diff origin/main..pr4-head -- crates/alyx-core/tests/phase_integration.rs`
  - Host integration assertions for generated artifact presence.

### 5. Add compact runtime bootstrap helper without heavy abstraction
- Status: [x]
- Files:
  - `crates/alyx-core/src/lib.rs`
- Completion criteria:
  - Minimal `run` helper added in `alyx-core`.
  - Helper is re-exported via `prelude`.
  - API remains additive; no trait/event semantics changed.
- Verification:
  - `git diff origin/main..pr4-head -- crates/alyx-core/src/lib.rs`
  - `git show HEAD:crates/alyx-core/src/lib.rs`

### 6. Document required deployment bundle shape for web hosting
- Status: [x]
- Files:
  - `docs/web-hosting.md`
  - `README.md`
  - `docs/getting-started.md`
- Completion criteria:
  - Hosting docs explicitly list `index.html`, `manifest.json`, `alyx-manifest.json`, `app.wasm`, and `alyx-loader.js`.
  - `getting-started` includes note on ergonomic API entry points and `run` helper.
- Verification:
  - `rg -n "app\.wasm|alyx-loader\.js|alyx-manifest\.json" docs/web-hosting.md README.md docs/getting-started.md`

### 7. Keep `plan.md` status accurate at each step
- Status: [x]
- Files:
  - `plan.md`
- Completion criteria:
  - Every completed task marked `[x]` has explicit completion evidence.
  - Any delayed task remains `[ ]` with reason (if any).
- Verification:
  - Manual review of this file before final PR summary.

### 8. PR4 residual scope tracking
- Status: [x]
- Files:
  - `docs/pr-summary.md`
- Completion criteria:
  - Remaining major roadmap items are documented in PR body and follow-up notes.
- Verification:
  - Manual review of `docs/pr-summary.md` before closing PR #4 scope.
  - Confirm follow-up items include:
    - Browser automation coverage expansion
    - Native renderer completeness roadmap
    - CI smoke reliability hardening

### 9. Fix host runtime key-up dispatch routing
- Status: [x]
- Files:
  - `crates/alyx-host/src/lib.rs`
- Completion criteria:
  - `serve_one_runtime` now routes `EventType::KeyUp` with valid `(node, element)` IDs through `dispatch_keyup_by_ids`.
  - The coordinate fallback to `dispatch_keyup` remains for ID-less keyboard events.
- Verification:
  - Manual code review of the `EventType::KeyUp` branch in `crates/alyx-host/src/lib.rs` in this fix commit.

### 10. Local verification command confirmation
- Status: [x]
- Files:
  - `N/A (environment checks)`
- Completion criteria:
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets --all-features`
  - `cargo test --workspace --all-features`
  - `cargo build --workspace --all-features`
- Verification:
  - Execute the above commands locally and confirm pass.
  - Additional required commands run in this pass:
    - `cargo doc --workspace --no-deps --all-features`
    - `rustup target add wasm32-unknown-unknown`
    - `cargo build --workspace --target wasm32-unknown-unknown`
    - `cargo run --package alyx-cli --bin alyx -- help`
    - `cargo run --package alyx-cli --bin alyx -- build-web dist`
    - `cargo check --package alyx-native --example native --features winit-backend`
    - `cargo check --package alyx-native --example native_pixels --features pixels-backend`
    - `cargo check --package alyx-native --example native_wgpu --features wgpu-backend`
  - `cargo test --workspace --all-features` requires `CARGO_BUILD_JOBS=1` in this environment to avoid a transient example artifact lock race.
  - All command batches completed successfully; two Cargo warnings remain (see task 11).

### 11. Resolve workspace artifact/build warnings before PR merge (optional hardening)
- Status: [ ]
- Files:
  - `crates/alyx-cli/Cargo.toml`
  - `Cargo.toml` workspace example name strategy
- Completion criteria:
  - Remove duplicate binary target name warning:
    - Keep one `alyx` binary target only; duplicate source mapping to a second bin should be avoided.
    - ✅ Done in `crates/alyx-cli/Cargo.toml`: removed the redundant `alyx-cli` bin alias.
  - Remove example output filename collisions between `alyx-core` and `alyx-examples` in the same workspace target directory.
  - CI should keep passing with warnings as either resolved or justified.
- Verification:
  - `cargo test --workspace --all-features` (warning-free for target/output naming) in a clean cache scenario.
  - Confirm no `output filename collision` or duplicate-bin warning lines remain.
