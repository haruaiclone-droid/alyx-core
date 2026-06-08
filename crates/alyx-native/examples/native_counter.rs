#[cfg(feature = "winit-backend")]
#[path = "../../../examples/shared_counter.rs"]
mod shared_counter;

#[cfg(feature = "winit-backend")]
use std::time::Duration;

#[cfg(feature = "winit-backend")]
use shared_counter::CounterApp;
#[cfg(feature = "winit-backend")]
use alyx_core::runtime::Command;
#[cfg(feature = "winit-backend")]
use alyx_executor::MemoryRenderer;
#[cfg(feature = "winit-backend")]
use alyx_ir::Size;
#[cfg(feature = "winit-backend")]
use alyx_native::{pump_native_events, WinitEventLoop};
#[cfg(feature = "winit-backend")]
use alyx_runtime::HeadlessRuntime;

#[cfg(feature = "winit-backend")]
fn main() {
    let mut runtime = HeadlessRuntime::new(
        CounterApp,
        Size {
            width: shared_counter::VIEW_WIDTH,
            height: shared_counter::VIEW_HEIGHT,
        },
    );
    let mut renderer = MemoryRenderer::default();
    let mut event_loop = WinitEventLoop::new("Alyx Native Counter")
        .expect("winit backend");
    println!("Alyx native counter running. Close the window to exit.");

    loop {
        runtime.step(&mut renderer);
        if let Some(Command::RequestExit) = pump_native_events(&mut runtime, &mut renderer, &mut event_loop)
        {
            break;
        }
        std::thread::sleep(Duration::from_millis(16));
    }
}

#[cfg(not(feature = "winit-backend"))]
fn main() {
    eprintln!("Alyx native counter example requires crate feature `winit-backend`.");
    eprintln!(
        "Run with: cargo run --package alyx-native --example native_counter --features winit-backend"
    );
}
