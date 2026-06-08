use alyx_plan::EventType;
use alyx_plan::RenderingPlan;
use alyx_plan::RpNode;
#[cfg(feature = "winit-backend")]
use std::io;
#[cfg(feature = "winit-backend")]
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(feature = "winit-backend")]
use std::sync::{Arc, mpsc as channel};
#[cfg(feature = "winit-backend")]
use std::thread::{self, JoinHandle};

/// Runtime-facing drawing backend contract.
pub trait NativeRenderer {
    fn init(&mut self);
    fn begin_frame(&mut self);
    fn end_frame(&mut self);
}

/// Native event source contract for runtime dispatch.
pub trait NativeEventLoop {
    fn poll(&mut self) -> Option<NativeEvent>;
}

/// Normalized events that can be produced by native adapters.
#[derive(Clone, Debug)]
pub enum NativeEvent {
    PointerDown {
        x: f32,
        y: f32,
    },
    PointerUp {
        x: f32,
        y: f32,
    },
    PointerMove {
        x: f32,
        y: f32,
    },
    Scroll {
        x: f32,
        y: f32,
        delta_x: f32,
        delta_y: f32,
    },
    Keyboard(char),
    KeyboardDown(String),
    KeyboardUp(String),
    WindowFocused,
    WindowBlurred,
    Focus(u64, u64),
    Blur(u64, u64),
    Exit,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct NativeSceneExport {
    pub total_nodes: usize,
    pub text_nodes: usize,
    pub image_nodes: usize,
}

impl NativeSceneExport {
    pub fn from_plan(plan: &RenderingPlan) -> Self {
        let mut export = Self {
            total_nodes: plan.nodes.len(),
            ..Self::default()
        };
        for node in &plan.nodes {
            match node {
                RpNode::Text(_) => export.text_nodes += 1,
                RpNode::Image(_) => export.image_nodes += 1,
            }
        }
        export
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct NativeEventState {
    focused_target: Option<(u64, u64)>,
    cursor: (f32, f32),
}

impl NativeEventState {
    pub fn cursor(&self) -> (f32, f32) {
        self.cursor
    }

    pub fn dispatch_event<A, R>(
        &mut self,
        runtime: &mut alyx_runtime::HeadlessRuntime<A>,
        renderer: &mut R,
        event: NativeEvent,
    ) -> Option<alyx_runtime::Command<A::Message>>
    where
        A: alyx_runtime::App,
        A::Message: Send,
        R: alyx_executor::RenderingPlanExecutor,
    {
        match event {
            NativeEvent::PointerDown { x, y } => {
                self.cursor = (x, y);
                let result = runtime.dispatch_pointer_down(x, y, renderer);
                if self.focused_target.is_none() {
                    self.focused_target = runtime
                        .target_for_event_at(x, y, EventType::KeyDown)
                        .or_else(|| runtime.target_for_event_at(x, y, EventType::Focus));
                }
                result
            }
            NativeEvent::PointerUp { x, y } => {
                self.cursor = (x, y);
                let result = runtime.dispatch_pointer_up(x, y, renderer);
                if self.focused_target.is_none() {
                    self.focused_target = runtime
                        .target_for_event_at(x, y, EventType::KeyDown)
                        .or_else(|| runtime.target_for_event_at(x, y, EventType::Focus));
                }
                result
            }
            NativeEvent::PointerMove { x, y } => {
                self.cursor = (x, y);
                runtime.dispatch_pointer_move(x, y, renderer)
            }
            NativeEvent::Scroll { x, y, .. } => runtime.dispatch_scroll(x, y, renderer),
            NativeEvent::Keyboard(ch) => {
                let key = ch.to_string();
                match self.focused_target {
                    Some((node, element)) => runtime.dispatch_keydown_by_ids_with_key(
                        node,
                        element,
                        Some(key.as_str()),
                        renderer,
                    ),
                    None => {
                        runtime.dispatch_keydown_with_key(0.0, 0.0, Some(key.as_str()), renderer)
                    }
                }
            }
            NativeEvent::KeyboardDown(key) => match self.focused_target {
                Some((node, element)) => runtime.dispatch_keydown_by_ids_with_key(
                    node,
                    element,
                    Some(key.as_str()),
                    renderer,
                ),
                None => runtime.dispatch_keydown_with_key(0.0, 0.0, Some(key.as_str()), renderer),
            },
            NativeEvent::KeyboardUp(_key) => match self.focused_target {
                Some((node, element)) => runtime.dispatch_keyup_by_ids(node, element, renderer),
                None => runtime.dispatch_keyup(0.0, 0.0, renderer),
            },
            NativeEvent::Focus(node, element) => {
                self.focused_target = Some((node, element));
                runtime.dispatch_focus_by_ids(node, element, renderer)
            }
            NativeEvent::Blur(node, element) => {
                if self.focused_target == Some((node, element)) {
                    self.focused_target = None;
                }
                runtime.dispatch_blur_by_ids(node, element, renderer)
            }
            NativeEvent::WindowFocused => None,
            NativeEvent::WindowBlurred => {
                self.focused_target = None;
                None
            }
            NativeEvent::Exit => Some(alyx_runtime::Command::RequestExit),
        }
    }
}

#[derive(Default)]
pub struct SkeletonAdapter {
    initialized: bool,
}

impl NativeRenderer for SkeletonAdapter {
    fn init(&mut self) {
        self.initialized = true;
    }

    fn begin_frame(&mut self) {}

    fn end_frame(&mut self) {}
}

pub struct DummyEventLoop;

impl NativeEventLoop for DummyEventLoop {
    fn poll(&mut self) -> Option<NativeEvent> {
        None
    }
}

#[cfg(feature = "winit-backend")]
pub struct WinitEventLoop {
    events: channel::Receiver<NativeEvent>,
    running: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

#[cfg(feature = "winit-backend")]
impl WinitEventLoop {
    pub fn new(title: impl Into<String>) -> io::Result<Self> {
        Self::with_size(title, 1024, 768)
    }

    pub fn with_size(title: impl Into<String>, width: u32, height: u32) -> io::Result<Self> {
        let (tx, rx) = channel::channel();
        let running = Arc::new(AtomicBool::new(true));
        let running_for_thread = Arc::clone(&running);
        let title = title.into();
        let handle = thread::spawn(move || {
            use winit::dpi::PhysicalSize;
            use winit::event::{ElementState, Event, MouseButton, MouseScrollDelta, WindowEvent};
            use winit::event_loop::EventLoop;
            use winit::keyboard::PhysicalKey;
            use winit::window::WindowBuilder;

            let event_loop = match EventLoop::new() {
                Ok(loop_) => loop_,
                Err(err) => {
                    eprintln!("winit event loop creation failed: {err}");
                    return;
                }
            };

            let _window = match WindowBuilder::new()
                .with_title(title)
                .with_inner_size(PhysicalSize::new(width, height))
                .build(&event_loop)
            {
                Ok(window) => window,
                Err(err) => {
                    eprintln!("winit window creation failed: {err}");
                    return;
                }
            };

            let mut cursor = (0.0f32, 0.0f32);
            let _ = event_loop.run(move |event, event_loop_target| {
                if !running_for_thread.load(Ordering::SeqCst) {
                    event_loop_target.exit();
                    return;
                }

                if let Event::WindowEvent {
                    event: window_event,
                    ..
                } = event
                {
                    match window_event {
                        WindowEvent::CloseRequested => {
                            let _ = tx.send(NativeEvent::Exit);
                            event_loop_target.exit();
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            cursor = (position.x as f32, position.y as f32);
                            let _ = tx.send(NativeEvent::PointerMove {
                                x: cursor.0,
                                y: cursor.1,
                            });
                        }
                        WindowEvent::MouseInput {
                            state: ElementState::Pressed,
                            button: MouseButton::Left,
                            ..
                        } => {
                            let _ = tx.send(NativeEvent::PointerDown {
                                x: cursor.0,
                                y: cursor.1,
                            });
                        }
                        WindowEvent::MouseInput {
                            state: ElementState::Released,
                            button: MouseButton::Left,
                            ..
                        } => {
                            let _ = tx.send(NativeEvent::PointerUp {
                                x: cursor.0,
                                y: cursor.1,
                            });
                        }
                        WindowEvent::MouseWheel { delta, .. } => {
                            let (delta_x, delta_y) = match delta {
                                MouseScrollDelta::LineDelta(x, y) => (x, y),
                                MouseScrollDelta::PixelDelta(offset) => {
                                    (offset.x as f32, offset.y as f32)
                                }
                            };
                            let _ = tx.send(NativeEvent::Scroll {
                                x: cursor.0,
                                y: cursor.1,
                                delta_x,
                                delta_y,
                            });
                        }
                        WindowEvent::KeyboardInput { event, .. } => {
                            let key = match event.logical_key.to_text() {
                                Some(text) => text.to_string(),
                                None => match event.physical_key {
                                    PhysicalKey::Code(code) => format!("{:?}", code),
                                    PhysicalKey::Unidentified(_) => "Unidentified".to_string(),
                                },
                            };

                            match event.state {
                                ElementState::Pressed => {
                                    if let Some(text) = event.text.as_ref()
                                        && let Some(ch) = text.chars().next()
                                    {
                                        let _ = tx.send(NativeEvent::Keyboard(ch));
                                    }
                                    let _ = tx.send(NativeEvent::KeyboardDown(key.clone()));
                                }
                                ElementState::Released => {
                                    let _ = tx.send(NativeEvent::KeyboardUp(key.clone()));
                                }
                            }
                        }
                        WindowEvent::Focused(true) => {
                            let _ = tx.send(NativeEvent::WindowFocused);
                        }
                        WindowEvent::Focused(false) => {
                            let _ = tx.send(NativeEvent::WindowBlurred);
                        }
                        _ => {}
                    }
                }
            });
        });

        Ok(Self {
            events: rx,
            running,
            thread: Some(handle),
        })
    }
}

#[cfg(feature = "winit-backend")]
impl Drop for WinitEventLoop {
    fn drop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.thread.take() {
            let _ = handle.join();
        }
    }
}

#[cfg(feature = "winit-backend")]
impl NativeEventLoop for WinitEventLoop {
    fn poll(&mut self) -> Option<NativeEvent> {
        self.events.try_recv().ok()
    }
}

pub fn pump_native_events<A, R>(
    runtime: &mut alyx_runtime::HeadlessRuntime<A>,
    renderer: &mut R,
    event_loop: &mut dyn NativeEventLoop,
) -> Option<alyx_runtime::Command<A::Message>>
where
    A: alyx_runtime::App,
    A::Message: Send,
    R: alyx_executor::RenderingPlanExecutor,
{
    let mut event_state = NativeEventState::default();
    let mut last_result = None;
    while let Some(event) = event_loop.poll() {
        let result = event_state.dispatch_event(runtime, renderer, event);
        if let Some(alyx_runtime::Command::RequestExit) = &result {
            return Some(alyx_runtime::Command::RequestExit);
        }
        if result.is_some() {
            last_result = result;
        }
    }
    last_result
}

#[cfg(test)]
mod tests {
    use super::*;
    use alyx_executor::MemoryRenderer;
    use alyx_ir::{
        Align, Color, Container, FlexDirection, FlexLayout, Font, HitArea, IrNode, Justify, Layout,
        Padding, Size, Text, TextStyle,
    };
    use alyx_plan::EventType;
    use alyx_runtime::{App, Command, HeadlessRuntime};
    use std::collections::VecDeque;

    #[derive(Clone)]
    enum Msg {
        Pulse,
    }

    struct TestApp;

    impl App for TestApp {
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

        fn view(&self, _state: &Self::State) -> IrNode<Self::Message> {
            let child = IrNode::Text(Text {
                content: "native".to_string(),
                style: TextStyle {
                    font: Font {
                        family: "mono".to_string(),
                    },
                    size: 10.0,
                    color: Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    },
                },
                size: Size {
                    width: 90.0,
                    height: 20.0,
                },
            });
            let hit_area = HitArea::new(
                Layout::Flex(FlexLayout::row()),
                child,
                Some(Msg::Pulse),
                Some(Msg::Pulse),
            )
            .pointer_down(Msg::Pulse)
            .pointer_up(Msg::Pulse)
            .key_down(Msg::Pulse)
            .focus(Msg::Pulse)
            .submit(Msg::Pulse);
            IrNode::Container(Container {
                children: vec![IrNode::HitArea(hit_area)],
                layout: Layout::Flex(FlexLayout {
                    direction: FlexDirection::Column,
                    gap: 4.0,
                    padding: Padding::default(),
                    align: Align::Start,
                    justify: Justify::Start,
                }),
            })
        }
    }

    struct SequenceEventLoop {
        queue: VecDeque<NativeEvent>,
    }

    impl SequenceEventLoop {
        fn new(events: Vec<NativeEvent>) -> Self {
            Self {
                queue: events.into_iter().collect(),
            }
        }
    }

    impl NativeEventLoop for SequenceEventLoop {
        fn poll(&mut self) -> Option<NativeEvent> {
            self.queue.pop_front()
        }
    }

    #[test]
    fn pump_native_events_dispatches_runtime_updates_and_exit() {
        let mut runtime = HeadlessRuntime::new(
            TestApp,
            Size {
                width: 140.0,
                height: 80.0,
            },
        );
        let mut renderer = MemoryRenderer::default();
        let mut events = SequenceEventLoop::new(vec![
            NativeEvent::PointerDown { x: 10.0, y: 10.0 },
            NativeEvent::PointerUp { x: 10.0, y: 10.0 },
            NativeEvent::Keyboard('a'),
            NativeEvent::KeyboardDown("A".to_string()),
            NativeEvent::Exit,
        ]);

        runtime.step(&mut renderer);
        let command = pump_native_events(&mut runtime, &mut renderer, &mut events);

        assert!(matches!(command, Some(Command::RequestExit)));
        assert_eq!(*runtime.state().expect("runtime state"), 4);
        assert!(runtime.metrics().is_some());
    }

    #[test]
    fn pump_native_events_supports_focus_and_blur_events() {
        let mut runtime = HeadlessRuntime::new(
            TestApp,
            Size {
                width: 140.0,
                height: 80.0,
            },
        );
        let mut renderer = MemoryRenderer::default();
        let mut events = SequenceEventLoop::new(vec![
            NativeEvent::Focus(0, 0),
            NativeEvent::Blur(0, 0),
            NativeEvent::KeyboardUp("Escape".to_string()),
        ]);

        runtime.step(&mut renderer);
        let command = pump_native_events(&mut runtime, &mut renderer, &mut events);
        assert!(command.is_some());
        assert_eq!(*runtime.state().expect("runtime state"), 1);
        assert!(runtime.has_focus_events());
    }

    #[test]
    fn pump_native_events_uses_focused_target_for_keyboard_events() {
        let mut runtime = HeadlessRuntime::new(
            TestApp,
            Size {
                width: 140.0,
                height: 80.0,
            },
        );
        let mut renderer = MemoryRenderer::default();
        let output = runtime.compile_frame().expect("compiled");
        let key_target = output
            .ep
            .hit_areas
            .iter()
            .find(|area| area.event_type == EventType::KeyDown)
            .expect("keydown target");

        let events = vec![
            NativeEvent::Focus(key_target.node_id.0, key_target.element_id.0),
            NativeEvent::Keyboard('k'),
            NativeEvent::KeyboardUp("k".to_string()),
        ];
        let mut event_loop = SequenceEventLoop::new(events);

        runtime.step(&mut renderer);
        let _ = pump_native_events(&mut runtime, &mut renderer, &mut event_loop);
        assert_eq!(*runtime.state().expect("runtime state"), 2);
    }

    #[test]
    fn pump_native_events_sets_focus_from_pointer_events() {
        let mut runtime = HeadlessRuntime::new(
            TestApp,
            Size {
                width: 140.0,
                height: 80.0,
            },
        );
        let mut renderer = MemoryRenderer::default();
        let mut events = SequenceEventLoop::new(vec![
            NativeEvent::PointerDown { x: 10.0, y: 10.0 },
            NativeEvent::Keyboard('k'),
        ]);

        runtime.step(&mut renderer);
        let _ = pump_native_events(&mut runtime, &mut renderer, &mut events);
        assert_eq!(*runtime.state().expect("runtime state"), 2);
    }
}
