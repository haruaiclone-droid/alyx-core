use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use alyx_core::{
    compiler::compile,
    host::build_web,
    ir::*,
    runtime::App,
    web::{export_static_html, supports_aria},
    widgets::{
        ButtonWidget, CheckboxWidget, ContainerWidget, IntoIr, LinkWidget, TextInputWidget,
        TextWidget, Widget,
    },
};

use alyx_executor::MemoryRenderer;
use alyx_plan::{CompilerOutput, EventType, NavigationAction, ResolvedHitArea};
use alyx_runtime::HeadlessRuntime as Runtime;

#[derive(Clone)]
struct DemoState {
    button_clicks: u32,
    checkbox_toggles: u32,
    input_changes: u32,
    checked: bool,
}

#[derive(Clone)]
enum DemoMsg {
    ButtonClick,
    CheckboxToggle,
    InputChange,
    Navigate,
}

fn demo_view(state: &DemoState) -> IrNode<DemoMsg> {
    let terms = if state.checked {
        "accepted"
    } else {
        "not accepted"
    };
    Widget::Container(ContainerWidget {
        children: vec![
            Widget::Text(TextWidget::new("Phase integration").size(220.0, 20.0)),
            Widget::Button(ButtonWidget::text("submit", DemoMsg::ButtonClick)),
            Widget::Checkbox(CheckboxWidget::new(
                format!("Terms {terms}"),
                state.checked,
                Some(DemoMsg::CheckboxToggle),
            )),
            Widget::TextInput(
                TextInputWidget::new(
                    format!("name {}", state.input_changes),
                    Some(DemoMsg::InputChange),
                )
                .placeholder("Your name"),
            ),
            Widget::Link(LinkWidget::new("docs", "/docs", Some(DemoMsg::Navigate))),
        ],
        layout: alyx_ir::Layout::Flex(alyx_ir::FlexLayout {
            direction: alyx_ir::FlexDirection::Column,
            gap: 8.0,
            padding: alyx_ir::Padding::default(),
            align: alyx_ir::Align::Start,
            justify: alyx_ir::Justify::Start,
        }),
    })
    .into_ir()
}

struct DemoApp;

impl App for DemoApp {
    type Message = DemoMsg;
    type State = DemoState;

    fn initial_state(&self) -> Self::State {
        DemoState {
            button_clicks: 0,
            checkbox_toggles: 0,
            input_changes: 0,
            checked: false,
        }
    }

    fn update(
        &self,
        state: &mut Self::State,
        message: Self::Message,
    ) -> Vec<alyx_runtime::Command<Self::Message>> {
        match message {
            DemoMsg::ButtonClick => {
                state.button_clicks += 1;
                vec![alyx_runtime::Command::None]
            }
            DemoMsg::CheckboxToggle => {
                state.checked = !state.checked;
                state.checkbox_toggles += 1;
                vec![alyx_runtime::Command::None]
            }
            DemoMsg::InputChange => {
                state.input_changes += 1;
                vec![alyx_runtime::Command::None]
            }
            DemoMsg::Navigate => vec![alyx_runtime::Command::None],
        }
    }

    fn view(&self, state: &Self::State) -> IrNode<Self::Message> {
        demo_view(state)
    }
}

fn compile_with_state(state: &DemoState) -> CompilerOutput<DemoMsg> {
    compile(
        &demo_view(state),
        Size {
            width: 360.0,
            height: 240.0,
        },
    )
}

fn area_for_role(
    output: &CompilerOutput<DemoMsg>,
    event_type: EventType,
    role: Role,
) -> Option<ResolvedHitArea> {
    output.ep.hit_areas.iter().find_map(|area| {
        if area.event_type != event_type {
            return None;
        }
        output.rp.accessibility.entries.iter().find_map(|entry| {
            if entry.node_id == area.node_id
                && entry.element_id == area.element_id
                && entry.metadata.role == role
            {
                Some(area.clone())
            } else {
                None
            }
        })
    })
}

