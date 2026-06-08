#[cfg(feature = "pixels-backend")]
use alyx_executor::RenderingPlanExecutor;
#[cfg(feature = "pixels-backend")]
use alyx_ir::Size;
#[cfg(feature = "pixels-backend")]
use alyx_native::{NativeEvent, NativeEventState, NativeSceneExport};
#[cfg(feature = "pixels-backend")]
use alyx_plan::{RenderingPlan, RpNode};
#[cfg(feature = "pixels-backend")]
use alyx_runtime::{App, Command, HeadlessRuntime};
#[cfg(feature = "pixels-backend")]
use alyx_widgets::{ButtonWidget, ContainerWidget, IntoIr, TextWidget, Widget};
#[cfg(feature = "pixels-backend")]
use pixels::{Error, Pixels, SurfaceTexture};
#[cfg(feature = "pixels-backend")]
use winit::dpi::PhysicalSize;
#[cfg(feature = "pixels-backend")]
use winit::event::{ElementState, Event, MouseButton, MouseScrollDelta, WindowEvent};
#[cfg(feature = "pixels-backend")]
use winit::event_loop::EventLoop;
#[cfg(feature = "pixels-backend")]
use winit::keyboard::PhysicalKey;
#[cfg(feature = "pixels-backend")]
use winit::window::WindowBuilder;

#[cfg(feature = "pixels-backend")]
#[derive(Clone, Debug)]
enum Msg {
    Pulse,
    Scroll,
}

#[cfg(feature = "pixels-backend")]
struct NativePixelsApp;

#[cfg(feature = "pixels-backend")]
impl App for NativePixelsApp {
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
            Msg::Scroll => {
                *state = state.saturating_add(1);
                vec![Command::None]
            }
        }
    }

    fn view(&self, state: &Self::State) -> alyx_ir::IrNode<Self::Message> {
        Widget::Container(
            ContainerWidget::column(vec![
                Widget::Text(TextWidget::new(format!("native pixels: {state}")).size(220.0, 24.0)),
                Widget::Button(ButtonWidget::text("pulse", Msg::Pulse)),
                Widget::Button(ButtonWidget::text("scroll", Msg::Scroll)),
            ])
            .gap(8.0)
            .with_padding(12.0, 12.0, 12.0, 12.0),
        )
        .into_ir()
    }
}

#[cfg(feature = "pixels-backend")]
struct PixelRenderer<'a> {
    pixels: Pixels<'a>,
    width: u32,
    height: u32,
}

#[cfg(feature = "pixels-backend")]
impl<'a> PixelRenderer<'a> {
    fn new(width: u32, height: u32, window: &'a winit::window::Window) -> Result<Self, Error> {
        let surface_texture = SurfaceTexture::new(width, height, window);
        let pixels = Pixels::new(width, height, surface_texture)?;
        Ok(Self {
            pixels,
            width,
            height,
        })
    }

    fn resize(&mut self, width: u32, height: u32) -> Result<(), Error> {
        self.pixels.resize_surface(width, height)?;
        self.pixels.resize_buffer(width, height)?;
        self.width = width;
        self.height = height;
        Ok(())
    }

    fn render_plan(&mut self, plan: &RenderingPlan) {
        let scene = NativeSceneExport::from_plan(plan);
        let accent = ((scene.total_nodes % 180) as u8).max(1);
        let mut background = [22u8, 22u8, 22u8, 0xFF];
        background[1] = background[1].saturating_add(accent);
        background[2] = background[2].saturating_add(accent.saturating_mul(3));
        {
            let frame = self.pixels.frame_mut();
            for pixel in frame.chunks_exact_mut(4) {
                pixel.copy_from_slice(&background);
            }
        }

        {
            let frame = self.pixels.frame_mut();
            let width = self.width;
            let height = self.height;
            for node in &plan.nodes {
                match node {
                    RpNode::Text(text) => Self::fill_rect(
                        frame,
                        text.x,
                        text.y,
                        text.width,
                        text.height,
                        [0x5A, 0xA0, 0xFF, 0xFF],
                        (width, height),
                    ),
                    RpNode::Image(image) => Self::fill_rect(
                        frame,
                        image.x,
                        image.y,
                        image.width,
                        image.height,
                        [0x7B, 0xD2, 0x7A, 0xFF],
                        (width, height),
                    ),
                }
            }
        }

        let _ = self.pixels.render();
    }

    fn fill_rect(
        frame: &mut [u8],
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: [u8; 4],
        viewport_size: (u32, u32),
    ) {
        let left = x.max(0.0).floor() as i32;
        let top = y.max(0.0).floor() as i32;
        let right = (x + width).max(0.0).ceil() as i32;
        let bottom = (y + height).max(0.0).ceil() as i32;

        if left >= right || top >= bottom {
            return;
        }

        let (viewport_width, viewport_height) = viewport_size;
        let viewport_w = viewport_width as i32;
        let viewport_h = viewport_height as i32;

        let x0 = left.max(0).min(viewport_w);
        let y0 = top.max(0).min(viewport_h);
        let x1 = right.max(0).min(viewport_w);
        let y1 = bottom.max(0).min(viewport_h);

        for py in y0..y1 {
            for px in x0..x1 {
                let base = ((py as usize * viewport_w as usize) + px as usize) * 4;
                if base + 3 < frame.len() {
                    frame[base..base + 4].copy_from_slice(&color);
                }
            }
        }
    }
}

