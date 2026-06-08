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

## Deferred / incomplete
- [ ] Browser automation matrix (non-deterministic interaction assertions not in CI).
- [ ] Native renderer parity and performance hardening.
- [ ] CI reliability around long-running preview smoke and flaky environment windows.
- [ ] Deep compatibility matrix for all historical consumer message patterns.

## Evidence used in this review pass
- Core implementation evidence: `crates/alyx-*`
- Review-hardening evidence:
  - `crates/alyx-compiler/src/layout.rs`
  - `crates/alyx-compiler/tests/compile.rs`
- PR-facing docs evidence:
  - `README.md`
  - `docs/implementation-notes.md`
  - `docs/pr-summary.md`
  - `docs/web-hosting.md`

## Notes
- This checklist is intentionally conservative; tasks left `[ ]` are explicitly deferred, not complete.
