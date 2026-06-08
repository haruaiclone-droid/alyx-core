use alyx_compiler::compile;
use alyx_executor::RenderingPlanExecutor;
use alyx_ir::Size;
use alyx_plan::{CompilerOutput, EventType, ResolvedHitArea};
use std::collections::VecDeque;

pub trait App {
    type Message: Clone;
    type State;

    fn initial_state(&self) -> Self::State;
    fn update(
        &self,
        state: &mut Self::State,
        message: Self::Message,
    ) -> Vec<Command<Self::Message>>;
    fn view(&self, state: &Self::State) -> alyx_ir::IrNode<Self::Message>;
}

#[derive(Debug, Default)]
pub struct HeadlessRuntime<A>
where
    A: App,
{
    app: Option<A>,
    state: Option<A::State>,
    viewport: Size,
    pub last_output: Option<CompilerOutput<A::Message>>,
}

#[derive(Clone, Debug)]
pub struct ViewMetrics {
    pub rendered_nodes: usize,
    pub hit_targets: usize,
    pub has_exit_command: bool,
}

pub enum Command<Msg> {
    None,
    Message(Msg),
    RequestExit,
}

impl<A> HeadlessRuntime<A>
where
    A: App,
    A::Message: Send,
{
    pub fn new(app: A, viewport: Size) -> Self {
        let state = app.initial_state();
        Self {
            app: Some(app),
            state: Some(state),
            viewport,
            last_output: None,
        }
    }

    pub fn compile_frame(&mut self) -> Option<CompilerOutput<A::Message>> {
        let app = self.app.as_ref()?;
        let state = self.state.as_ref()?;
        let root = app.view(state);
        let output = compile(&root, self.viewport);
        self.last_output = Some(output.clone());
        Some(output)
    }

    pub fn dispatch<R>(
        &mut self,
        message: A::Message,
        renderer: &mut R,
    ) -> Option<Command<A::Message>>
    where
        R: RenderingPlanExecutor,
    {
        let app = self.app.as_ref()?;
        let state = self.state.as_mut()?;

        let mut queue = VecDeque::new();
        queue.push_back(message);
        let mut should_render = false;
        let mut requested_exit = false;

        while let Some(msg) = queue.pop_front() {
            let new_commands = app.update(state, msg);
            for next in new_commands {
                match next {
                    Command::None => should_render = true,
                    Command::Message(msg) => queue.push_back(msg),
                    Command::RequestExit => {
                        requested_exit = true;
                    }
                }
            }
        }

        if requested_exit {
            return Some(Command::RequestExit);
        }

        if should_render {
            if let Some(output) = self.compile_frame() {
                renderer.execute(&output.rp);
                return Some(Command::None);
            }
            return None;
        }

        Some(Command::None)
    }

    pub fn dispatch_event<R>(
        &mut self,
        resolved: &ResolvedHitArea,
        renderer: &mut R,
    ) -> Option<Command<A::Message>>
    where
        R: RenderingPlanExecutor,
    {
        let handler = {
            let output = self.last_output.as_ref()?;
            output.handlers.get(resolved.handler_id).cloned()
        };
        self.dispatch(handler?, renderer)
    }

    pub fn dispatch_pointer_event_with_ids<R>(
        &mut self,
        x: f32,
        y: f32,
        event_type: EventType,
        target_node: Option<u64>,
        target_element: Option<u64>,
        renderer: &mut R,
    ) -> Option<Command<A::Message>>
    where
        R: RenderingPlanExecutor,
    {
        let hit = self
            .resolve_hit_area(event_type, x, y, target_node, target_element)?
            .clone();
        self.dispatch_event(&hit, renderer)
    }

    fn resolve_hit_area(
        &self,
        event_type: EventType,
        x: f32,
        y: f32,
        target_node: Option<u64>,
        target_element: Option<u64>,
    ) -> Option<&ResolvedHitArea> {
        let output = self.last_output.as_ref()?;
        let same_type_and_with_point = output
            .ep
            .hit_areas
            .iter()
            .filter(|area| area.event_type == event_type)
            .filter(|area| {
                x >= area.rect.x
                    && y >= area.rect.y
                    && x <= area.rect.x + area.rect.width
                    && y <= area.rect.y + area.rect.height
            });

        if let (Some(node), Some(element)) = (target_node, target_element) {
            let by_id = output
                .ep
                .hit_areas
                .iter()
                .filter(|area| {
                    area.event_type == event_type
                        && area.node_id.0 == node
                        && area.element_id.0 == element
                })
                .max_by(|left, right| left.z_index.cmp(&right.z_index));
            if by_id.is_some() {
                return by_id;
            }
            return None;
        }

        same_type_and_with_point.max_by(|left, right| left.z_index.cmp(&right.z_index))
    }

    pub fn dispatch_focus_by_ids<R>(
        &mut self,
        node: u64,
        element: u64,
        renderer: &mut R,
    ) -> Option<Command<A::Message>>
    where
        R: RenderingPlanExecutor,
    {
        self.dispatch_pointer_event_with_ids(
            0.0,
            0.0,
            EventType::Focus,
            Some(node),
            Some(element),
            renderer,
        )
    }

    pub fn dispatch_blur_by_ids<R>(
        &mut self,
        node: u64,
        element: u64,
        renderer: &mut R,
    ) -> Option<Command<A::Message>>
    where
        R: RenderingPlanExecutor,
    {
        self.dispatch_pointer_event_with_ids(
            0.0,
            0.0,
            EventType::Blur,
            Some(node),
            Some(element),
            renderer,
        )
    }

    pub fn dispatch_submit_by_ids<R>(
        &mut self,
        node: u64,
        element: u64,
        renderer: &mut R,
    ) -> Option<Command<A::Message>>
    where
        R: RenderingPlanExecutor,
    {
        self.dispatch_pointer_event_with_ids(
            0.0,
            0.0,
            EventType::Submit,
            Some(node),
            Some(element),
            renderer,
        )
    }

    pub fn dispatch_pointer<R>(
        &mut self,
        x: f32,
        y: f32,
        renderer: &mut R,
    ) -> Option<Command<A::Message>>
    where
        R: RenderingPlanExecutor,
    {
        let hit = self
            .last_output
            .as_ref()?
            .ep
            .hit_areas
            .iter()
            .find(|area| {
                x >= area.rect.x
                    && y >= area.rect.y
                    && x <= area.rect.x + area.rect.width
                    && y <= area.rect.y + area.rect.height
            })?
            .clone();
        self.dispatch_event(&hit, renderer)
    }

    pub fn dispatch_pointer_event<R>(
        &mut self,
        x: f32,
        y: f32,
        event_type: EventType,
        renderer: &mut R,
    ) -> Option<Command<A::Message>>
    where
        R: RenderingPlanExecutor,
    {
        self.dispatch_pointer_event_with_ids(x, y, event_type, None, None, renderer)
    }

    pub fn dispatch_event_by_type<R>(
        &mut self,
        x: f32,
        y: f32,
        event_type: EventType,
        renderer: &mut R,
    ) -> Option<Command<A::Message>>
    where
        R: RenderingPlanExecutor,
    {
        self.dispatch_pointer_event(x, y, event_type, renderer)
    }

    pub fn target_for_event_at(&self, x: f32, y: f32, event_type: EventType) -> Option<(u64, u64)> {
        self.resolve_hit_area(event_type, x, y, None, None)
            .map(|area| (area.node_id.0, area.element_id.0))
    }

    pub fn dispatch_hover<R>(
        &mut self,
        x: f32,
        y: f32,
        renderer: &mut R,
    ) -> Option<Command<A::Message>>
    where
        R: RenderingPlanExecutor,
    {
        self.dispatch_pointer_event(x, y, EventType::Hover, renderer)
    }

    pub fn dispatch_pointer_move<R>(
        &mut self,
        x: f32,
        y: f32,
        renderer: &mut R,
    ) -> Option<Command<A::Message>>
    where
        R: RenderingPlanExecutor,
    {
        self.dispatch_pointer_event(x, y, EventType::PointerMove, renderer)
    }

    pub fn dispatch_pointer_down<R>(
        &mut self,
        x: f32,
        y: f32,
        renderer: &mut R,
    ) -> Option<Command<A::Message>>
    where
        R: RenderingPlanExecutor,
    {
        self.dispatch_pointer_event(x, y, EventType::PointerDown, renderer)
    }

    pub fn dispatch_pointer_up<R>(
        &mut self,
        x: f32,
        y: f32,
        renderer: &mut R,
    ) -> Option<Command<A::Message>>
    where
        R: RenderingPlanExecutor,
    {
        self.dispatch_pointer_event(x, y, EventType::PointerUp, renderer)
    }

    pub fn dispatch_click<R>(
        &mut self,
        x: f32,
        y: f32,
        renderer: &mut R,
    ) -> Option<Command<A::Message>>
    where
        R: RenderingPlanExecutor,
    {
        self.dispatch_event_by_type(x, y, EventType::Click, renderer)
    }

    pub fn dispatch_keydown<R>(
        &mut self,
        x: f32,
        y: f32,
        renderer: &mut R,
    ) -> Option<Command<A::Message>>
    where
        R: RenderingPlanExecutor,
    {
        self.dispatch_keydown_with_key(x, y, None, renderer)
    }

    pub fn dispatch_keydown_with_key<R>(
        &mut self,
        x: f32,
        y: f32,
        key: Option<&str>,
        renderer: &mut R,
    ) -> Option<Command<A::Message>>
    where
        R: RenderingPlanExecutor,
    {
        if is_activation_key(key) {
            if let Some(hit) = self
                .resolve_hit_area(EventType::KeyDown, x, y, None, None)
                .cloned()
            {
                return self.dispatch_event(&hit, renderer);
            }

            if let Some(hit) = self
                .resolve_hit_area(EventType::Click, x, y, None, None)
                .cloned()
            {
                return self.dispatch_event(&hit, renderer);
            }
        }

        self.dispatch_pointer_event(x, y, EventType::KeyDown, renderer)
    }

    pub fn dispatch_keydown_by_ids<R>(
        &mut self,
        node: u64,
        element: u64,
        renderer: &mut R,
    ) -> Option<Command<A::Message>>
    where
        R: RenderingPlanExecutor,
    {
        self.dispatch_keydown_by_ids_with_key(node, element, None, renderer)
    }

    pub fn dispatch_keydown_by_ids_with_key<R>(
        &mut self,
        node: u64,
        element: u64,
        key: Option<&str>,
        renderer: &mut R,
    ) -> Option<Command<A::Message>>
    where
        R: RenderingPlanExecutor,
    {
        if is_activation_key(key) {
            if let Some(hit) = self
                .resolve_hit_area(EventType::KeyDown, 0.0, 0.0, Some(node), Some(element))
                .cloned()
            {
                return self.dispatch_event(&hit, renderer);
            }

            if let Some(hit) = self
                .resolve_hit_area(EventType::Click, 0.0, 0.0, Some(node), Some(element))
                .cloned()
            {
                return self.dispatch_event(&hit, renderer);
            }
        }

        self.dispatch_pointer_event_with_ids(
            0.0,
            0.0,
            EventType::KeyDown,
            Some(node),
            Some(element),
            renderer,
        )
    }

    pub fn dispatch_keyup<R>(
        &mut self,
        x: f32,
        y: f32,
        renderer: &mut R,
    ) -> Option<Command<A::Message>>
    where
        R: RenderingPlanExecutor,
    {
        self.dispatch_pointer_event(x, y, EventType::KeyUp, renderer)
    }

    pub fn dispatch_keyup_by_ids<R>(
        &mut self,
        node: u64,
        element: u64,
        renderer: &mut R,
    ) -> Option<Command<A::Message>>
    where
        R: RenderingPlanExecutor,
    {
        self.dispatch_pointer_event_with_ids(
            0.0,
            0.0,
            EventType::KeyUp,
            Some(node),
            Some(element),
            renderer,
        )
    }

    pub fn dispatch_submit<R>(
        &mut self,
        x: f32,
        y: f32,
        renderer: &mut R,
    ) -> Option<Command<A::Message>>
    where
        R: RenderingPlanExecutor,
    {
        self.dispatch_pointer_event(x, y, EventType::Submit, renderer)
    }

    pub fn dispatch_focus<R>(
        &mut self,
        x: f32,
        y: f32,
        renderer: &mut R,
    ) -> Option<Command<A::Message>>
    where
        R: RenderingPlanExecutor,
    {
        self.dispatch_pointer_event(x, y, EventType::Focus, renderer)
    }

    pub fn dispatch_blur<R>(
        &mut self,
        x: f32,
        y: f32,
        renderer: &mut R,
    ) -> Option<Command<A::Message>>
    where
        R: RenderingPlanExecutor,
    {
        self.dispatch_pointer_event(x, y, EventType::Blur, renderer)
    }

    pub fn dispatch_scroll<R>(
        &mut self,
        x: f32,
        y: f32,
        renderer: &mut R,
    ) -> Option<Command<A::Message>>
    where
        R: RenderingPlanExecutor,
    {
        self.dispatch_event_by_type(x, y, EventType::Scroll, renderer)
    }

    pub fn has_focus_events(&self) -> bool {
        self.last_output.as_ref().is_some_and(|output| {
            output
                .ep
                .hit_areas
                .iter()
                .any(|area| matches!(area.event_type, EventType::Focus | EventType::Blur))
        })
    }

    pub fn step<R>(&mut self, renderer: &mut R)
    where
        R: RenderingPlanExecutor,
    {
        if let Some(output) = self.compile_frame() {
            renderer.execute(&output.rp);
        }
    }

    pub fn handle_hit_target(&self, resolved: &ResolvedHitArea) -> (f32, f32, f32, f32) {
        (
            resolved.rect.x,
            resolved.rect.y,
            resolved.rect.width,
            resolved.rect.height,
        )
    }

    pub fn metrics(&self) -> Option<ViewMetrics> {
        let output = self.last_output.as_ref()?;
        Some(ViewMetrics {
            rendered_nodes: output.rp.nodes.len(),
            hit_targets: output.ep.hit_areas.len(),
            has_exit_command: false,
        })
    }

    pub fn state(&self) -> Option<&A::State> {
        self.state.as_ref()
    }

    pub fn state_mut(&mut self) -> Option<&mut A::State> {
        self.state.as_mut()
    }
}

