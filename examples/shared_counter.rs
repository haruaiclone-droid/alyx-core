use alyx_core::{
    ir::{Align, FlexDirection, FlexLayout, Justify, Layout, Padding},
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

fn button_with_label(msg: Msg, label: &str) -> Widget<Msg> {
    Widget::Button(ButtonWidget::text(label, msg))
}

pub fn counter_ui(state: &u32) -> IrNode<Msg> {
    let count_text = Widget::Text(TextWidget::new(format!("count: {state}")).size(120.0, 24.0));

    Widget::Container(ContainerWidget {
        children: vec![
            count_text,
            Widget::Spacer {
                width: 0.0,
                height: 8.0,
            },
            button_with_label(Msg::Increment, "+"),
            Widget::Spacer {
                width: 0.0,
                height: 8.0,
            },
            button_with_label(Msg::Reset, "reset"),
        ],
        layout: Layout::Flex(FlexLayout {
            direction: FlexDirection::Column,
            gap: 6.0,
            padding: Padding {
                left: 8.0,
                top: 8.0,
                right: 8.0,
                bottom: 8.0,
            },
            align: Align::Start,
            justify: Justify::Start,
        }),
    })
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
