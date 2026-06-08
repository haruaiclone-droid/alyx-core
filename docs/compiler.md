# Compiler

`alyx-compiler` owns translation from IR to executable plans:

- allocate stable IDs for every IR node and element
- build `RenderingPlan`
- build `EventPlan` hit targets and focus targets
- collect `HandlerTable` for callback routing

## API

- `compile(root: &IrNode<Msg>, viewport: Size) -> CompilerOutput<Msg>`

## Current behavior

- `Container` is flattened using a flex row/column layout calculator.
- `HitArea` nodes register `click` / `hover` handlers into handler table.
- `Text` and `Image` nodes produce `RpText` and `RpImage` entries with coordinates.
- Accessibility metadata is propagated into `AccessibilityPlanEntry`.
