use alyx_core::ir::Size;
use alyx_core::runtime::{App, HeadlessRuntime};
use alyx_core::widgets::*;
use alyx_executor::MemoryRenderer;

#[derive(Clone)]
enum Msg {
    Inc,
    Dec,
}

fn main() {
    let mut runtime = HeadlessRuntime::new(
        NestedLayoutApp,
        Size {
            width: 360.0,
            height: 180.0,
        },
    );
    let mut renderer = MemoryRenderer::default();
    runtime.step(&mut renderer);
}

struct NestedLayoutApp;

impl App for NestedLayoutApp {
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
            Msg::Inc => {
                *state += 1;
                vec![alyx_core::runtime::Command::None]
            }
            Msg::Dec => {
                *state -= 1;
                vec![alyx_core::runtime::Command::None]
            }
        }
    }

    fn view(&self, state: &Self::State) -> alyx_core::ir::IrNode<Self::Message> {
        let nested_sidebar = Widget::Container(
            ContainerWidget::column(vec![
                Widget::Text(TextWidget::new("sidebar").size(64.0, 20.0)),
                Widget::Button(ButtonWidget::text("inc", Msg::Inc)),
                Widget::Button(ButtonWidget::text("dec", Msg::Dec)),
                Widget::Spacer {
                    width: 0.0,
                    height: 6.0,
                },
            ])
            .gap(6.0)
            .with_padding(8.0, 8.0, 8.0, 8.0),
        );

        let detail_card = Widget::Container(
            ContainerWidget::row(vec![
                Widget::Text(TextWidget::new("count").size(60.0, 24.0)),
                Widget::Text(TextWidget::new(format!("value: {state}")).size(90.0, 24.0)),
            ])
            .gap(8.0)
            .with_padding(6.0, 6.0, 6.0, 6.0),
        );

        Widget::Container(
            ContainerWidget::column(vec![
                Widget::Text(TextWidget::new("Nested Layout Example").size(220.0, 22.0)),
                Widget::Container(ContainerWidget::row(vec![
                    nested_sidebar,
                    Widget::Spacer {
                        width: 8.0,
                        height: 0.0,
                    },
                    detail_card,
                    Widget::Spacer {
                        width: 8.0,
                        height: 0.0,
                    },
                    Widget::Text(TextWidget::new("nested").size(56.0, 20.0)),
                ])),
            ])
            .gap(8.0)
            .with_padding(10.0, 10.0, 10.0, 10.0),
        )
        .into_ir()
    }
}