fn is_activation_key(key: Option<&str>) -> bool {
    matches!(
        key,
        Some("Enter") | Some(" ") | Some("Space") | Some("Spacebar")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use alyx_executor::MemoryRenderer;
    use alyx_ir::{
        Align, Color, Container, FlexDirection, FlexLayout, Font, HitArea, IrNode, Justify, Layout,
        Padding, Size, Text, TextStyle,
    };

    #[derive(Clone)]
    enum Msg {
        Click,
        KeyDown,
        Focus,
        Blur,
        Submit,
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
                Msg::Click => {
                    *state += 1;
                    vec![Command::None]
                }
                Msg::KeyDown => {
                    *state += 2;
                    vec![Command::None]
                }
                Msg::Focus => {
                    *state += 3;
                    vec![Command::None]
                }
                Msg::Blur => {
                    *state += 4;
                    vec![Command::None]
                }
                Msg::Submit => {
                    *state += 5;
                    vec![Command::None]
                }
            }
        }

        fn view(&self, _state: &Self::State) -> IrNode<Self::Message> {
            let layout = Layout::Flex(FlexLayout::row());
            let child = IrNode::Text(Text {
                content: "Alyx".to_string(),
                style: TextStyle {
                    font: Font {
                        family: "mono".to_string(),
                    },
                    size: 16.0,
                    color: Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    },
                },
                size: Size {
                    width: 120.0,
                    height: 32.0,
                },
            });
            let root = HitArea::new(layout, child, Some(Msg::Click), Some(Msg::Click))
                .pointer_down(Msg::Click)
                .pointer_up(Msg::Submit)
                .key_down(Msg::KeyDown)
                .key_up(Msg::KeyDown)
                .focus(Msg::Focus)
                .blur(Msg::Blur)
                .submit(Msg::Submit);
            IrNode::Container(Container {
                children: vec![IrNode::HitArea(root)],
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

    fn build_runtime() -> (HeadlessRuntime<TestApp>, MemoryRenderer) {
        let app = TestApp;
        let runtime = HeadlessRuntime::new(
            app,
            Size {
                width: 200.0,
                height: 120.0,
            },
        );
        let renderer = MemoryRenderer::default();
        (runtime, renderer)
    }

    #[derive(Clone)]
    enum ActivationMsg {
        Click,
    }

    struct ActivationApp;

    impl App for ActivationApp {
        type Message = ActivationMsg;
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
                ActivationMsg::Click => {
                    *state += 1;
                    vec![Command::None]
                }
            }
        }

        fn view(&self, _state: &Self::State) -> IrNode<Self::Message> {
            let layout = Layout::Flex(FlexLayout::row());
            let child = IrNode::Text(Text {
                content: "activate".to_string(),
                style: TextStyle {
                    font: Font {
                        family: "mono".to_string(),
                    },
                    size: 16.0,
                    color: Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    },
                },
                size: Size {
                    width: 120.0,
                    height: 32.0,
                },
            });
            let root = HitArea::new(layout, child, Some(ActivationMsg::Click), None)
                .pointer_down(ActivationMsg::Click);

            IrNode::HitArea(root)
        }
    }

    fn build_activation_runtime() -> (HeadlessRuntime<ActivationApp>, MemoryRenderer) {
        let app = ActivationApp;
        let runtime = HeadlessRuntime::new(
            app,
            Size {
                width: 200.0,
                height: 120.0,
            },
        );
        let renderer = MemoryRenderer::default();
        (runtime, renderer)
    }

    #[test]
    fn dispatch_pointer_and_keyboard_events_in_runtime() {
        let (mut runtime, mut renderer) = build_runtime();
        runtime.step(&mut renderer);
        assert!(runtime.dispatch_click(10.0, 10.0, &mut renderer).is_some());
        assert!(
            runtime
                .dispatch_pointer_down(10.0, 10.0, &mut renderer)
                .is_some()
        );
        assert!(
            runtime
                .dispatch_pointer_up(10.0, 10.0, &mut renderer)
                .is_some()
        );
        assert!(
            runtime
                .dispatch_keydown(10.0, 10.0, &mut renderer)
                .is_some()
        );
        assert!(runtime.dispatch_keyup(10.0, 10.0, &mut renderer).is_some());

        assert_eq!(runtime.state, Some(11));
        assert_eq!(runtime.metrics().unwrap().rendered_nodes, 1);
    }

    #[test]
    fn dispatch_keydown_with_enter_and_space_triggers_click_if_no_keydown_handler() {
        let (mut runtime, mut renderer) = build_activation_runtime();
        runtime.step(&mut renderer);
        assert!(
            runtime
                .dispatch_keydown_with_key(10.0, 10.0, Some("Enter"), &mut renderer)
                .is_some()
        );
        assert!(
            runtime
                .dispatch_keydown_with_key(10.0, 10.0, Some(" "), &mut renderer)
                .is_some()
        );

        assert_eq!(runtime.state, Some(2));
    }

    #[test]
    fn dispatch_focus_blur_submit_and_exit_paths() {
        let (mut runtime, mut renderer) = build_runtime();
        let output = runtime.compile_frame().expect("compiled");
        let target = output
            .ep
            .hit_areas
            .iter()
            .find(|area| area.event_type == EventType::Focus)
            .expect("focus handler");
        let node = target.node_id.0;
        let element = target.element_id.0;

        assert!(
            runtime
                .dispatch_focus_by_ids(node, element, &mut renderer)
                .is_some()
        );
        assert!(
            runtime
                .dispatch_blur_by_ids(node, element, &mut renderer)
                .is_some()
        );
        assert!(
            runtime
                .dispatch_submit_by_ids(node, element, &mut renderer)
                .is_some()
        );

        assert!(runtime.dispatch_focus(0.0, 0.0, &mut renderer).is_some());
        assert!(runtime.dispatch_blur(0.0, 0.0, &mut renderer).is_some());
        assert!(runtime.dispatch_submit(0.0, 0.0, &mut renderer).is_some());

        let command = runtime.dispatch_keyup(0.0, 0.0, &mut renderer);
        assert!(command.is_some());
        assert_eq!(runtime.state, Some(26));
        assert_eq!(runtime.metrics().unwrap().hit_targets, 9);
    }

    #[test]
    fn target_lookup_by_event_type_and_coordinates() {
        let (mut runtime, _renderer) = build_runtime();
        let output = runtime.compile_frame().expect("compiled");
        let key_target = output
            .ep
            .hit_areas
            .iter()
            .find(|area| area.event_type == EventType::KeyDown)
            .expect("keydown target");

        let runtime_target = runtime
            .target_for_event_at(key_target.rect.x, key_target.rect.y, EventType::KeyDown)
            .expect("runtime target lookup");

        assert_eq!(
            runtime_target,
            (key_target.node_id.0, key_target.element_id.0)
        );
    }

    #[test]
    fn dispatch_key_events_with_unknown_ids_do_not_fallback_by_position() {
        let (mut runtime, mut renderer) = build_runtime();
        runtime.step(&mut renderer);

        assert!(
            runtime
                .dispatch_keydown_by_ids(999, 999, &mut renderer)
                .is_none()
        );
        assert!(
            runtime
                .dispatch_keyup_by_ids(999, 999, &mut renderer)
                .is_none()
        );
    }
}
