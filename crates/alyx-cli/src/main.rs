use std::env;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;

use alyx_core::{
    compiler::compile,
    executor::MemoryRenderer,
    host::{HostOptions, build_web, serve_http_with_runtime},
    ir::Size,
    runtime::{App, Command as RuntimeCommand},
    widgets::{IntoIr, Widget, button, column, text},
};

type CliResult<T> = Result<T, Box<dyn std::error::Error>>;

const WASM_TARGET: &str = "wasm32-unknown-unknown";
const APP_WASM_NAME: &str = "app.wasm";
const DEFAULT_WEB_EXAMPLE: &str = "web_counter";

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match parse_command(&args) {
        Command::Help => {
            print_help();
        }
        Command::BuildWeb {
            output_dir,
            example,
        } => match run_build_web(output_dir, example) {
            Ok(path) => println!("wrote static export to {}", path.display()),
            Err(error) => {
                eprintln!("build-web failed: {error}");
                std::process::exit(1);
            }
        },
        Command::Serve { port, output_dir } => {
            let options = HostOptions {
                address: "127.0.0.1".to_string(),
                port,
                output_dir,
            };
            println!("starting preview at {}:{}", options.address, options.port);
            if let Err(error) = run_serve(options) {
                eprintln!("serve failed: {error}");
                std::process::exit(1);
            }
        }
    }
}

enum Command {
    Help,
    BuildWeb {
        output_dir: PathBuf,
        example: String,
    },
    Serve { port: u16, output_dir: PathBuf },
}

fn parse_command(args: &[String]) -> Command {
    if args.is_empty() {
        return Command::Help;
    }
    match args.first().map(String::as_str) {
        Some("-h") | Some("--help") | Some("help") => Command::Help,
        Some("build-web") => {
            let mut output_dir = PathBuf::from("dist");
            let mut example = DEFAULT_WEB_EXAMPLE.to_string();

            let mut idx = 1;
            while idx < args.len() {
                match args.get(idx).map(String::as_str) {
                    Some("--help") => return Command::Help,
                    Some("--example") => {
                        idx += 1;
                        if let Some(value) = args.get(idx) {
                            example = value.clone();
                        } else {
                            return Command::Help;
                        }
                    }
                    Some(value) if !value.starts_with('-') && output_dir == PathBuf::from("dist") => {
                        output_dir = PathBuf::from(value);
                    }
                    Some(value) => {
                        eprintln!("build-web: unknown argument '{value}'");
                        return Command::Help;
                    }
                    None => return Command::Help,
                }
                idx += 1;
            }

            Command::BuildWeb { output_dir, example }
        }
        Some("serve") => {
            let (port, output_dir) = match args.get(1).map(String::as_str) {
                Some(value) => {
                    if let Ok(port) = value.parse::<u16>() {
                        (
                            port,
                            args.get(2)
                                .map_or_else(|| PathBuf::from("dist"), PathBuf::from),
                        )
                    } else {
                        let port = args
                            .get(2)
                            .and_then(|value| value.parse::<u16>().ok())
                            .unwrap_or(3000);
                        (port, PathBuf::from(value))
                    }
                }
                None => (3000, PathBuf::from("dist")),
            };
            Command::Serve { port, output_dir }
        }
        Some(_other) => Command::Help,
        None => Command::Help,
    }
}

fn run_build_web(output_dir: PathBuf, example: String) -> CliResult<PathBuf> {
    let ui = cli_demo_ui();
    let output = compile(
        &ui,
        Size {
            width: 320.0,
            height: 120.0,
        },
    );
    let written = build_web(&output.rp, &output_dir)?;
    if let Err(error) = write_runtime_wasm(&output_dir, &example) {
        eprintln!("warning: failed to build browser runtime wasm: {error}");
    }
    Ok(written)
}

fn write_runtime_wasm(output_dir: &Path, example: &str) -> CliResult<()> {
    const RUNTIME_PLACEHOLDER_WASM: [u8; 8] = [0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| std::io::Error::other("invalid workspace manifest path"))?
        .to_path_buf();

    let status = ProcessCommand::new("cargo")
        .arg("build")
        .arg("--target")
        .arg(WASM_TARGET)
        .arg("--example")
        .arg(example)
        .arg("--manifest-path")
        .arg(workspace_root.join("Cargo.toml"))
        .status();

    let output_wasm = output_dir.join(APP_WASM_NAME);
    let write_placeholder = |reason: String| {
        eprintln!("warning: runtime wasm build unavailable ({reason}), writing fallback app.wasm");
        std::fs::write(&output_wasm, RUNTIME_PLACEHOLDER_WASM)
            .map(|_| ())
            .map_err(|error| {
                std::io::Error::other(format!("failed writing fallback app.wasm: {error}"))
            })
            .map_err(Into::into)
    };

    let Ok(status) = status else {
        return write_placeholder("cargo command invocation failed".to_string());
    };
    if !status.success() {
        return write_placeholder(format!("cargo build exited with status {status}"));
    }

    let built_wasm = workspace_root
        .join("target")
        .join(WASM_TARGET)
        .join("debug")
        .join("examples");
    let built_wasm = built_wasm.join(format!("{example}.wasm"));

    let copied = std::fs::copy(&built_wasm, &output_wasm);
    match copied {
        Ok(0) => write_placeholder("copied runtime asset was empty".to_string()),
        Ok(_) => Ok(()),
        Err(error) => write_placeholder(format!("failed to copy runtime artifact: {error}")),
    }
}

