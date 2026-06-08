# Alyx Overview

This repository implements the first complete pass of the "Alyx full concept" as a single workspace:

- Core IR and rendering plan pipeline
- Widget API
- Event dispatch loop
- Runtime orchestration
- Web/native/host extension points

## Project goals

1. Keep the pipeline pure and deterministic: `IR -> Plan`.
2. Keep runtime state updates explicit and serializable.
3. Keep platform integrations adapter-based.
4. Keep API surface small but extensible for future backends.

## Crate responsibilities

- `alyx-ir`: declarative UI description and shared IDs/metadata.
- `alyx-plan`: rendering nodes, event areas, focus order, and navigation plans.
- `alyx-compiler`: compiles IR to plan and handler table.
- `alyx-runtime`: App lifecycle (`initial_state`, `view`, `update`).
- `alyx-executor`: render/event-plan contracts.
- `alyx-widgets`: ergonomic Rust API with conversion to IR.
- `alyx-web`, `alyx-host`, `alyx-native`: platform-facing extensions.
- `alyx-core`: unified exports and developer entrypoint.
