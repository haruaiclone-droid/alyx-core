# Completion Checklist (single PR scope)

This checklist maps the requested phases to implemented artifacts.

## Done

- [x] Workspace and crate expansion for full stack layers.
- [x] IR identity/accessibility additions.
- [x] Widget API with IntoIr conversion.
- [x] Compiler emitting RenderingPlan/EventPlan/HandlerTable with stable IDs.
- [x] Runtime (`App`, `HeadlessRuntime`) and command dispatch flow.
- [x] Executor contracts (`RenderingPlanExecutor`, `EventPlanExecutor`) and trace/memory renderers.
- [x] Web helper exports / host static build and preview host pipeline.
- [x] Hosted static preview server (`serve_http`) skeleton that serves generated dist files.
- [x] Runtime-integrated preview path (`serve_http_with_runtime`) to keep plan and event bridge in sync during local serve.
- [x] Native adapter traits and skeleton.
- [x] Native event-loop demonstration example (`crates/alyx-native/examples/native.rs`) behind `winit-backend` feature.
- [x] Native pixel rendering example (`crates/alyx-native/examples/native_pixels.rs`) behind `pixels-backend` feature.
- [x] Native wgpu rendering example (`crates/alyx-native/examples/native_wgpu.rs`) behind `wgpu-backend` feature.
- [x] Example entrypoints for hello / counter / layout / web counter.
- [x] Additional examples for nested layout, form/input, accessibility/focus, image, and static export.
- [x] `Row`/`Column` widget `IntoIr` conversions.
- [x] PR-oriented implementation documentation set.
- [x] Extended event handling pipeline for hover/pointer/key/focus/submit hit areas.
- [x] Event handling now respects disabled `HitArea`s (no registered handlers when disabled).
- [x] CI workflow scaffold at `.github/workflows/ci.yml` covering fmt/clippy/test/build/doc/wasm/examples/cli matrix.
- [x] Browser event bridge now forwards dataset IDs for focus/blur/submit and active-keyboard targets, with runtime ID-based dispatch fallback.
- [x] `LinkWidget` `href` now feeds `NavigationPlan` via compile output (`NavigationAction::NavigateTo`).
- [x] `alyx-cli` package scaffold to support CLI smoke (`cargo run --package alyx-cli -- --help`).
- [x] CLI preview smoke in CI: `cargo run --package alyx-cli -- build-web dist` then `serve` returns HTTP `200 OK` on `/` and `/index.html`.
- [x] Web export now emits basic ARIA metadata attributes from accessibility plan entries.
- [x] Browser event parser now recognizes explicit navigation events (`navigate_back` / `navigate_forward`) through event payloads.
- [x] Keyboard activation enhancement: Enter/Space keydown dispatch falls back to click handlers when explicit keydown handlers are not registered.
- [x] Expanded integration tests for major phase coverage via `crates/alyx-core/tests/phase_integration.rs`.
- [x] Native renderer production parity across backends (shared input state abstraction + scene export/summary path used by pixel/wgpu examples).
- [x] Full CI matrix execution with successful runs captured (local GNU+MinGW-backed run completed; CI remains source of authoritative green on hosted runners).

## Evidence

- Crate edits: `crates/*`
- Docs: `docs/*`
- Examples: `crates/alyx-core/examples/*` (plus legacy root `examples/*`)
- Host/runtime/event evidence: `crates/alyx-web/src/lib.rs`, `crates/alyx-host/src/lib.rs`, `crates/alyx-runtime/src/lib.rs`
- Native bridge evidence: `crates/alyx-native/src/lib.rs`
- Native desktop example evidence: `crates/alyx-native/examples/native.rs`
- GPU-backed desktop example evidence: `crates/alyx-native/examples/native_wgpu.rs`
- Local environment note: command checks are now passing locally when using the GNU MinGW toolchain from  
  `C:\\Ruby34-x64\\msys64\\ucrt64\\bin` with `cargo +stable-x86_64-pc-windows-gnu`.
- Verified locally:
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - `cargo test --workspace --all-features`
  - `cargo build --workspace --all-features`
  - `cargo doc --workspace --no-deps --all-features`
  - `cargo build --target wasm32-unknown-unknown --workspace`
  - `cargo run --package alyx-cli -- --help`
  - `cargo run --package alyx-cli -- build-web dist`
  - `cargo run --package alyx-cli -- serve 38888 dist`
