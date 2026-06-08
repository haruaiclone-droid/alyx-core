# Native Renderers

`alyx-native` exposes:

- `NativeRenderer` trait
- `NativeEventLoop` trait
- `NativeEventState` shared input/dispatch state for native backends
- `NativeSceneExport` summary helper to derive simple scene metrics from rendering plans
- `SkeletonAdapter` as an extension template
- `pump_native_events` helper that repeatedly polls `NativeEventLoop` and dispatches into `HeadlessRuntime`
- `WinitEventLoop` (optional behind `winit-backend` feature) to source events from a local desktop window
- `pixels-backend` example (`native_pixels`) that wires `HeadlessRuntime`, `Pixels`, and native pointer/keyboard events into a real desktop preview
- `wgpu-backend` example (`native_wgpu`) that wires `HeadlessRuntime`, `winit`, and a minimal GPU render loop

## Default integration mode

The crate always provides a minimal backend-agnostic bridge:

- implement a `NativeEventLoop` for your platform layer
- pass it into `pump_native_events(runtime, renderer, event_loop)`
- map polled events to `HeadlessRuntime` dispatch calls

This is currently wired by `alyx-native::pump_native_events` and the event variants:

- pointer events: down/up/move
- wheel/scroll
- keyboard down/up (with text + code variants)
- focus/blur
- exit sentinel

`pump_native_events` now keeps the most recent focused `(node_id, element_id)` from native focus/blur events, and routes subsequent keyboard events through runtime ID-based handlers when possible. This is the bridge-side foundation for richer focus-sensitive keyboard routing on desktop backends.

## `winit-backend` feature

Enable with:

```toml
[features]
winit-backend = ["alyx-native/winit-backend"]
pixels-backend = ["alyx-native/pixels-backend"]
wgpu-backend = ["alyx-native/wgpu-backend"]
```

Use example:

```bash
cargo run --package alyx-native --example native --features winit-backend
cargo run --package alyx-native --example native_pixels --features pixels-backend
cargo run --package alyx-native --example native_wgpu --features wgpu-backend
```

This example uses `pump_native_events` with `WinitEventLoop` and exits when the request-exit command is emitted.
`native_pixels` uses the same `HeadlessRuntime` compile/render/update flow with direct event-loop rendering through `pixels`.

## What this gives

- real native event source for prototyping native desktop flow
- no hard dependency on `winit` unless the feature is enabled
- stable bridge path for native render backends (`pixels`, `wgpu`) and event/ID dispatch
