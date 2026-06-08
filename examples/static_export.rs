use std::path::PathBuf;

use alyx_core::ir::Justify;
use alyx_core::ir::{Align, FlexDirection, FlexLayout, Layout, Padding, Size};
use alyx_core::{host::HostOptions, host::build_web, host::serve_http, widgets::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use alyx_core::compiler::compile;
    let ui = app_ui();
    let output = compile(
        &ui,
        Size {
            width: 320.0,
            height: 120.0,
        },
    );

    let out_dir = PathBuf::from("dist");
    let _ = build_web(&output.rp, &out_dir)?;
    let options = HostOptions {
        address: "127.0.0.1".to_string(),
        port: 4200,
        output_dir: out_dir,
    };
    println!(
        "Serving static preview at {}:{}",
        options.address, options.port
    );
    let _ = serve_http(&options, &output.rp);

    Ok(())
}

fn app_ui() -> alyx_core::ir::IrNode<()> {
    Widget::Container(ContainerWidget {
        children: vec![
            Widget::Text(TextWidget::new("Static export").size(160.0, 24.0)),
            Widget::Text(TextWidget::new("Alyx host build preview").size(220.0, 18.0)),
        ],
        layout: Layout::Flex(FlexLayout {
            direction: FlexDirection::Column,
            gap: 8.0,
            padding: Padding::default(),
            align: Align::Start,
            justify: Justify::Start,
        }),
    })
    .into_ir()
}
