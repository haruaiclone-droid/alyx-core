use alyx_core::ir::ImageSource;
use alyx_core::runtime::HeadlessRuntime;
use alyx_core::{ir::Size, runtime::App, widgets::*};
use alyx_executor::MemoryRenderer;

#[derive(Clone)]
enum Msg {
    Reset,
}

fn main() {
    let mut runtime = HeadlessRuntime::new(
        ImageDemo,
        Size {
            width: 320.0,
            height: 120.0,
        },
    );
    let mut renderer = MemoryRenderer::default();
    runtime.step(&mut renderer);
}

struct ImageDemo;

impl App for ImageDemo {
    type Message = Msg;
    type State = ();

    fn initial_state(&self) -> Self::State {}

    fn update(
        &self,
        _state: &mut Self::State,
        message: Self::Message,
    ) -> Vec<alyx_core::runtime::Command<Self::Message>> {
        match message {
            Msg::Reset => vec![alyx_core::runtime::Command::None],
        }
    }

    fn view(&self, _state: &Self::State) -> alyx_core::ir::IrNode<Self::Message> {
        column::<Msg>([
            Widget::Text(TextWidget::new("logo preview").size(120.0, 24.0)),
            Widget::Image(
                ImageWidget::new(ImageSource::Url("https://example.com/logo.png".to_string()))
                    .size(128.0, 64.0),
            ),
            Widget::Link(LinkWidget::new("open website", "https://example.com", None)),
            button("reset").on_click(Msg::Reset),
        ])
        .gap(6.0)
        .into_ir()
    }
}
