# Compatibility Matrix

This document tracks compatibility acceptance for PR #4 review scope.

## API compatibility

| Area | Legacy surface | New surface | Validation |
| --- | --- | --- | --- |
| Button construction | `ButtonWidget::text("label", msg)` | `button("label").on_click(msg)` | `crates/alyx-widgets/tests/compatibility_api.rs` |
| Container construction | `Widget::Container(ContainerWidget { children, layout })` | `row(...)` / `column(...)` | `crates/alyx-widgets/tests/compatibility_api.rs` |
| Runtime/native event bridge | legacy-focused event IDs and fallback routing | `NativeEventState` with focus-first keyboard dispatch + fallback path | `crates/alyx-native/src/lib.rs` tests |

## Native renderer parity baseline

- `native_pixels` and `native_wgpu` consume the same `HeadlessRuntime` and `NativeSceneExport` metrics.
- Both examples route native input through `NativeEventState` and `pump_native_events`.
- `NativeSceneExport` tests now assert deterministic node classification on renderer output.

## Message/update safety

- `Msg: Send` requirements were removed from compile/runtime/native/public boundaries where not required.
- Existing message handlers still use `Clone` to support event duplication and handler tables.

## Verification matrix

- `cargo test -p alyx-widgets --test compatibility_api`
- `cargo test -p alyx-native`
- `cargo test -p alyx-widgets`
