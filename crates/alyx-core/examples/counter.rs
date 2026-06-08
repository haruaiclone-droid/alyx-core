use alyx_core::runtime::{App, HeadlessRuntime};
use alyx_core::{ir::Size, widgets::*};
use alyx_executor::MemoryRenderer;

#[derive(Clone)]
enum Msg {
    Increment,
    Reset,
}

fn main() {
    let mut runtime = HeadlessRuntime::new(
        DemoApp,
        Size {
            width: 320.0,
            height: 120.0,
        },
    );
    let mut renderer = MemoryRenderer::default();
    runtime.step(&mut renderer);
}

struct DemoApp;

impl App for DemoApp {
    type Message = Msg;
    type State = u32;

    fn initial_state(&self) -> Self::State {
        0
    }

    fn update(
        &self,
        state: &mut Self::State,
        message: Self::Message,
    ) -> Vec<alyx_core::runtime::Command<Self::Message>> {
        match message {
            Msg::Increment => {
                *state += 1;
                vec![alyx_core::runtime::Command::None]
            }
            Msg::Reset => {
                *state = 0;
                vec![alyx_core::runtime::Command::None]
            }
        }
    }

    fn view(&self, state: &Self::State) -> alyx_core::ir::IrNode<Self::Message> {
        let count_text = Widget::Text(TextWidget::new(format!("count: {state}")).size(120.0, 24.0));
        let button = Widget::Button(ButtonWidget {
            label: TextWidget::new("increment"),
            on_click: Some(Msg::Increment),
        });
        let reset = Widget::Button(ButtonWidget {
            label: TextWidget::new("reset"),
            on_click: Some(Msg::Reset),
        });

        Widget::Container(ContainerWidget {
            children: vec![
                count_text,
                Widget::Spacer {
                    width: 0.0,
                    height: 8.0,
                },
                button,
                Widget::Spacer {
                    width: 0.0,
                    height: 8.0,
                },
                reset,
            ],
            layout: alyx_core::ir::Layout::Flex(alyx_core::ir::FlexLayout {
                direction: alyx_core::ir::FlexDirection::Column,
                gap: 6.0,
                padding: alyx_core::ir::Padding {
                    left: 8.0,
                    top: 8.0,
                    right: 8.0,
                    bottom: 8.0,
                },
                align: alyx_core::ir::Align::Start,
                justify: alyx_core::ir::Justify::Start,
            }),
        })
        .into_ir()
    }
}