fn run_serve(options: HostOptions) -> CliResult<()> {
    let mut runtime = HostRuntime::new(
        CliDemoApp,
        Size {
            width: 320.0,
            height: 120.0,
        },
    );
    let mut renderer = MemoryRenderer::default();
    runtime.step(&mut renderer);
    println!(
        "starting runtime-backed preview: {}",
        options.output_dir.join("index.html").display()
    );
    serve_http_with_runtime(&options, &mut runtime, &mut renderer)?;
    Ok(())
}

struct CliDemoApp;

type HostRuntime = alyx_core::runtime::HeadlessRuntime<CliDemoApp>;

impl App for CliDemoApp {
    type Message = ();
    type State = ();

    fn initial_state(&self) -> Self::State {}

    fn update(
        &self,
        _state: &mut Self::State,
        _message: Self::Message,
    ) -> Vec<RuntimeCommand<Self::Message>> {
        vec![RuntimeCommand::None]
    }

    fn view(&self, _state: &Self::State) -> alyx_core::ir::IrNode<Self::Message> {
        cli_demo_ui()
    }
}

fn cli_demo_ui() -> alyx_core::ir::IrNode<()> {
    column([
        Widget::Text(text("Alyx CLI Demo").size(160.0, 24.0)),
        button::<()>("noop").on_click(()),
        Widget::Text(text("static export demo").size(140.0, 18.0)),
    ])
    .padding(8.0, 10.0, 8.0, 10.0)
    .gap(10.0)
    .into_ir()
}

fn print_help() {
    println!("Alyx CLI");
    println!("Usage: alyx <command> [options]");
    println!("Commands:");
    println!("  help                   Show this help");
    println!("  build-web [dir] [--example <name>]");
    println!("                       Build static bundle into [dir] (default: dist)");
    println!("                       Uses Rust example `<name>` for app.wasm (default: {DEFAULT_WEB_EXAMPLE})");
    println!(
        "  serve [port] [dir]     Start static preview server from [dir] (default: 3000, dist)"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_when_no_args() {
        let command = parse_command(&[]);
        assert!(matches!(command, Command::Help));
    }

    #[test]
    fn parse_build_web_dir() {
        let command = parse_command(&[String::from("build-web"), String::from("tmp")]);
        assert!(matches!(
            command,
            Command::BuildWeb { output_dir, example } if output_dir == std::path::Path::new("tmp")
                && example == DEFAULT_WEB_EXAMPLE
        ));
    }

    #[test]
    fn parse_build_web_example() {
        let command = parse_command(&[
            String::from("build-web"),
            String::from("tmp"),
            String::from("--example"),
            String::from("my_example"),
        ]);
        assert!(matches!(
            command,
            Command::BuildWeb {
                output_dir,
                example,
            } if output_dir == std::path::Path::new("tmp")
                && example == "my_example"
        ));
    }

    #[test]
    fn parse_build_web_example_without_dir() {
        let command = parse_command(&[
            String::from("build-web"),
            String::from("--example"),
            String::from("my_example"),
        ]);
        assert!(matches!(
            command,
            Command::BuildWeb {
                output_dir,
                example,
            } if output_dir == std::path::Path::new("dist")
                && example == "my_example"
        ));
    }

    #[test]
    fn parse_serve_defaults() {
        let command = parse_command(&[String::from("serve")]);
        assert!(matches!(
            command,
            Command::Serve { port, output_dir }
                if port == 3000 && output_dir == std::path::Path::new("dist")
        ));
    }

    #[test]
    fn parse_serve_dir_first() {
        let command = parse_command(&[String::from("serve"), String::from("dist")]);
        assert!(matches!(
            command,
            Command::Serve { port, output_dir }
                if port == 3000 && output_dir == std::path::Path::new("dist")
        ));
    }

    #[test]
    fn parse_serve_custom_host_and_port() {
        let command = parse_command(&[
            String::from("serve"),
            String::from("8080"),
            String::from("static"),
        ]);
        assert!(matches!(
            command,
            Command::Serve { port, output_dir }
                if port == 8080 && output_dir == std::path::Path::new("static")
        ));
    }

    #[test]
    fn help_for_unknown_command() {
        let command = parse_command(&[String::from("unexpected")]);
        assert!(matches!(command, Command::Help));
    }
}
