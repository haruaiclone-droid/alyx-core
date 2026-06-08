# Alyx Architecture

The repository is organized as independent layers:

- `alyx-ir`: renderer-independent UI tree and data types.
- `alyx-plan`: compiled rendering/event descriptions.
- `alyx-compiler`: translates IR into rendering/event plans.
- `alyx-runtime`: stateful App model and runtime loop.
- `alyx-executor`: generic render/event execution interfaces.
- `alyx-widgets`: ergonomic widget API with `IntoIr` conversion.
- `alyx-web`: web export and event encoding helpers.
- `alyx-host`: static build and preview helpers.
- `alyx-native`: native adapter extension points.
- `alyx-core`: convenience re-export facade.

## Data flow

`Widget -> IR -> Compiler -> Plan -> Runtime -> Executor -> Renderer/Host`

```
Widget API
    -> IntoIr
IR (alyx-ir)
    -> compile()
RP / EP / Handlers (alyx-plan, alyx-compiler)
    -> dispatch/update
Runtime state loop (alyx-runtime)
    -> execute()
Renderer (alyx-executor / alyx-web / alyx-native)
```

## PR scope

- Every requested layer is present at the crate level.
- Concrete platform implementations are intentionally minimal and extension-ready.
- Core pipeline can compile widget trees and emit plans, render, and dispatch events.
