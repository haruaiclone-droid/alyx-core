use alyx_core::runtime::HeadlessRuntime;
use alyx_core::{ir::Size, runtime::App, widgets::TextInputWidget, widgets::*};
use alyx_executor::MemoryRenderer;

#[derive(Clone)]
enum Msg {
    ToggleTerms,
    ChangeName,
    Submit,
}

fn main() {
    let mut runtime = HeadlessRuntime::new(
        FormApp,
        Size {
            width: 360.0,
            height: 180.0,
        },
    );
    let mut renderer = MemoryRenderer::default();
    runtime.step(&mut renderer);
}

struct FormApp;

impl App for FormApp {
    type Message = Msg;
    type State = bool;

    fn initial_state(&self) -> Self::State {
        true
    }

    fn update(
        &self,
        state: &mut Self::State,
        message: Self::Message,
    ) -> Vec<alyx_core::runtime::Command<Self::Message>> {
        match message {
            Msg::ToggleTerms => {
                *state = !*state;
                vec![alyx_core::runtime::Command::None]
            }
            Msg::ChangeName => {
                vec![alyx_core::runtime::Command::None]
            }
            Msg::Submit => {
                vec![alyx_core::runtime::Command::None]
            }
        }
    }

    fn view(&self, state: &Self::State) -> alyx_core::ir::IrNode<Self::Message> {
        let terms = if *state { "accepted" } else { "not accepted" };

        column::<Msg>([
            Widget::Text(TextWidget::new("Registration").size(220.0, 24.0)),
            Widget::TextInput(
                TextInputWidget::new("", Some(Msg::ChangeName)).placeholder("Your name"),
            ),
            Widget::Spacer {
                width: 0.0,
                height: 8.0,
            },
            Widget::Checkbox(CheckboxWidget::new(
                format!("Terms {terms}"),
                *state,
                Some(Msg::ToggleTerms),
            )),
            Widget::Spacer {
                width: 0.0,
                height: 8.0,
            },
            button("submit").on_click(Msg::Submit),
        ])
        .gap(8.0)
        .into_ir()
    }
}
