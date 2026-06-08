# Alyx PR #4 Remediation Plan

## Rules
- Only mark `[x]` after implementation and verification are complete.
- Leave unfinished items as `[ ]` and include an explicit reason.
- Prefer additive, minimal changes that preserve compatibility.
- Keep `plan.md` aligned with code and PR notes at the end.

## Tasks

### 1. Baseline review and scope lock
- Status: [x]
- Files:
  - `README.md`
  - `docs/alyx-overview.md`
  - `root Cargo.toml`
  - `crates/*/Cargo.toml`
  - `crates/**/*` (diff scope)
  - `examples/**/*`
  - `crates/*/tests/**/*`
  - `.github/workflows/ci.yml`
- Completion criteria:
  - Required project files are reviewed before edits.
  - No new crates or dependency families are introduced solely by this review pass.
- Verification:
  - `Get-Content -Raw README.md`
  - `Get-Content -Raw docs/alyx-overview.md`
  - `Get-ChildItem -Recurse -Filter Cargo.toml -File`
  - `Get-Content -Raw .github/workflows/ci.yml`

### 2. Preserve PR #4 API compatibility while hardening
- Status: [x]
- Files:
  - `crates/alyx-core/src/lib.rs`
  - `crates/alyx-runtime/src/lib.rs`
  - `crates/alyx-host/src/lib.rs`
  - `crates/alyx-web/src/lib.rs`
  - `crates/alyx-widgets/src/lib.rs`
- Completion criteria:
  - New symbols remain additive.
- Verification:
  - Manual review of exports/events/CLI entrypoints for changed behavior and public signatures.

### 3. Remove unnecessary compiler trait bound (`Msg: Send`)
- Status: [x]
- Files:
  - `crates/alyx-compiler/src/layout.rs`
- Completion criteria:
  - `compile` path accepts message types that are `Clone` but not `Send`.
  - Public API signature is not stricter than required.
- Verification:
  - `rg -n "Msg: Clone \\+ Send|where\\s*Msg: Clone \\+ Send" crates/alyx-compiler/src`

### 4. Add compiler regression test for non-Send message types
- Status: [x]
- Files:
  - `crates/alyx-compiler/tests/compile.rs`
- Completion criteria:
  - Non-`Send` test message type compiles through `compile`.
  - Hit area and handler table paths are exercised.
- Verification:
  - `cargo test -p alyx-compiler --test compile compile_accepts_non_send_messages`

### 5. Normalize completion and PR-facing docs
- Status: [x]
- Files:
  - `docs/completion-checklist.md`
  - `README.md`
- Completion criteria:
  - Checklist and README only claim implemented items.
  - Remaining follow-up topics are explicitly deferred.
- Verification:
  - Manual review of both documents after patch.

### 6. Verify targeted compile/test evidence before merge handoff
- Status: [x]
- Files:
  - `crates/alyx-compiler`
- Completion criteria:
  - Targeted test suite in compiler scope is green.
  - No new warnings introduced by added test.
- Verification:
  - `cargo test -p alyx-compiler --test compile`

### 7. Record residual PR scope (browser/native/CI depth)
- Status: [x]
- Files:
  - `docs/pr-summary.md`
  - `docs/implementation-notes.md`
- Completion criteria:
  - Follow-up scope includes browser automation depth, native renderer hardening, and CI reliability.
  - Completion claims avoid full-production-ready assertions.
- Verification:
  - Manual review before PR draft completion.

### 8. Widen runtime/host/native public bounds to `Clone`-based message models
- Status: [x]
- Files:
  - `crates/alyx-runtime/src/lib.rs`
  - `crates/alyx-host/src/lib.rs`
  - `crates/alyx-native/src/lib.rs`
  - `crates/alyx-core/src/lib.rs`
- Completion criteria:
  - Runtime, host, and native dispatch/public entrypoints no longer require `Msg: Send`.
  - Cross-target message models can use non-`Send` payloads when threading is not involved.
- Verification:
  - `rg -n "A::Message: Send|Msg: Clone \\+ Send|Send\\b" crates/alyx-runtime/src crates/alyx-host/src crates/alyx-native/src crates/alyx-core/src`
  - `cargo test -p alyx-runtime`
  - `cargo test -p alyx-host`
  - `cargo test -p alyx-native`

### 9. Make CLI `build-web` output deterministic for `app.wasm`
- Status: [x]
- Files:
  - `crates/alyx-cli/src/main.rs`
- Completion criteria:
  - `build-web` always writes an `app.wasm` for a successful run even if wasm compilation fails.
  - Existing `app.wasm` is replaced by a valid placeholder when runtime copy/build fails, avoiding stale artifacts.
- Verification:
  - `cargo test -p alyx-cli`
  - `cargo run --package alyx-cli -- build-web <tmp_dir>` (manual spot-check)

### 10. Normalize button creation in examples and tests to builder API
- Status: [x]
- Files:
  - `examples/form.rs`
  - `examples/image.rs`
  - `crates/alyx-core/examples/nested_layout.rs`
  - `crates/alyx-core/tests/phase_integration.rs`
  - `crates/alyx-native/examples/native.rs`
  - `crates/alyx-native/examples/native_pixels.rs`
  - `crates/alyx-native/examples/native_wgpu.rs`
- Completion criteria:
  - Replaced button construction is using `button("label").on_click(msg)` (or equivalent helper use) in updated sample code.
  - No `ButtonWidget::text(..., ...)` remains in these target files.
  - No additional behavior change beyond API usage shape.
- Verification:
  - `rg -n "ButtonWidget::text" examples crates/alyx-core/examples crates/alyx-native/examples crates/alyx-core/tests`
  - `cargo test -p alyx-core --test phase_integration`
  - `cargo check --workspace --examples`

### 11. Reuse shared counter sample across web and native desktop example path
- Status: [x]
- Files:
  - `crates/alyx-native/examples/native.rs`
  - `examples/shared_counter.rs`
- Completion criteria:
  - Native winit example uses the same shared counter widget tree/state model as web counter.
  - No duplicated counter business logic (`increment`/`reset`) in native counter example.
  - Compile/runtime smoke for native winit example remains green with feature gate.
- Verification:
  - `cargo check -p alyx-native --example native --features winit-backend`
  - `cargo check --workspace --examples`
  - `cargo test --workspace --all-features`

### 12. Add runtime endpoint integration test for click/enter event handling
- Status: [x]
- Files:
  - `crates/alyx-host/src/lib.rs`
  - `crates/alyx-host/Cargo.toml`
- Completion criteria:
  - `serve_one_runtime` processes `/__alyx_event` POST with click and Enter key payloads.
  - Runtime state updates are asserted from an observable channel and match expected value (`2`).
  - Test compiles without clippy-significant warnings and does not affect public API.
- Verification:
  - `cargo test -p alyx-host --tests`
  - `cargo test --workspace --all-features`

### 13. Align CLI demo and docs on button builder-first API
- Status: [x]
- Files:
  - `crates/alyx-cli/src/main.rs`
  - `docs/getting-started.md`
  - `docs/widgets.md`
- Completion criteria:
  - CLI demo UI and public docs consistently showcase `button(\"label\").on_click(...)` style.
  - No regression to existing behavior in CLI static export and host preview.
- Verification:
  - `cargo check -p alyx-cli`
  - `cargo test --workspace --all-features`
  - `cargo run -p alyx-cli -- build-web`
  - `cargo run -p alyx-cli -- serve 35123 dist` (single request smoke)
