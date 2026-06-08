# IR Reference

`alyx-ir` contains renderer-agnostic primitives:

- `IrNode`: `Container`, `Text`, `Image`, `HitArea`, `Pane`
- Layout: `Layout::Flex` with `direction`, `gap`, `padding`, `align`, `justify`
- Geometry: `Size`, `Rect`, `Padding`
- Source: `ImageSource` (`Path`, `Bytes`, `Url`)
- Identity: `NodeId`, `ElementId`
- Accessibility: `Role`, `AccessibilityMetadata`, `Accessibility`

## Conversion targets

- `widgets` crate converts strongly-typed widget models to `IrNode` through `IntoIr`.