#[cfg(feature = "pixels-backend")]
impl<'a> RenderingPlanExecutor for PixelRenderer<'a> {
    fn execute(&mut self, plan: &RenderingPlan) {
        self.render_plan(plan);
    }
}

#[cfg(feature = "pixels-backend")]
fn main() {
    let event_loop = match EventLoop::new() {
        Ok(event_loop) => event_loop,
        Err(err) => {
            eprintln!("winit init failed: {err}");
            return;
        }
    };

    let window = match WindowBuilder::new()
        .with_title("Alyx Native Pixels")
        .with_inner_size(PhysicalSize::new(420, 180))
        .build(&event_loop)
    {
        Ok(window) => window,
        Err(err) => {
            eprintln!("window init failed: {err}");
            return;
        }
    };

    let size = window.inner_size();
    let mut runtime = HeadlessRuntime::new(
        NativePixelsApp,
        Size {
            width: size.width as f32,
            height: size.height as f32,
        },
    );

    let mut renderer = match PixelRenderer::new(size.width, size.height, &window) {
        Ok(renderer) => renderer,
        Err(err) => {
            eprintln!("pixels init failed: {err}");
            return;
        }
    };

    let mut input_state = NativeEventState::default();
    let mut should_exit = false;

    let _ = event_loop.run(move |event, target| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    if let Some(Command::RequestExit) =
                        input_state.dispatch_event(&mut runtime, &mut renderer, NativeEvent::Exit)
                    {
                        should_exit = true;
                    }
                }
                WindowEvent::Resized(size) => {
                    let _ = renderer.resize(size.width, size.height);
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let event = NativeEvent::PointerMove {
                        x: position.x as f32,
                        y: position.y as f32,
                    };
                    if let Some(Command::RequestExit) =
                        input_state.dispatch_event(&mut runtime, &mut renderer, event)
                    {
                        should_exit = true;
                    }
                }
                WindowEvent::MouseInput {
                    state: ElementState::Pressed,
                    button: MouseButton::Left,
                    ..
                } => {
                    let (cursor_x, cursor_y) = input_state.cursor();
                    let command = input_state.dispatch_event(
                        &mut runtime,
                        &mut renderer,
                        NativeEvent::PointerDown {
                            x: cursor_x,
                            y: cursor_y,
                        },
                    );
                    if let Some(Command::RequestExit) = command {
                        should_exit = true;
                    }
                }
                WindowEvent::MouseInput {
                    state: ElementState::Released,
                    button: MouseButton::Left,
                    ..
                } => {
                    let (cursor_x, cursor_y) = input_state.cursor();
                    let command = input_state.dispatch_event(
                        &mut runtime,
                        &mut renderer,
                        NativeEvent::PointerUp {
                            x: cursor_x,
                            y: cursor_y,
                        },
                    );
                    if let Some(Command::RequestExit) = command {
                        should_exit = true;
                    }
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    let (delta_x, delta_y) = match delta {
                        MouseScrollDelta::LineDelta(x, y) => (x, y),
                        MouseScrollDelta::PixelDelta(offset) => (offset.x as f32, offset.y as f32),
                    };
                    let (cursor_x, cursor_y) = input_state.cursor();
                    let command = input_state.dispatch_event(
                        &mut runtime,
                        &mut renderer,
                        NativeEvent::Scroll {
                            x: cursor_x,
                            y: cursor_y,
                            delta_x,
                            delta_y,
                        },
                    );
                    if let Some(Command::RequestExit) = command {
                        should_exit = true;
                    }
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    let key = match event.logical_key.to_text() {
                        Some(text) => text.to_string(),
                        None => match event.physical_key {
                            PhysicalKey::Code(code) => format!("{:?}", code),
                            PhysicalKey::Unidentified(_) => "Unidentified".to_string(),
                        },
                    };

                    if let Some(text) = event.text.as_ref()
                        && let Some(ch) = text.chars().next()
                        && let Some(Command::RequestExit) = input_state.dispatch_event(
                            &mut runtime,
                            &mut renderer,
                            NativeEvent::Keyboard(ch),
                        )
                    {
                        should_exit = true;
                    }

                    let keyboard_event = match event.state {
                        ElementState::Pressed => NativeEvent::KeyboardDown(key),
                        ElementState::Released => NativeEvent::KeyboardUp(key),
                    };
                    if let Some(Command::RequestExit) =
                        input_state.dispatch_event(&mut runtime, &mut renderer, keyboard_event)
                    {
                        should_exit = true;
                    }
                }
                WindowEvent::Focused(true) => {
                    if let Some(Command::RequestExit) = input_state.dispatch_event(
                        &mut runtime,
                        &mut renderer,
                        NativeEvent::WindowFocused,
                    ) {
                        should_exit = true;
                    }
                }
                WindowEvent::Focused(false) => {
                    if let Some(Command::RequestExit) = input_state.dispatch_event(
                        &mut runtime,
                        &mut renderer,
                        NativeEvent::WindowBlurred,
                    ) {
                        should_exit = true;
                    }
                }
                WindowEvent::RedrawRequested => {
                    runtime.step(&mut renderer);
                }
                _ => {}
            },
            Event::AboutToWait => {
                if should_exit {
                    target.exit();
                    return;
                }
                runtime.step(&mut renderer);
            }
            _ => {}
        }

        if should_exit {
            target.exit();
        }
    });
}

#[cfg(not(feature = "pixels-backend"))]
fn main() {
    eprintln!("Alyx native pixels demo requires crate feature `pixels-backend`.");
    eprintln!(
        "Run with: cargo run --package alyx-native --example native_pixels --features pixels-backend"
    );
}
