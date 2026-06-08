use alyx_core::{
    ir::IrNode,
    runtime::{App, Command},
    widgets::*,
};

#[derive(Clone, Debug)]
pub enum Msg {
    Increment,
    Reset,
}

#[derive(Clone, Copy, Debug)]
pub struct CounterApp;

pub const VIEW_WIDTH: f32 = 320.0;
pub const VIEW_HEIGHT: f32 = 120.0;

#[allow(dead_code)]
fn main() {}

fn button_with_label(msg: Msg, label: &str) -> Widget<Msg> {
    button(label).on_click(msg)
}

pub fn counter_ui(state: &u32) -> IrNode<Msg> {
    let count_text = Widget::Text(TextWidget::new(format!("count: {state}")).size(120.0, 24.0));

    column::<Msg>([
        count_text,
        button_with_label(Msg::Increment, "+"),
        button_with_label(Msg::Reset, "reset"),
    ])
    .gap(8.0)
    .padding(8.0, 8.0, 8.0, 8.0)
    .into_ir()
}

impl App for CounterApp {
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
            Msg::Increment => {
                *state += 1;
                vec![Command::None]
            }
            Msg::Reset => {
                *state = 0;
                vec![Command::None]
            }
        }
    }

    fn view(&self, state: &Self::State) -> IrNode<Self::Message> {
        counter_ui(state)
    }
}
