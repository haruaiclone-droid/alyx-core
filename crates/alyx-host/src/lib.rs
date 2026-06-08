use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};

use alyx_executor::RenderingPlanExecutor;
use alyx_plan::RenderingPlan;
use alyx_runtime::{App, Command, HeadlessRuntime};
use alyx_web::{browser_event_to_runtime, export_static_html_with_endpoint, parse_browser_event};

const ALYX_MANIFEST_NAME: &str = "manifest.json";
const ALYX_CANONICAL_MANIFEST_NAME: &str = "alyx-manifest.json";
const ALYX_APP_WASM_NAME: &str = "app.wasm";
const ALYX_LOADER_JS_NAME: &str = "alyx-loader.js";

pub const DEFAULT_INDEX: &str = "index.html";

#[derive(Debug)]
pub struct HostOptions {
    pub address: String,
    pub port: u16,
    pub output_dir: PathBuf,
}

impl Default for HostOptions {
    fn default() -> Self {
        Self {
            address: "127.0.0.1".to_string(),
            port: 3000,
            output_dir: PathBuf::from("dist"),
        }
    }
}

pub fn build_static_dist(plan: &RenderingPlan, output_dir: &Path) -> io::Result<PathBuf> {
    let index = output_dir.join(DEFAULT_INDEX);
    if let Some(parent) = index.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let html = alyx_web::export_static_html(plan, "Alyx");
    let mut file = std::fs::File::create(&index)?;
    file.write_all(html.as_bytes())?;
    write_default_loader_script(output_dir)?;
    write_default_wasm(output_dir)?;
    write_manifest(output_dir)?;
    Ok(index)
}

pub fn build_web(plan: &RenderingPlan, output_dir: &Path) -> io::Result<PathBuf> {
    build_static_dist(plan, output_dir)
}

pub fn serve_http(options: &HostOptions, plan: &RenderingPlan) -> io::Result<()> {
    let output_dir = &options.output_dir;
    build_static_dist(plan, output_dir)?;
    let addr = format!("{}:{}", options.address, options.port);
    let listener = TcpListener::bind(&addr)?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(error) = serve_one_static(stream, output_dir) {
                    eprintln!("incoming connection failed: {error}");
                }
            }
            Err(error) => {
                eprintln!("incoming connection failed: {error}");
            }
        }
    }

    Ok(())
}

pub fn serve_http_with_runtime<A, R>(
    options: &HostOptions,
    runtime: &mut HeadlessRuntime<A>,
    renderer: &mut R,
) -> io::Result<()>
where
    A: App,
    A::Message: Send,
    R: RenderingPlanExecutor,
{
    let output_dir = &options.output_dir;
    let plan = runtime
        .compile_frame()
        .ok_or(io::Error::other("runtime failed to compile"))?;
    build_static_dist_with_bridge(&plan.rp, output_dir)?;
    let addr = format!("{}:{}", options.address, options.port);
    let listener = TcpListener::bind(&addr)?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                if let Err(error) = serve_one_runtime(&mut stream, output_dir, runtime, renderer) {
                    eprintln!("incoming connection failed: {error}");
                }
            }
            Err(error) => {
                eprintln!("incoming connection failed: {error}");
            }
        }
    }

    Ok(())
}

fn serve_one_static(mut stream: TcpStream, output_dir: &Path) -> io::Result<()> {
    let request = read_http_request(&mut stream)?.0;
    if request.is_empty() {
        return Ok(());
    }
    let requested = parse_request_path(&request).unwrap_or_else(|| DEFAULT_INDEX.to_string());
    let file = output_dir.join(requested.trim_start_matches('/'));
    let (content, found) = read_or_404(&file)?;
    let media = content_type_for(&file);
    write_http_response(&mut stream, &content, media, found)
}

