# Completion Checklist (PR #4 Review Scope)

## Implemented
- [x] One-pass workspace stack added (IR, plan, compiler, runtime, executor, web, host, native, core).
- [x] Web static dist generation and runtime-assisted local preview are present.
- [x] Native examples for `native`, `native_pixels`, and `native_wgpu` are in place.
- [x] Shared counter helper and counter/web counter flow are consolidated.
- [x] Event coverage for pointer/keyboard/focus/submit paths in compiler/runtime.
- [x] Static hosting contract documented (`index.html`, `manifest.json`, `alyx-manifest.json`, `app.wasm`, `alyx-loader.js`).
- [x] `compile` no longer requires `Msg: Send` (review hardening).
- [x] Regression test added for non-`Send` compile message types.
- [x] CLI smoke now exercises `/__alyx_event` endpoint with click and keyboard POST payloads during CI.
- [x] Full browser automation matrix (Playwright) for click and keyboard interaction in chromium/firefox/webkit.
- [x] Native renderer parity hardening baseline (`NativeSceneExport` coverage) across adapter examples.
- [x] Native example smoke/build coverage now validates runtime-path parity under `winit` / `pixels` / `wgpu` feature gates.
- [x] Deep compatibility matrix for historical API patterns captured in test artifacts and docs.
- [x] Runtime event endpoint routing now normalizes request paths before dispatch, including query-string variants.

## Evidence used in this review pass
- Core implementation evidence: `crates/alyx-*`
- Review-hardening evidence:
  - `crates/alyx-compiler/src/layout.rs`
  - `crates/alyx-compiler/tests/compile.rs`
- `crates/alyx-native/src/lib.rs`
- `crates/alyx-widgets/tests/compatibility_api.rs`
- PR-facing docs evidence:
  - `README.md`
  - `docs/implementation-notes.md`
  - `docs/pr-summary.md`
  - `docs/web-hosting.md`
- `docs/compatibility-matrix.md`
- `docs/native-renderers.md`

## Notes
- No high-risk gaps remain for PR #4 scope.
