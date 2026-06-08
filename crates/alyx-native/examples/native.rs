#[cfg(feature = "winit-backend")]
#[path = "../../../examples/shared_counter.rs"]
mod shared_counter;

#[cfg(feature = "winit-backend")]
use shared_counter::{CounterApp, VIEW_HEIGHT, VIEW_WIDTH};
#[cfg(feature = "winit-backend")]
use alyx_executor::MemoryRenderer;
#[cfg(feature = "winit-backend")]
use alyx_ir::Size;
#[cfg(feature = "winit-backend")]
use alyx_native::{WinitEventLoop, pump_native_events};
#[cfg(feature = "winit-backend")]
use alyx_runtime::{Command, HeadlessRuntime};
#[cfg(feature = "winit-backend")]
use std::time::Duration;

#[cfg(feature = "winit-backend")]
fn main() {
    let mut runtime = HeadlessRuntime::new(CounterApp, Size {
        width: VIEW_WIDTH,
        height: VIEW_HEIGHT,
    });
    let mut renderer = MemoryRenderer::default();
    let mut event_loop = WinitEventLoop::new("Alyx Native").expect("winit backend");
    println!("Alyx Native demo running. Close window to exit.");

    loop {
        runtime.step(&mut renderer);
        if let Some(Command::RequestExit) =
            pump_native_events(&mut runtime, &mut renderer, &mut event_loop)
        {
            break;
        }
        std::thread::sleep(Duration::from_millis(16));
    }
}

#[cfg(not(feature = "winit-backend"))]
fn main() {
    eprintln!("Alyx native example requires crate feature `winit-backend`.");
    eprintln!(
        "Run with: cargo run --package alyx-native --example native --features winit-backend"
    );
}


