# Implementation Notes

Date: 2026-06-08

## Scope from request

The attached request asks for a phased, end-to-end Alyx implementation:

- Phase 0: Baseline + repository audit
- Phase 1-17: Add architecture layers, compiler/runtime/event/executor/platform support, examples, tests, docs, and PR-ready summary

## Current repository state

- Workspace now includes the full conceptual crate stack:
  - `alyx-core`
  - `alyx-ir`
  - `alyx-plan`
  - `alyx-compiler`
  - `alyx-widgets`
  - `alyx-runtime`
  - `alyx-executor`
  - `alyx-web`
  - `alyx-host`
  - `alyx-native`
  - `alyx-cli`
- IR/Plan/Runtime/Event flows are implemented with stable ids and accessibility fields.
- Host helpers for static export and local serving are available, with a browser runtime event bridge for local preview and runtime-backed serve path.
- CLI smoke entrypoints exist for `--help`, `build-web`, and `serve`.
- Docs and example coverage were expanded in a single implementation pass to match the pasted plan phases.

## 1PR execution status (this pass)

- Added and stabilized crate expansion for all conceptual layers in the pasted plan.
- Implemented command-based runtime dispatch and nested message handling in `alyx-runtime`.
- Added hit-testing/event helper APIs in `alyx-executor` (`hit_areas_at`, `pick_topmost_hit`, `dispatch_event`).
- Expanded `alyx-widgets` with ergonomic constructors and conversion targets (`Checkbox`, `TextInput`, `Link`, `List`, `Scroll`, `Stack`, `Custom`).
- Added example entrypoints for:
  - hello
  - counter
  - layout
  - web counter
  - form
  - image
  - static export
  - nested layout
  - accessibility/focus
- Added `alyx-cli` command implementations for basic static preview flows, now backed by runtime for local serve.
- Added `alyx-native/examples/native.rs` demonstrating an event-pumped desktop preview loop with optional `winit` backend.
- Added compiler propagation for `LinkWidget` destinations into `NavigationPlan::actions` (`NavigateTo`), and coverage for this path in compiler tests.
- Added `alyx-native/examples/native_pixels.rs` and `alyx-native/examples/native_wgpu.rs` demonstrating live pixel- and GPU-backed native preview paths with direct event handling.
- Added shared native event-state handling via `alyx-native::NativeEventState` and scene summary support so both pixel and wgpu backends share one input bridge path.

## Current pass summary

- Workspace and dependencies now align with a unified stack in one pass.
- Execution/host bridge and CLI smoke surfaces are present in code and CI command matrix.
- Native adapter integration is runnable in demo form with `winit` event sourcing; `pixels` and `wgpu` examples now provide concrete native render paths.
- Both native backends now share one input state abstraction through `NativeEventState`, and both consume scene summaries for parity-safe rendering behavior.

### Verification posture (workspace)

- Command-based verification is planned for this workspace, but `cargo` is not available in this environment (`cargo` command is not in PATH).
- As a result, workspace-wide verification (`test`, `build`, `fmt`, `clippy`, `doc`, `wasm`, and CLI smoke commands) remains pending.
- Known environment constraints on this host remain around toolchain availability and PATH defaults.
- The CI `smoke` job now includes an HTTP smoke for `alyx-cli serve` (`/` and `/index.html`), but this still requires a working Rust toolchain to execute.

## Completion matrix draft (for single PR scope)

### Done

- Repository expansion and crate boundaries (`alyx-ir`, `alyx-plan`, `alyx-compiler`, `alyx-runtime`, `alyx-executor`, `alyx-widgets`, `alyx-web`, `alyx-host`, `alyx-native`, `alyx-cli`, `alyx-core`).
- ID-aware RP/EP generation (`NodeId`/`ElementId`) and accessibility metadata pipeline.
- Runtime/headless execution loop with event dispatch helpers.
- `alyx-web` now emits accessibility metadata attributes from rendering plan entries.
- Browser bridge/parsing includes navigation event types for future navigation action handling.
- Example gallery and host/CLI proof points.
- Disabled `HitArea` behavior now suppresses runtime handler registration while keeping accessibility metadata, supporting disabled-state semantics.


### In-progress / intentionally minimal

- Full browser renderer beyond static export and bridge.
- Full phase-by-phase integration tests across all commands, web smoke, and native paths.
