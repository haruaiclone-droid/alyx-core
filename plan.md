# Alyx PR #4 Completion Plan (Review Remediation)

## Rules
- Mark `[x]` only after implementation and verification are complete.
- Keep unfinished items as `[ ]` and include reasons.
- Preserve backward compatibility and keep changes minimal.
- PR-level residual items are listed explicitly at the end.

## Tasks

### 1) Read diff and core docs baseline
- Status: [x]
- Files:
  - `Cargo.toml`
  - `README.md`
  - `docs/alyx-overview.md`
  - `.github/workflows/ci.yml`
  - `examples/**/*`
  - `crates/**/*`
  - `crates/**/tests/**/*`
- Completion criteria:
  - Base branch (`origin/main`) and PR branch (`HEAD`) diff is reviewed.
  - No obvious out-of-scope rewrite is left untracked.
- Verification:
  - `git diff --stat origin/main..HEAD`
  - `git diff origin/main..HEAD -- README.md docs/alyx-overview.md .github/workflows/ci.yml`

### 2) Fix CLI command parsing for host preview invocation
- Status: [x]
- Files:
  - `crates/alyx-cli/src/main.rs`
- Completion criteria:
  - `alyx serve dist` is parsed as port `3000` and `dist` directory.
  - Existing `alyx serve <port> [dir]` and `alyx serve` remain supported.
  - Parser coverage test is added.
- Verification:
  - `git diff origin/main..HEAD -- crates/alyx-cli/src/main.rs`
  - New test `parse_serve_dir_first` is present.

### 3) Expose `alyx` binary name while keeping `alyx-cli` package continuity
- Status: [x]
- Files:
  - `crates/alyx-cli/Cargo.toml`
  - `crates/alyx-cli/src/main.rs`
- Completion criteria:
  - `alyx` binary target exists.
  - Existing package-based invocation still works (`alyx-cli` package).
  - User-facing help text is aligned.
- Verification:
  - `git diff origin/main..HEAD -- crates/alyx-cli/Cargo.toml crates/alyx-cli/src/main.rs`

### 4) Add `alyx-manifest.json` output while preserving `manifest.json`
- Status: [x]
- Files:
  - `crates/alyx-host/src/lib.rs`
  - `crates/alyx-core/tests/phase_integration.rs`
- Completion criteria:
  - Host static build writes both `manifest.json` and `alyx-manifest.json`.
  - Integration test validates both artifacts.
- Verification:
  - `git diff origin/main..HEAD -- crates/alyx-host/src/lib.rs crates/alyx-core/tests/phase_integration.rs`

### 5) Align docs/help text for CLI and manifest expectations
- Status: [x]
- Files:
  - `crates/alyx-cli/src/main.rs`
  - `docs/web-hosting.md`
  - `README.md`
- Completion criteria:
  - Help output uses `alyx` name.
  - Hosting docs mention `alyx-manifest.json` with legacy manifest compatibility.
- Verification:
  - `git diff origin/main..HEAD -- README.md docs/web-hosting.md crates/alyx-cli/src/main.rs`

### 6) PR readiness reporting
- Status: [ ]
- Files:
  - `docs/pr-summary.md`
- Completion criteria:
  - Remaining follow-up scope is visible in PR summary and final plan.
- Verification:
  - Deferred to next PR follow-up pass.

## Remaining items for full completion image (beyond this compatibility pass)

- [ ] Web renderer/backend crate split (canvas/dom/wgpu) is still a roadmap item; current state remains static export + JS bridge.
- [ ] Full feature parity for native adapters is partial (winit/pixels/wgpu examples exist, but crate boundaries are not yet split as idealized in the roadmap).
- [ ] CLI deployment UX (`alyx build`, `alyx dev`, project scaffolding) is not in-scope for this PR.
- [ ] Browser accessibility overlay and input IME pipelines are not in this pass.