#[test]
fn phase_integration_flow_covers_widgets_compiler_runtime_and_web_export() {
    let state = DemoState {
        button_clicks: 0,
        checkbox_toggles: 0,
        input_changes: 0,
        checked: false,
    };

    let output = compile_with_state(&state);

    assert!(
        output
            .rp
            .accessibility
            .entries
            .iter()
            .any(|entry| entry.metadata.role == Role::Button)
    );
    assert!(
        output
            .rp
            .accessibility
            .entries
            .iter()
            .any(|entry| entry.metadata.role == Role::Checkbox)
    );
    assert!(
        output
            .rp
            .accessibility
            .entries
            .iter()
            .any(|entry| entry.metadata.role == Role::InputText)
    );
    assert!(
        output
            .rp
            .accessibility
            .entries
            .iter()
            .any(|entry| entry.metadata.role == Role::Link)
    );
    assert!(
        output
            .ep
            .navigation
            .actions
            .contains(&NavigationAction::NavigateTo("/docs".to_string()))
    );

    let html = export_static_html(&output.rp, "Alyx Integration");
    assert!(html.contains("role=\"text\""));

    let mut runtime = Runtime::new(
        DemoApp,
        Size {
            width: 360.0,
            height: 240.0,
        },
    );
    let mut renderer = MemoryRenderer::default();
    runtime.step(&mut renderer);
    runtime.compile_frame().expect("compiled");

    let button_area = area_for_role(&output, EventType::Click, Role::Button).expect("button area");
    let checkbox_area =
        area_for_role(&output, EventType::Click, Role::Checkbox).expect("checkbox area");
    let text_input_area =
        area_for_role(&output, EventType::KeyDown, Role::InputText).expect("input area");

    runtime
        .dispatch_event_by_type(
            button_area.rect.x + 1.0,
            button_area.rect.y + 1.0,
            EventType::Click,
            &mut renderer,
        )
        .expect("button click dispatch");
    runtime
        .dispatch_event_by_type(
            checkbox_area.rect.x + 1.0,
            checkbox_area.rect.y + 1.0,
            EventType::Click,
            &mut renderer,
        )
        .expect("checkbox click dispatch");
    runtime
        .dispatch_keydown_by_ids_with_key(
            text_input_area.node_id.0,
            text_input_area.element_id.0,
            Some("a"),
            &mut renderer,
        )
        .expect("text input keydown dispatch");

    let state = runtime.state().expect("runtime state");
    assert_eq!(state.button_clicks, 1);
    assert_eq!(state.checkbox_toggles, 1);
    assert_eq!(state.input_changes, 1);

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let output_dir = std::env::temp_dir().join(format!("alyx-phase-integration-{now}"));
    let index_path = build_web(&output.rp, &output_dir).expect("build web");
    let index_html = fs::read_to_string(&index_path).expect("index html");
    let manifest_path = output_dir.join("manifest.json");
    let alyx_manifest_path = output_dir.join("alyx-manifest.json");
    let loader_path = output_dir.join("alyx-loader.js");
    let app_wasm_path = output_dir.join("app.wasm");
    assert!(index_html.contains("data-node-id"));
    assert!(manifest_path.is_file());
    assert!(fs::read_to_string(&manifest_path).is_ok());
    assert!(alyx_manifest_path.is_file());
    let alyx_manifest = fs::read_to_string(&alyx_manifest_path).expect("alyx manifest");
    assert!(alyx_manifest.contains(r#""entry":"app.wasm""#));
    assert!(alyx_manifest.contains(r#""renderer":"canvas""#));
    assert!(loader_path.is_file());
    assert!(app_wasm_path.is_file());
    let _ = fs::remove_dir_all(&output_dir);
}

#[test]
fn supports_aria_summary_matches_visibility() {
    let output = compile_with_state(&DemoState {
        button_clicks: 0,
        checkbox_toggles: 0,
        input_changes: 0,
        checked: false,
    });
    let summary = supports_aria(&output.rp, &output.ep);
    assert!(summary.contains("nodes"));
    assert!(summary.contains("hit areas"));
    assert!(summary.contains("focus order"));

    assert!(
        !output
            .ep
            .navigation
            .actions
            .contains(&NavigationAction::NavigateBack)
    );
    assert!(
        !output
            .ep
            .navigation
            .actions
            .contains(&NavigationAction::NavigateForward)
    );
    let all_roles: Vec<_> = output
        .rp
        .accessibility
        .entries
        .iter()
        .map(|entry| &entry.metadata.role)
        .collect();
    assert!(all_roles.contains(&&Role::Button));
    assert!(all_roles.contains(&&Role::InputText));
}