fn serve_one_runtime<A, R>(
    stream: &mut TcpStream,
    output_dir: &Path,
    runtime: &mut HeadlessRuntime<A>,
    renderer: &mut R,
) -> io::Result<()>
where
    A: App,
    A::Message: Send,
    R: RenderingPlanExecutor,
{
    let (request, body) = read_http_request(stream)?;
    if request.is_empty() {
        return Ok(());
    }
    let (method, event_path) =
        parse_request_line(&request).unwrap_or(("GET", DEFAULT_INDEX.to_string()));
    let path = parse_request_path(&request).unwrap_or_else(|| DEFAULT_INDEX.to_string());

    if event_path == "/__alyx_event" && method == "POST" {
        if let Ok(raw) = std::str::from_utf8(&body)
            && let Some(event) = parse_browser_event(raw)
        {
            let runtime_event = browser_event_to_runtime(&event);
            let command = match runtime_event.event_type {
                alyx_plan::EventType::Click => {
                    if let (Some(node), Some(element)) = (runtime_event.node, runtime_event.element)
                    {
                        runtime.dispatch_pointer_event_with_ids(
                            runtime_event.x,
                            runtime_event.y,
                            alyx_plan::EventType::Click,
                            Some(node),
                            Some(element),
                            renderer,
                        )
                    } else {
                        runtime.dispatch_click(runtime_event.x, runtime_event.y, renderer)
                    }
                }
                alyx_plan::EventType::PointerDown => {
                    if let (Some(node), Some(element)) = (runtime_event.node, runtime_event.element)
                    {
                        runtime.dispatch_pointer_event_with_ids(
                            runtime_event.x,
                            runtime_event.y,
                            alyx_plan::EventType::PointerDown,
                            Some(node),
                            Some(element),
                            renderer,
                        )
                    } else {
                        runtime.dispatch_pointer_down(runtime_event.x, runtime_event.y, renderer)
                    }
                }
                alyx_plan::EventType::PointerUp => {
                    if let (Some(node), Some(element)) = (runtime_event.node, runtime_event.element)
                    {
                        runtime.dispatch_pointer_event_with_ids(
                            runtime_event.x,
                            runtime_event.y,
                            alyx_plan::EventType::PointerUp,
                            Some(node),
                            Some(element),
                            renderer,
                        )
                    } else {
                        runtime.dispatch_pointer_up(runtime_event.x, runtime_event.y, renderer)
                    }
                }
                alyx_plan::EventType::PointerMove => {
                    if let (Some(node), Some(element)) = (runtime_event.node, runtime_event.element)
                    {
                        runtime.dispatch_pointer_event_with_ids(
                            runtime_event.x,
                            runtime_event.y,
                            alyx_plan::EventType::PointerMove,
                            Some(node),
                            Some(element),
                            renderer,
                        )
                    } else {
                        runtime.dispatch_pointer_move(runtime_event.x, runtime_event.y, renderer)
                    }
                }
                alyx_plan::EventType::KeyDown => {
                    if let (Some(node), Some(element)) = (runtime_event.node, runtime_event.element)
                    {
                        runtime.dispatch_keydown_by_ids_with_key(
                            node,
                            element,
                            runtime_event.key.as_deref(),
                            renderer,
                        )
                    } else {
                        runtime.dispatch_keydown_with_key(
                            runtime_event.x,
                            runtime_event.y,
                            runtime_event.key.as_deref(),
                            renderer,
                        )
                    }
                }
                alyx_plan::EventType::KeyUp => {
                    if let (Some(node), Some(element)) = (runtime_event.node, runtime_event.element)
                    {
                        runtime.dispatch_keyup_by_ids(node, element, renderer)
                    } else {
                        runtime.dispatch_keyup(runtime_event.x, runtime_event.y, renderer)
                    }
                }
                alyx_plan::EventType::Focus => {
                    if let (Some(node), Some(element)) = (runtime_event.node, runtime_event.element)
                    {
                        runtime.dispatch_focus_by_ids(node, element, renderer)
                    } else {
                        runtime.dispatch_focus(runtime_event.x, runtime_event.y, renderer)
                    }
                }
                alyx_plan::EventType::Blur => {
                    if let (Some(node), Some(element)) = (runtime_event.node, runtime_event.element)
                    {
                        runtime.dispatch_blur_by_ids(node, element, renderer)
                    } else {
                        runtime.dispatch_blur(runtime_event.x, runtime_event.y, renderer)
                    }
                }
                alyx_plan::EventType::Scroll => {
                    runtime.dispatch_scroll(runtime_event.x, runtime_event.y, renderer)
                }
                alyx_plan::EventType::Submit => {
                    if let (Some(node), Some(element)) = (runtime_event.node, runtime_event.element)
                    {
                        runtime.dispatch_submit_by_ids(node, element, renderer)
                    } else {
                        runtime.dispatch_submit(runtime_event.x, runtime_event.y, renderer)
                    }
                }
                alyx_plan::EventType::Hover => {
                    runtime.dispatch_hover(runtime_event.x, runtime_event.y, renderer)
                }
                alyx_plan::EventType::NavigateBack => {
                    return write_http_status(stream, 200, b"ok");
                }
                alyx_plan::EventType::NavigateForward => {
                    return write_http_status(stream, 200, b"ok");
                }
            };

            if let Some(Command::RequestExit) = command {
                return write_http_status(stream, 200, b"exit");
            }
            return write_http_status(stream, 200, b"ok");
        }
        return write_http_status(stream, 400, b"invalid event");
    }

    let file = output_dir.join(path.trim_start_matches('/'));
    let (content, found) = read_or_404(&file)?;
    let media = content_type_for(&file);
    write_http_response(stream, &content, media, found)
}

