# Widgets

`alyx-widgets` currently provides ergonomic Rust constructors that compile into `alyx_ir::IrNode`.

## Supported widgets

- `TextWidget`
- `ButtonWidget`
- `ImageWidget`
- `ContainerWidget`
- `Row` / `Column`
- `Stack`
- `List`
- `Scroll`
- `TextInput`
- `Checkbox`
- `Link`
- `Spacer`
- `Pane`
- `Custom` (IR passthrough)

## Conversion model

All widget values implement `IntoIr<Msg>` and output `IrNode<Msg>` in a predictable order.

## Example

```rust
let node = button("save").on_click(Msg::Save);
let ir = node.into_ir();
```
