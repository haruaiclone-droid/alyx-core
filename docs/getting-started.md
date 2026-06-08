# Getting Started

## 1) Install dependencies

```bash
cargo build
```

## 2) Build a widget tree

Use widgets and convert to IR with `IntoIr`.

```rust
use alyx_core::{ir::*, runtime::App, widgets::*};

let root = Widget::Container(ContainerWidget {
    children: vec![
        Widget::Text(TextWidget::new("Alyx").size(120.0, 24.0)),
        Widget::Button(ButtonWidget {
            label: TextWidget::new("Save"),
            on_click: None,
        }),
    ],
    layout: alyx_ir::Layout::Flex(alyx_ir::FlexLayout {
        direction: alyx_ir::FlexDirection::Column,
        gap: 8.0,
        padding: alyx_ir::Padding {
            left: 8.0,
            top: 8.0,
            right: 8.0,
            bottom: 8.0,
        },
        align: alyx_ir::Align::Start,
        justify: alyx_ir::Justify::Start,
    }),
}).into_ir();
```

## 3) Compile and render

```rust
let output = alyx_core::compiler::compile(&root, alyx_ir::Size { width: 320.0, height: 120.0 });
println!("nodes: {}", output.rp.nodes.len());
```

## 4) Drive state

Implement `App`, then connect `HeadlessRuntime` + a renderer.

```rust
use alyx_core::runtime::{App, HeadlessRuntime};
use alyx_executor::MemoryRenderer;

let mut runtime = HeadlessRuntime::new(MyApp, alyx_ir::Size { width: 320.0, height: 120.0 });
let mut renderer = MemoryRenderer::default();
runtime.step(&mut renderer);
```

## 5) Ergonomic core API entry points

`alyx-core` also provides compact helpers through `prelude::*`:

- `text("label")`
- `button("label") -> ButtonBuilder`
- `row([..])` and `column([..])`

`run` is also available as a minimal convenience for building a runtime instance:

```rust
use alyx_core::prelude::{self, run, App};

// returns a HeadlessRuntime without manually naming runtime types
let _runtime = run(MyApp, alyx_core::ir::Size { width: 320.0, height: 120.0 });
```

If you prefer explicit control, keep using `HeadlessRuntime::new` directly.
