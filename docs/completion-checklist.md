# Completion Checklist (single PR scope)

This checklist maps the requested phases to implemented artifacts.

## Implemented in this PR

- [x] Workspace and crate expansion for full stack layers.
- [x] `alyx-ir` identity/accessibility additions.
- [x] `alyx-widgets` API with `IntoIr` conversion.
- [x] Compiler emitting `RenderingPlan` / `EventPlan` / `HandlerTable` with stable IDs.
- [x] `alyx-runtime` `App` / `HeadlessRuntime` update loop.
- [x] `alyx-executor` contracts (`RenderingPlanExecutor`, `EventPlanExecutor`) and helper renderers.
- [x] Web helper exports / host static build and preview host pipeline.
- [x] Runtime-integrated preview path (`serve_http_with_runtime`) for local event bridge.
- [x] Native adapter traits and event loop demo scaffolding.
- [x] Native examples for `native`, `native_pixels`, and `native_wgpu` features.
- [x] Extended examples (`hello`, `counter`, `layout`, `web counter`, `nested layout`, `form`, `focus`, `image`, `static_export`).
- [x] `alyx-core` top-level re-export surface update.
- [x] Implementation docs and PR summary updates.
- [x] Integration test coverage in `crates/alyx-core/tests/phase_integration.rs`.
- [x] CI workflow definition and local CLI smoke pipeline.
- [x] Event bridge coverage for pointer/keyboard/focus/submit/scroll/navigation paths.
- [x] Keyboard activation fallback to click when explicit keydown handler is absent.
- [x] Accessibility metadata export and ARIA attribute emission.

## Acceptance evidence

- CI job definitions in `.github/workflows/ci.yml`.
- Core implementation evidence in:
  - `crates/alyx-core/src/lib.rs`
  - `crates/alyx-runtime/src/lib.rs`
  - `crates/alyx-host/src/lib.rs`
  - `crates/alyx-web/src/lib.rs`
  - `crates/alyx-native/src/lib.rs`
  - `crates/alyx-executor/src/lib.rs`
  - `crates/alyx-compiler/src/compile.rs`
  - `crates/alyx-widgets/src/lib.rs`
  - `crates/alyx-core/tests/phase_integration.rs`
- Docs and examples in `docs/*` and `examples/*`.

## Current status

- Full command-level green status is intentionally sourced from CI runs, not local manual claims in this review pass.
