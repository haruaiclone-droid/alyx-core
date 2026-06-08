use alyx_core::runtime::App;
use alyx_core::widgets::*;
use alyx_core::{ir::Size, runtime::HeadlessRuntime};
use alyx_executor::MemoryRenderer;

#[derive(Clone)]
enum Msg {
    Increment,
}

fn main() {
    let mut runtime = HeadlessRuntime::new(
        WebCounterApp,
        Size {
            width: 320.0,
            height: 120.0,
        },
    );
    let mut renderer = MemoryRenderer::default();
    runtime.step(&mut renderer);
}

struct WebCounterApp;

impl App for WebCounterApp {
    type Message = Msg;
    type State = i32;

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
        }
    }

    fn view(&self, state: &Self::State) -> alyx_core::ir::IrNode<Self::Message> {
        Widget::Container(ContainerWidget {
            children: vec![
                Widget::Text(TextWidget::new(format!("web counter: {state}")).size(140.0, 24.0)),
                Widget::Button(ButtonWidget {
                    label: TextWidget::new("increment"),
                    on_click: Some(Msg::Increment),
                }),
            ],
            layout: alyx_ir::Layout::Flex(alyx_ir::FlexLayout {
                direction: alyx_ir::FlexDirection::Column,
                gap: 10.0,
                padding: alyx_ir::Padding {
                    left: 12.0,
                    top: 10.0,
                    right: 12.0,
                    bottom: 10.0,
                },
                align: alyx_ir::Align::Start,
                justify: alyx_ir::Justify::Start,
            }),
        })
        .into_ir()
    }
}
