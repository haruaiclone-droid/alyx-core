#[cfg(feature = "wgpu-backend")]
use alyx_ir::Size;
#[cfg(feature = "wgpu-backend")]
use alyx_native::{NativeEvent, NativeEventState, NativeSceneExport};
#[cfg(feature = "wgpu-backend")]
use alyx_runtime::{App, Command, HeadlessRuntime};
#[cfg(feature = "wgpu-backend")]
use alyx_widgets::{ButtonWidget, ContainerWidget, IntoIr, TextWidget, Widget};
#[cfg(feature = "wgpu-backend")]
use wgpu::{Device, Queue, Surface, SurfaceError};
#[cfg(feature = "wgpu-backend")]
use winit::dpi::PhysicalSize;
#[cfg(feature = "wgpu-backend")]
use winit::event::{ElementState, Event, MouseButton, MouseScrollDelta, WindowEvent};
#[cfg(feature = "wgpu-backend")]
use winit::event_loop::EventLoop;
#[cfg(feature = "wgpu-backend")]
use winit::keyboard::PhysicalKey;
#[cfg(feature = "wgpu-backend")]
use winit::window::WindowBuilder;

#[cfg(feature = "wgpu-backend")]
#[derive(Clone, Debug)]
enum Msg {
    Pulse,
    Scroll,
}

#[cfg(feature = "wgpu-backend")]
struct NativeWgpuApp;

#[cfg(feature = "wgpu-backend")]
impl App for NativeWgpuApp {
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
                Widget::Text(TextWidget::new(format!("native wgpu: {state}")).size(220.0, 24.0)),
                Widget::Button(ButtonWidget::text("pulse", Msg::Pulse)),
                Widget::Button(ButtonWidget::text("scroll", Msg::Scroll)),
            ])
            .gap(8.0)
            .with_padding(12.0, 12.0, 12.0, 12.0),
        )
        .into_ir()
    }
}

#[cfg(feature = "wgpu-backend")]
struct NativeWgpuRenderer<'a> {
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    clear_color: wgpu::Color,
}

#[cfg(feature = "wgpu-backend")]
impl<'a> NativeWgpuRenderer<'a> {
    async fn new(window: &'a winit::window::Window) -> Result<Self, String> {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = instance
            .create_surface(window)
            .map_err(|err| err.to_string())?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| "no compatible adapter found".to_string())?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    ..Default::default()
                },
                None,
            )
            .await
            .map_err(|err| format!("device request failed: {err}"))?;

        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|format| format.is_srgb())
            .or_else(|| caps.formats.first().copied())
            .ok_or_else(|| "no supported surface format".to_string())?;

        let present_mode = caps
            .present_modes
            .iter()
            .copied()
            .find(|mode| *mode == wgpu::PresentMode::Fifo)
            .or_else(|| caps.present_modes.first().copied())
            .ok_or_else(|| "no supported present mode".to_string())?;

        let alpha_mode = caps
            .alpha_modes
            .first()
            .copied()
            .unwrap_or(wgpu::CompositeAlphaMode::Auto);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode,
            alpha_mode,
            view_formats: vec![],
            desired_maximum_frame_latency: 1,
        };
        surface.configure(&device, &config);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size,
            clear_color: wgpu::Color {
                r: 0.14,
                g: 0.14,
                b: 0.14,
                a: 1.0,
            },
        })
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width == 0 || size.height == 0 {
            return;
        }
        self.size = size;
        self.config.width = size.width.max(1);
        self.config.height = size.height.max(1);
        self.surface.configure(&self.device, &self.config);
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let texture_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("alyx-native-wgpu-encoder"),
            });

        {
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("alyx-native-wgpu-pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();
        Ok(())
    }
}

#[cfg(feature = "wgpu-backend")]
impl<'a> alyx_executor::RenderingPlanExecutor for NativeWgpuRenderer<'a> {
    fn execute(&mut self, plan: &alyx_plan::RenderingPlan) {
        let scene = NativeSceneExport::from_plan(plan);
        let nodes = scene.total_nodes as f64;
        let phase = (nodes / 8.0) % 1.0;
        let text_ratio = if scene.total_nodes == 0 {
            0.0
        } else {
            scene.text_nodes as f64 / scene.total_nodes as f64
        };
        self.clear_color = wgpu::Color {
            r: 0.07 + phase * 0.25,
            g: 0.11 + phase * 0.18,
            b: 0.18 + phase * 0.12 + text_ratio * 0.15,
            a: 1.0,
        };
    }
}

#[cfg(feature = "wgpu-backend")]
fn main() {
    let event_loop = match EventLoop::new() {
        Ok(event_loop) => event_loop,
        Err(err) => {
            eprintln!("winit init failed: {err}");
            return;
        }
    };
    let window = match WindowBuilder::new()
        .with_title("Alyx Native WGPU")
        .with_inner_size(PhysicalSize::new(520, 180))
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
        NativeWgpuApp,
        Size {
            width: size.width as f32,
            height: size.height as f32,
        },
    );
    let mut renderer = match pollster::block_on(NativeWgpuRenderer::new(&window)) {
        Ok(renderer) => renderer,
        Err(err) => {
            eprintln!("wgpu init failed: {err}");
            return;
        }
    };
    let mut input_state = NativeEventState::default();
    let mut should_exit = false;

    let _ = event_loop.run(move |event, target| {
        match event {
            Event::WindowEvent {
                event,
                window_id: _,
            } => match event {
                WindowEvent::CloseRequested => {
                    if let Some(Command::RequestExit) =
                        input_state.dispatch_event(&mut runtime, &mut renderer, NativeEvent::Exit)
                    {
                        should_exit = true;
                    }
                }
                WindowEvent::Resized(size) => {
                    renderer.resize(size);
                    runtime.step(&mut renderer);
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
                    if let Err(error) = renderer.render() {
                        match error {
                            SurfaceError::OutOfMemory => {
                                should_exit = true;
                            }
                            SurfaceError::Lost => {
                                renderer.resize(renderer.size);
                            }
                            SurfaceError::Timeout | SurfaceError::Outdated => {}
                        }
                    }
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

#[cfg(not(feature = "wgpu-backend"))]
fn main() {
    eprintln!("Alyx native WGPU example requires crate feature `wgpu-backend`.");
    eprintln!(
        "Run with: cargo run --package alyx-native --example native_wgpu --features wgpu-backend"
    );
}