fn write_http_status(stream: &mut TcpStream, status: u16, body: &[u8]) -> io::Result<()> {
    let message = match status {
        200 => "200 OK",
        400 => "400 Bad Request",
        404 => "404 Not Found",
        500 => "500 Internal Server Error",
        501 => "501 Not Implemented",
        503 => "503 Service Unavailable",
        _ => "500 Internal Server Error",
    };
    let mut out = String::new();
    out.push_str(&format!("HTTP/1.1 {message}\r\n"));
    out.push_str(&format!("Content-Length: {}\r\n", body.len()));
    out.push_str("Content-Type: text/plain; charset=utf-8\r\n");
    out.push_str("Connection: close\r\n");
    out.push_str("\r\n");
    out.push_str(&String::from_utf8_lossy(body));
    stream.write_all(out.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn write_http_response(
    stream: &mut TcpStream,
    body: &[u8],
    content_type: &str,
    found: bool,
) -> io::Result<()> {
    let status = if found { "200 OK" } else { "404 Not Found" };
    let mut out = String::new();
    out.push_str(&format!("HTTP/1.1 {status}\r\n"));
    out.push_str(&format!("Content-Length: {}\r\n", body.len()));
    out.push_str(&format!("Content-Type: {content_type}\r\n"));
    out.push_str("Connection: close\r\n");
    out.push_str("\r\n");
    stream.write_all(out.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()?;
    Ok(())
}

fn write_manifest(output_dir: &Path) -> io::Result<()> {
    let manifest_names = [ALYX_MANIFEST_NAME, ALYX_CANONICAL_MANIFEST_NAME];
    let manifest_body = format!(
        "{{\"name\":\"Alyx App\",\"entry\":\"{}\",\"renderer\":\"canvas\",\"alyx_version\":\"{}\",\"assets\":[],\"start_url\":\"./index.html\",\"display\":\"standalone\"}}",
        ALYX_APP_WASM_NAME,
        env!("CARGO_PKG_VERSION")
    );
    for name in manifest_names {
        let manifest = output_dir.join(name);
        if let Some(parent) = manifest.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&manifest, manifest_body)?;
    }
    Ok(())
}

fn build_static_dist_with_bridge(plan: &RenderingPlan, output_dir: &Path) -> io::Result<PathBuf> {
    let index = output_dir.join(DEFAULT_INDEX);
    if let Some(parent) = index.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let html = export_static_html_with_endpoint(plan, "Alyx", "/__alyx_event");
    let mut file = std::fs::File::create(&index)?;
    file.write_all(html.as_bytes())?;
    write_default_loader_script(output_dir)?;
    write_default_wasm(output_dir)?;
    write_manifest(output_dir)?;
    Ok(index)
}

fn write_default_loader_script(output_dir: &Path) -> io::Result<()> {
    let loader = output_dir.join(ALYX_LOADER_JS_NAME);
    if !loader.exists() {
        let script = r#"(function (global) {
  var state = {
    endpoint: "/__alyx_event",
    version: "placeholder",
  };

  function init(endpoint) {
    state.endpoint = endpoint || state.endpoint;
    console.info("Alyx loader initialized", state);
    return state;
  }

  global.alyxLoader = {
    init: init,
    state: state,
  };
})(typeof window !== "undefined" ? window : globalThis);"#;
        std::fs::write(loader, script)?;
    }
    Ok(())
}

