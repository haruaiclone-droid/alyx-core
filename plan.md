# Alyx PR #4 Completion Plan (Review Remediation)

## Rules
- Mark `[x]` only after implementation and verification are complete.
- Keep unfinished items as `[ ]` and include why they remain unfinished.
- Preserve backward compatibility and keep changes minimal.
- PR-level residual items must be listed explicitly at the end of this file.

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
- Status: [ ]
- Files:
  - `crates/alyx-cli/src/main.rs`
- Completion criteria:
  - `alyx serve dist` is parsed as port `3000` + output `dist`.
  - Existing `alyx serve <port> <dir>` and `alyx serve` continue to work.
  - Unit test covers directory-first parsing.
- Verification:
  - `cargo test -p alyx-cli`

### 3) Expose `alyx` binary name without dropping `alyx-cli` compatibility
- Status: [ ]
- Files:
  - `crates/alyx-cli/Cargo.toml`
  - `crates/alyx-cli/src/main.rs`
- Completion criteria:
  - Binary entrypoint is `alyx` for `cargo run`/packaging UX.
  - Existing `--help` and existing invocation paths still compile and run.
  - Help text no longer advertises only `alyx-cli`.
- Verification:
  - `cargo run --package alyx-cli -- --help`

### 4) Add `alyx-manifest.json` output while preserving `manifest.json`
- Status: [ ]
- Files:
  - `crates/alyx-host/src/lib.rs`
  - `crates/alyx-core/tests/phase_integration.rs`
- Completion criteria:
  - Host static build writes both files:
    - `manifest.json` (existing behavior retained)
    - `alyx-manifest.json` (target-compatible behavior added)
  - Integration test checks the expected manifest artifact exists.
- Verification:
  - `cargo test -p alyx-core --test phase_integration`

### 5) Align docs/help for CLI/manifest expectations
- Status: [ ]
- Files:
  - `crates/alyx-cli/src/main.rs`
  - `docs/web-hosting.md`
  - `README.md`
  - `docs/implementation-notes.md`
- Completion criteria:
  - Command help text uses `alyx` naming where user-facing.
  - Deployment docs include `alyx-manifest.json` as target output name without removing `manifest.json` compatibility.
- Verification:
  - Manual doc review against the patched behavior (`git diff origin/main..HEAD`).

### 6) PR readiness reporting
- Status: [ ]
- Files:
  - `docs/pr-summary.md`
  - PR body (if available)
- Completion criteria:
  - Remaining scope from this PR remains clearly listed (not claimed done).
- Verification:
  - Manual check of PR description + plan file.

## Remaining items for full completion image (beyond this minimal compatibility pass)

- [ ] Web renderer/backend crate split (canvas/dom/wgpu) is still a roadmap item; current state remains static export + JS bridge.
- [ ] Full feature parity for native adapters is partial (winit/pixels/wgpu examples exist, but crate boundaries are not yet split as idealized in the complete roadmap).
- [ ] CLI deployment UX (`alyx build`, `alyx dev`, project scaffolding) is not in-scope for this PR.
- [ ] Browser accessibility overlay and input IME pipelines are not in this pass.
