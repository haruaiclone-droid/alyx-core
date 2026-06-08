# Runtime

The runtime glues state and plans:

- `compile_frame` compiles the current `App::view` into `CompilerOutput`.
- `dispatch` accepts a message, runs `App::update`, resolves nested commands and re-renders if needed.
- `step` renders once with current state.
- `dispatch_event` converts a resolved hit target into a command flow when a handler is available.
- `dispatch_pointer` resolves the topmost hit target from coordinates and triggers dispatch.

`HeadlessRuntime` stores:

- `last_output`: latest `CompilerOutput`
- state and app instances
- simple frame metrics (`ViewMetrics`)

This is intentionally minimal and acts as the single place for `compile -> execute` orchestration in this first implementation pass.