fn write_default_wasm(output_dir: &Path) -> io::Result<()> {
    let wasm = output_dir.join(ALYX_APP_WASM_NAME);
    if !wasm.exists() {
        let data = [0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
        std::fs::write(wasm, data)?;
    }
    Ok(())
}

fn read_http_request(stream: &mut TcpStream) -> io::Result<(String, Vec<u8>)> {
    let mut raw = Vec::new();
    let mut chunk = [0_u8; 2048];

    loop {
        let read = stream.read(&mut chunk)?;
        if read == 0 {
            break;
        }
        raw.extend_from_slice(&chunk[..read]);
        if raw.windows(4).any(|window| window == b"\r\n\r\n") {
            break;
        }
        if raw.len() > 16_384 {
            break;
        }
    }

    let raw_string = String::from_utf8_lossy(&raw).to_string();
    let (head, body_start) = raw_string
        .split_once("\r\n\r\n")
        .unwrap_or((&raw_string, ""));
    let headers = parse_headers(head);
    let mut body = body_start.as_bytes().to_vec();
    if let Some(length) = headers
        .get("content-length")
        .and_then(|value| value.parse::<usize>().ok())
    {
        while body.len() < length {
            let read = stream.read(&mut chunk)?;
            if read == 0 {
                break;
            }
            body.extend_from_slice(&chunk[..read]);
        }
    }

    Ok((head.to_string(), body))
}

fn parse_headers(raw: &str) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    for line in raw.lines().skip(1) {
        if let Some((name, value)) = line.split_once(':') {
            headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_string());
        } else if line.is_empty() {
            break;
        }
    }
    headers
}

fn parse_request_line(raw: &str) -> Option<(&str, String)> {
    let line = raw.lines().next()?;
    let mut parts = line.split_whitespace();
    let method = parts.next()?;
    let path = parts.next().unwrap_or(DEFAULT_INDEX);
    Some((method, path.to_string()))
}

fn parse_request_path(raw: &str) -> Option<String> {
    parse_request_line(raw).map(|(_, path)| {
        let path = path.split('?').next().unwrap_or(&path).to_string();
        if path == "/" || path.is_empty() {
            DEFAULT_INDEX.to_string()
        } else {
            path
        }
    })
}

fn read_or_404(path: &Path) -> io::Result<(Vec<u8>, bool)> {
    if path.is_file() {
        return std::fs::read(path).map(|bytes| (bytes, true));
    }

    Ok((
        format!(
            "<!doctype html><h1>404</h1><p>not found: {}</p>",
            path.display()
        )
        .into_bytes(),
        false,
    ))
}

fn content_type_for(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default()
    {
        "html" | "htm" => "text/html; charset=utf-8",
        "js" => "application/javascript; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "wasm" => "application/wasm",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "svg" => "image/svg+xml",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_request_line_extracts_path() {
        let (method, path) = parse_request_line("POST /__alyx_event HTTP/1.1\r\n").unwrap();
        assert_eq!(method, "POST");
        assert_eq!(path, "/__alyx_event");
    }

    #[test]
    fn parse_request_path_defaults_index() {
        let path = parse_request_path("GET / HTTP/1.1\r\n").unwrap();
        assert_eq!(path, DEFAULT_INDEX);
    }

    #[test]
    fn content_type_for_file_ext() {
        assert_eq!(
            content_type_for(Path::new("foo.html")),
            "text/html; charset=utf-8"
        );
        assert_eq!(
            content_type_for(Path::new("foo.css")),
            "text/css; charset=utf-8"
        );
        assert_eq!(
            content_type_for(Path::new("foo.bin")),
            "application/octet-stream"
        );
        assert_eq!(
            content_type_for(Path::new("foo.wasm")),
            "application/wasm"
        );
    }

    #[test]
    fn build_static_dist_writes_expected_artifacts() {
        use std::fs;
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let output_dir = std::env::temp_dir().join(format!("alyx-host-static-{now}"));

        let output = crate::build_static_dist(
            &alyx_plan::RenderingPlan {
                nodes: vec![],
                accessibility: alyx_plan::AccessibilityPlan { entries: vec![] },
            },
            &output_dir,
        )
        .expect("build");

        assert!(output.exists());
        assert!(output_dir.join(ALYX_APP_WASM_NAME).is_file());
        assert!(output_dir.join(ALYX_LOADER_JS_NAME).is_file());
        assert!(output_dir.join(ALYX_MANIFEST_NAME).is_file());
        assert!(output_dir.join(ALYX_CANONICAL_MANIFEST_NAME).is_file());

        let manifest = fs::read_to_string(output_dir.join(ALYX_CANONICAL_MANIFEST_NAME))
            .expect("canonical manifest");
        assert!(manifest.contains(r#""entry":"app.wasm""#));
        assert!(manifest.contains("\"renderer\":\"canvas\""));

        let _ = fs::remove_dir_all(output_dir);
    }
}
