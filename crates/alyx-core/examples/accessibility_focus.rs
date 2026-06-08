use alyx_core::ir::Size as NodeSize;
use alyx_core::ir::{
    Align, Color, Container, FlexDirection, FlexLayout, Font, HitArea, IrNode, Justify, Layout,
    Padding, Size, Text, TextStyle,
};
use alyx_core::runtime::HeadlessRuntime as Runtime;
use alyx_core::runtime::{App, Command};
use alyx_executor::MemoryRenderer;
use alyx_plan::EventType;

#[derive(Clone)]
enum Msg {
    Focus,
    Blur,
    Submit,
}

fn main() {
    let mut runtime = Runtime::new(
        AccessibilityFocusApp,
        NodeSize {
            width: 320.0,
            height: 140.0,
        },
    );
    let mut renderer = MemoryRenderer::default();
    runtime.step(&mut renderer);

    let Some(output) = runtime.last_output.clone() else {
        println!("No frame output yet");
        return;
    };

    let focus_area = output
        .ep
        .hit_areas
        .iter()
        .find(|area| matches!(area.event_type, EventType::Focus))
        .expect("focus area exists");

    let _ =
        runtime.dispatch_focus_by_ids(focus_area.node_id.0, focus_area.element_id.0, &mut renderer);
    let _ = runtime.dispatch_submit_by_ids(
        focus_area.node_id.0,
        focus_area.element_id.0,
        &mut renderer,
    );
    let _ =
        runtime.dispatch_blur_by_ids(focus_area.node_id.0, focus_area.element_id.0, &mut renderer);
}

struct AccessibilityFocusApp;

impl App for AccessibilityFocusApp {
    type Message = Msg;
    type State = usize;

    fn initial_state(&self) -> Self::State {
        0
    }

    fn update(
        &self,
        state: &mut Self::State,
        message: Self::Message,
    ) -> Vec<Command<Self::Message>> {
        match message {
            Msg::Focus => {
                *state += 1;
                vec![Command::None]
            }
            Msg::Blur => {
                *state += 1;
                vec![Command::None]
            }
            Msg::Submit => {
                *state += 1;
                vec![Command::None]
            }
        }
    }

    fn view(&self, state: &Self::State) -> IrNode<Self::Message> {
        let status = format!("accessibility focus demo (state={state})");
        let target = HitArea::new(
            Layout::Flex(FlexLayout {
                direction: FlexDirection::Row,
                gap: 8.0,
                padding: Padding {
                    left: 8.0,
                    top: 6.0,
                    right: 8.0,
                    bottom: 6.0,
                },
                align: Align::Center,
                justify: Justify::Start,
            }),
            IrNode::Text(Text {
                content: status,
                style: TextStyle {
                    font: Font {
                        family: "system-ui".to_string(),
                    },
                    size: 14.0,
                    color: Color {
                        r: 0.2,
                        g: 0.2,
                        b: 0.2,
                        a: 1.0,
                    },
                },
                size: Size {
                    width: 260.0,
                    height: 24.0,
                },
            }),
            None,
            None,
        )
        .focus(Msg::Focus)
        .blur(Msg::Blur)
        .submit(Msg::Submit);

        IrNode::Container(Container {
            children: vec![IrNode::HitArea(target)],
            layout: Layout::Flex(FlexLayout {
                direction: FlexDirection::Column,
                gap: 8.0,
                padding: Padding::default(),
                align: Align::Start,
                justify: Justify::Start,
            }),
        })
    }
}
