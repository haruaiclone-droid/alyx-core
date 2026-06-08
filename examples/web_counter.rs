mod shared_counter;

use alyx_core::ir::Size;
use alyx_executor::MemoryRenderer;
use alyx_core::runtime::HeadlessRuntime;
use shared_counter::CounterApp;
use shared_counter::{VIEW_HEIGHT, VIEW_WIDTH};

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
