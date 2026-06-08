# Alyx

![Status](https://img.shields.io/badge/status-in%20progress-yellow)

Alyx is a Rust-first, cross-platform UI concept composed of:

- `alyx-ir`: renderer-independent UI intermediate representation
- `alyx-plan`: rendering and event plan models
- `alyx-compiler`: IR -> plan compiler
- `alyx-runtime`: state + view + update loop
- `alyx-executor`: generic rendering and event plan executors
- `alyx-web`: web export and browser event helpers
- `alyx-host`: local preview / static dist helpers
- `alyx-native`: native adapter abstraction layer
- `alyx-core`: public re-exports
- `alyx-cli`: package for host build/serve smoke flow (binary: `alyx`)

## What is included in this repository

This repository now contains a one-PR implementation pass that adds:

1. Extended IR model (node IDs, element IDs, accessibility metadata)
2. Widget-level API and conversion into IR
3. Compiler output of `RenderingPlan`, `EventPlan`, and `HandlerTable`
4. Basic runtime loop with `HeadlessRuntime`
5. Executor traits and test-oriented renderers
6. Web/static export and hosting helpers
7. Native adapter traits and skeleton extension points
8. Documentation set for architecture and workflows
9. Expanded examples (hello/counter/layout/web_counter/form/image/static_export/nested_layout/accessibility_focus)

## Static web bundle contract

`alyx-host` build artifacts use a fixed file contract for deployment:

- `index.html`
- `manifest.json`
- `alyx-manifest.json`
- `app.wasm`
- `alyx-loader.js`

## Quick start

```rust
use alyx_core::{ir::*, runtime::App, widgets::*, runtime::HeadlessRuntime};
use alyx_executor::MemoryRenderer;

#[derive(Clone)]
enum Msg {
    Increment,
}

struct DemoApp;

impl App for DemoApp {
    type Message = Msg;
    type State = u32;

    fn initial_state(&self) -> Self::State {
        0
    }

    fn update(&self, state: &mut Self::State, message: Self::Message) -> Vec<alyx_core::runtime::Command<Self::Message>> {
        match message {
            Msg::Increment => {
                *state += 1;
                vec![alyx_core::runtime::Command::None]
            }
        }
    }

    fn view(&self, state: &Self::State) -> alyx_core::ir::IrNode<Self::Message> {
        Widget::Container(ContainerWidget {
            children: vec![
                Widget::Text(TextWidget::new(format!("count: {state}")).size(120.0, 24.0)),
                Widget::Button(ButtonWidget {
                    label: TextWidget::new("increment").size(80.0, 24.0),
                    on_click: Some(Msg::Increment),
                }),
            ],
            layout: alyx_ir::Layout::Flex(alyx_ir::FlexLayout {
                direction: alyx_ir::FlexDirection::Column,
                gap: 8.0,
                padding: alyx_ir::Padding {
                    left: 12.0,
                    top: 12.0,
                    right: 12.0,
                    bottom: 12.0,
                },
                align: alyx_ir::Align::Start,
                justify: alyx_ir::Justify::Start,
            }),
        })
        .into_ir()
    }
}

fn main() {
    let mut runtime = HeadlessRuntime::new(DemoApp, alyx_ir::Size { width: 320.0, height: 120.0 });
    let mut renderer = MemoryRenderer::default();
    runtime.step(&mut renderer);
}
```

## Event handling support

- `EventPlan` carries typed hit areas (`Click`, `Hover`, `PointerDown`, `PointerUp`, `PointerMove`, `KeyDown`, `KeyUp`, `Focus`, `Blur`, `Submit`, `NavigateBack`, `NavigateForward`).
- `HeadlessRuntime` includes helper dispatch methods for core input/events and carries navigation events through runtime event payloads.
- `HitArea` supports builder methods for pointer/keyboard/focus/submit handlers.

## Workspace crates

- `alyx-core`
- `alyx-ir`
- `alyx-plan`
- `alyx-compiler`
- `alyx-widgets`
- `alyx-runtime`
- `alyx-executor`
- `alyx-web`
- `alyx-host`
- `alyx-native`
- `alyx-cli`

## Notes

This implementation is a single-PR full-stack pass toward the pasted-text plan and includes most core layers.
Focus is on completing the architecture, API, web host path, and adapter boundaries in one PR.

## Verification commands

The acceptance commands used in this PR scope are:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features`
- `cargo test --workspace --all-features`
- `cargo build --workspace --all-features`
- `cargo doc --workspace --no-deps --all-features`
- `rustup target add wasm32-unknown-unknown`
- `cargo build --target wasm32-unknown-unknown --workspace`
- `cargo run --package alyx-cli -- --help`
- `cargo run --package alyx-cli --bin alyx -- --help`
- `cargo run --package alyx-cli -- build-web dist`
- `cargo check --package alyx-native --example native --features winit-backend`
- `cargo check --package alyx-native --example native_pixels --features pixels-backend`
- `cargo check --package alyx-native --example native_wgpu --features wgpu-backend`

### Verification status

- CI is the source of authoritative verification for the full matrix.
- In this environment we have not re-executed the full command suite during this review-only fix pass.

## Current deferred items

- Browser deployment hardening (non-local preview deployment workflows) remains a follow-up.
- Expanded browser-side interactive smoke coverage beyond static HTTP checks remains a follow-up.
- Additional hosted CI matrix reporting consistency is a follow-up.

### Native demo

- Run the native event loop example with `winit` enabled:
  - `cargo run --package alyx-native --example native --features winit-backend`
- Run the native pixel renderer example:
  - `cargo run --package alyx-native --example native_pixels --features pixels-backend`
- Run the native wgpu renderer example:
  - `cargo run --package alyx-native --example native_wgpu --features wgpu-backend`

`Alyx` is now in a "single-PR comprehensive baseline" state, with follow-up blockers tracked separately in operational notes.

## License

MIT OR Apache-2.0

