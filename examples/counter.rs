mod shared_counter;

use alyx_core::ir::Size;
use alyx_core::runtime::HeadlessRuntime;
use alyx_executor::MemoryRenderer;
use shared_counter::{CounterApp, VIEW_HEIGHT, VIEW_WIDTH};

fn main() {
    let mut runtime = HeadlessRuntime::new(
        CounterApp,
        Size {
            width: VIEW_WIDTH,
            height: VIEW_HEIGHT,
        },
    );
    let mut renderer = MemoryRenderer::default();
    runtime.step(&mut renderer);
}
