#[cfg(feature = "winit-backend")]
use alyx_executor::MemoryRenderer;
#[cfg(feature = "winit-backend")]
use alyx_ir::Size;
#[cfg(feature = "winit-backend")]
use alyx_native::{WinitEventLoop, pump_native_events};
#[cfg(feature = "winit-backend")]
use alyx_runtime::{App, Command, HeadlessRuntime};
#[cfg(feature = "winit-backend")]
use alyx_widgets::{button, ContainerWidget, IntoIr, TextWidget, Widget};
#[cfg(feature = "winit-backend")]
use std::time::Duration;

#[cfg(feature = "winit-backend")]
#[derive(Clone, Debug)]
enum Msg {
    Pulse,
}

#[cfg(feature = "winit-backend")]
struct NativeDemoApp;

#[cfg(feature = "winit-backend")]
impl App for NativeDemoApp {
    type Message = Msg;
    type State = u32;

    fn initial_state(&self) -> Self::State {
        0
    }

    fn update(
        &self,
        state: &mut Self::State,
        message: Self::Message,
    ) -> Vec<Command<Self::Message>> {
        match message {
            Msg::Pulse => {
                *state += 1;
                vec![Command::None]
            }
        }
    }

    fn view(&self, state: &Self::State) -> alyx_ir::IrNode<Self::Message> {
        Widget::Container(
            ContainerWidget::column(vec![
                Widget::Text(TextWidget::new(format!("native ticks: {state}")).size(220.0, 24.0)),
                button("pulse").on_click(Msg::Pulse),
            ])
            .gap(10.0)
            .with_padding(12.0, 12.0, 12.0, 12.0),
        )
        .into_ir()
    }
}

#[cfg(feature = "winit-backend")]
fn main() {
    let mut runtime = HeadlessRuntime::new(
        NativeDemoApp,
        Size {
            width: 360.0,
            height: 120.0,
        },
    );
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


