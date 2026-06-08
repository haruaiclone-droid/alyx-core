use std::collections::HashMap;

use alyx_ir::{AccessibilityMetadata, Role};
use alyx_plan::{EventPlan, EventType, RenderingPlan, RpImage, RpNode, RpText};

#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(target_arch = "wasm32")]
pub use wasm::BrowserDomRenderer;

#[derive(Clone, Debug)]
pub enum BrowserEvent {
    Click {
        x: f32,
        y: f32,
        node: Option<u64>,
        element: Option<u64>,
    },
    PointerDown {
        x: f32,
        y: f32,
        node: Option<u64>,
        element: Option<u64>,
    },
    PointerUp {
        x: f32,
        y: f32,
        node: Option<u64>,
        element: Option<u64>,
    },
    PointerMove {
        x: f32,
        y: f32,
        node: Option<u64>,
        element: Option<u64>,
    },
    KeyboardDown {
        key: String,
        node: Option<u64>,
        element: Option<u64>,
    },
    KeyboardUp {
        key: String,
        node: Option<u64>,
        element: Option<u64>,
    },
    Scroll {
        x: f32,
        y: f32,
        delta_x: f32,
        delta_y: f32,
    },
    Focus {
        node: u64,
        element: u64,
    },
    Blur {
        node: u64,
        element: u64,
    },
    Submit {
        node: u64,
        element: u64,
    },
    NavigateBack,
    NavigateForward,
}

#[derive(Clone, Debug)]
pub struct RuntimeEvent {
    pub x: f32,
    pub y: f32,
    pub event_type: EventType,
    pub key: Option<String>,
    pub node: Option<u64>,
    pub element: Option<u64>,
}

pub fn rendering_plan_to_css_canvas(plan: &RenderingPlan) -> String {
    let accessibility_by_element = accessibility_by_element(&plan.accessibility);
    let mut out = String::new();
    out.push_str("<div class=\"alyx-canvas\">\\n");
    for node in &plan.nodes {
        match node {
            RpNode::Text(text) => {
                let metadata = accessibility_by_element.get(&element_key(text.element));
                out.push_str(&render_text_node(text, metadata));
            }
            RpNode::Image(image) => {
                let metadata = accessibility_by_element.get(&element_key(image.element));
                out.push_str(&render_image_node(image, metadata));
            }
        }
    }
    out.push_str("</div>");
    out
}

pub fn export_static_html(plan: &RenderingPlan, title: &str) -> String {
    export_static_html_with_endpoint(plan, title, "/__alyx_event")
}

pub fn export_static_html_with_endpoint(
    plan: &RenderingPlan,
    title: &str,
    event_endpoint: &str,
) -> String {
    format!(
        "<!doctype html>
<html><head><meta charset=\"utf-8\"><title>{}</title></head>
<body><main>{}</main>
<script src=\"./alyx-loader.js\"></script>
<script>{}</script>
</body>
</html>",
        title,
        rendering_plan_to_css_canvas(plan),
        js_event_bridge(event_endpoint)
    )
}

fn render_text_node(node: &RpText, metadata: Option<&AccessibilityMetadata>) -> String {
    let metadata_attrs = accessibility_attrs(metadata);
    format!(
        "<p data-node-id=\"{}\" data-element-id=\"{}\"{} style=\"position:absolute; left:{}px; top:{}px; width:{}px; height:{}px; font-size:{}px\">{}</p>\\n",
        node.element.node_id.0,
        node.element.element_id.0,
        if metadata_attrs.is_empty() {
            String::new()
        } else {
            format!(" {}", metadata_attrs)
        },
        node.x,
        node.y,
        node.width,
        node.height,
        node.style.size,
        html_escape(&node.content)
    )
}

fn render_image_node(node: &RpImage, metadata: Option<&AccessibilityMetadata>) -> String {
    let metadata_attrs = accessibility_attrs(metadata);
    let attr_prefix = if metadata_attrs.is_empty() {
        String::new()
    } else {
        format!(" {}", metadata_attrs)
    };
    match &node.src {
        alyx_ir::ImageSource::Path(path) => format!(
            "<img data-node-id=\"{}\" data-element-id=\"{}\"{} src=\"{}\" style=\"position:absolute; left:{}px; top:{}px; width:{}px; height:{}px\"/>\\n",
            node.element.node_id.0,
            node.element.element_id.0,
            attr_prefix,
            path.display(),
            node.x,
            node.y,
            node.width,
            node.height
        ),
        alyx_ir::ImageSource::Url(url) => format!(
            "<img data-node-id=\"{}\" data-element-id=\"{}\"{} src=\"{}\" style=\"position:absolute; left:{}px; top:{}px; width:{}px; height:{}px\"/>\\n",
            node.element.node_id.0,
            node.element.element_id.0,
            attr_prefix,
            url,
            node.x,
            node.y,
            node.width,
            node.height
        ),
        alyx_ir::ImageSource::Bytes(_) => {
            "<!-- binary image unavailable in static export -->\\n".to_string()
        }
    }
}

pub fn browser_event_to_json(event: &BrowserEvent) -> String {
    match event {
        BrowserEvent::Click {
            x,
            y,
            node,
            element,
        } => format!(
            "{{\"type\":\"click\",\"x\":{x},\"y\":{y},\"node\":{},\"element\":{}}}",
            node.unwrap_or(0),
            element.unwrap_or(0)
        ),
        BrowserEvent::PointerDown {
            x,
            y,
            node,
            element,
        } => {
            format!(
                "{{\"type\":\"pointerdown\",\"x\":{x},\"y\":{y},\"node\":{},\"element\":{}}}",
                node.unwrap_or(0),
                element.unwrap_or(0)
            )
        }
        BrowserEvent::PointerUp {
            x,
            y,
            node,
            element,
        } => {
            format!(
                "{{\"type\":\"pointerup\",\"x\":{x},\"y\":{y},\"node\":{},\"element\":{}}}",
                node.unwrap_or(0),
                element.unwrap_or(0)
            )
        }
        BrowserEvent::PointerMove {
            x,
            y,
            node,
            element,
        } => {
            format!(
                "{{\"type\":\"pointermove\",\"x\":{x},\"y\":{y},\"node\":{},\"element\":{}}}",
                node.unwrap_or(0),
                element.unwrap_or(0)
            )
        }
        BrowserEvent::KeyboardDown { key, node, element } => format!(
            "{{\"type\":\"keydown\",\"key\":\"{}\",\"node\":{},\"element\":{}}}",
            key,
            node.unwrap_or(0),
            element.unwrap_or(0)
        ),
        BrowserEvent::KeyboardUp { key, node, element } => format!(
            "{{\"type\":\"keyup\",\"key\":\"{}\",\"node\":{},\"element\":{}}}",
            key,
            node.unwrap_or(0),
            element.unwrap_or(0)
        ),
        BrowserEvent::Scroll {
            x,
            y,
            delta_x,
            delta_y,
        } => format!(
            "{{\"type\":\"scroll\",\"x\":{x},\"y\":{y},\"delta_x\":{delta_x},\"delta_y\":{delta_y}}}"
        ),
        BrowserEvent::Focus { node, element } => {
            format!("{{\"type\":\"focus\",\"node\":{node},\"element\":{element}}}")
        }
        BrowserEvent::Blur { node, element } => {
            format!("{{\"type\":\"blur\",\"node\":{node},\"element\":{element}}}")
        }
        BrowserEvent::Submit { node, element } => {
            format!("{{\"type\":\"submit\",\"node\":{node},\"element\":{element}}}")
        }
        BrowserEvent::NavigateBack => "{\"type\":\"navigate_back\"}".to_string(),
        BrowserEvent::NavigateForward => "{\"type\":\"navigate_forward\"}".to_string(),
    }
}

pub fn browser_event_to_runtime(event: &BrowserEvent) -> RuntimeEvent {
    match event {
        BrowserEvent::Click {
            x,
            y,
            node,
            element,
        } => RuntimeEvent {
            x: *x,
            y: *y,
            event_type: EventType::Click,
            key: None,
            node: *node,
            element: *element,
        },
        BrowserEvent::PointerDown {
            x,
            y,
            node,
            element,
        } => RuntimeEvent {
            x: *x,
            y: *y,
            event_type: EventType::PointerDown,
            key: None,
            node: *node,
            element: *element,
        },
        BrowserEvent::PointerUp {
            x,
            y,
            node,
            element,
        } => RuntimeEvent {
            x: *x,
            y: *y,
            event_type: EventType::PointerUp,
            key: None,
            node: *node,
            element: *element,
        },
        BrowserEvent::PointerMove {
            x,
            y,
            node,
            element,
        } => RuntimeEvent {
            x: *x,
            y: *y,
            event_type: EventType::PointerMove,
            key: None,
            node: *node,
            element: *element,
        },
        BrowserEvent::KeyboardDown { key, node, element } => RuntimeEvent {
            x: 0.0,
            y: 0.0,
            event_type: EventType::KeyDown,
            key: Some(key.clone()),
            node: *node,
            element: *element,
        },
        BrowserEvent::KeyboardUp { key, node, element } => RuntimeEvent {
            x: 0.0,
            y: 0.0,
            event_type: EventType::KeyUp,
            key: Some(key.clone()),
            node: *node,
            element: *element,
        },
        BrowserEvent::Scroll {
            x,
            y,
            delta_x: _,
            delta_y: _,
        } => RuntimeEvent {
            x: *x,
            y: *y,
            event_type: EventType::Scroll,
            key: None,
            node: None,
            element: None,
        },
        BrowserEvent::Focus { node, element } => RuntimeEvent {
            x: 0.0,
            y: 0.0,
            event_type: EventType::Focus,
            key: None,
            node: Some(*node),
            element: Some(*element),
        },
        BrowserEvent::Blur { node, element } => RuntimeEvent {
            x: 0.0,
            y: 0.0,
            event_type: EventType::Blur,
            key: None,
            node: Some(*node),
            element: Some(*element),
        },
        BrowserEvent::Submit { node, element } => RuntimeEvent {
            x: 0.0,
            y: 0.0,
            event_type: EventType::Submit,
            key: None,
            node: Some(*node),
            element: Some(*element),
        },
        BrowserEvent::NavigateBack => RuntimeEvent {
            x: 0.0,
            y: 0.0,
            event_type: EventType::NavigateBack,
            key: None,
            node: None,
            element: None,
        },
        BrowserEvent::NavigateForward => RuntimeEvent {
            x: 0.0,
            y: 0.0,
            event_type: EventType::NavigateForward,
            key: None,
            node: None,
            element: None,
        },
    }
}

pub fn parse_browser_event(payload: &str) -> Option<BrowserEvent> {
    use serde_json::Value;

    let value: Value = serde_json::from_str(payload).ok()?;
    let event_type = value.get("type")?.as_str()?;

    match event_type {
        "click" => Some(BrowserEvent::Click {
            x: value.get("x")?.as_f64()? as f32,
            y: value.get("y")?.as_f64()? as f32,
            node: event_id(value.get("node")),
            element: event_id(value.get("element")),
        }),
        "pointerdown" => Some(BrowserEvent::PointerDown {
            x: value.get("x")?.as_f64()? as f32,
            y: value.get("y")?.as_f64()? as f32,
            node: event_id(value.get("node")),
            element: event_id(value.get("element")),
        }),
        "pointerup" => Some(BrowserEvent::PointerUp {
            x: value.get("x")?.as_f64()? as f32,
            y: value.get("y")?.as_f64()? as f32,
            node: event_id(value.get("node")),
            element: event_id(value.get("element")),
        }),
        "pointermove" => Some(BrowserEvent::PointerMove {
            x: value.get("x")?.as_f64()? as f32,
            y: value.get("y")?.as_f64()? as f32,
            node: event_id(value.get("node")),
            element: event_id(value.get("element")),
        }),
        "keydown" => Some(BrowserEvent::KeyboardDown {
            key: value.get("key")?.as_str()?.to_string(),
            node: event_id(value.get("node")),
            element: event_id(value.get("element")),
        }),
        "keyup" => Some(BrowserEvent::KeyboardUp {
            key: value.get("key")?.as_str()?.to_string(),
            node: event_id(value.get("node")),
            element: event_id(value.get("element")),
        }),
        "scroll" => Some(BrowserEvent::Scroll {
            x: value.get("x")?.as_f64()? as f32,
            y: value.get("y")?.as_f64()? as f32,
            delta_x: value.get("delta_x")?.as_f64()? as f32,
            delta_y: value.get("delta_y")?.as_f64()? as f32,
        }),
        "focus" => Some(BrowserEvent::Focus {
            node: event_id(value.get("node"))?,
            element: event_id(value.get("element"))?,
        }),
        "blur" => Some(BrowserEvent::Blur {
            node: event_id(value.get("node"))?,
            element: event_id(value.get("element"))?,
        }),
        "submit" => Some(BrowserEvent::Submit {
            node: event_id(value.get("node"))?,
            element: event_id(value.get("element"))?,
        }),
        "navigate_back" => Some(BrowserEvent::NavigateBack),
        "navigate_forward" => Some(BrowserEvent::NavigateForward),
        _ => None,
    }
}

fn accessibility_by_element(
    plan: &alyx_plan::AccessibilityPlan,
) -> HashMap<(u64, u64), AccessibilityMetadata> {
    let mut map = HashMap::with_capacity(plan.entries.len());
    for entry in &plan.entries {
        map.insert(
            (entry.node_id.0, entry.element_id.0),
            entry.metadata.clone(),
        );
    }
    map
}

fn accessibility_attrs(metadata: Option<&AccessibilityMetadata>) -> String {
    let metadata = match metadata {
        Some(metadata) => metadata,
        None => return String::new(),
    };

    let mut attrs = Vec::new();
    match metadata.role {
        Role::Generic => {}
        Role::Button => attrs.push(r#"role="button""#.to_string()),
        Role::Checkbox => attrs.push(r#"role="checkbox""#.to_string()),
        Role::Link => attrs.push(r#"role="link""#.to_string()),
        Role::Text => attrs.push(r#"role="text""#.to_string()),
        Role::Image => attrs.push(r#"role="img""#.to_string()),
        Role::Pane => attrs.push(r#"role="group""#.to_string()),
        Role::InputText => attrs.push(r#"role="textbox""#.to_string()),
        Role::List => attrs.push(r#"role="list""#.to_string()),
        Role::Custom(ref name) if !name.is_empty() => {
            attrs.push(format!(r#"role="{}""#, html_escape(name)))
        }
        Role::Custom(_) => {}
    }
    if let Some(label) = &metadata.label {
        attrs.push(format!(r#"aria-label="{}""#, html_escape(label)));
    }
    if let Some(description) = &metadata.description {
        attrs.push(format!(
            r#"aria-description="{}""#,
            html_escape(description)
        ));
    }
    if metadata.disabled {
        attrs.push(r#"aria-disabled="true""#.to_string());
    }
    if metadata.focusable {
        attrs.push(r#"tabindex="0""#.to_string());
    }
    attrs.join(" ")
}

fn element_key(element: alyx_plan::RenderingElement) -> (u64, u64) {
    (element.node_id.0, element.element_id.0)
}

fn event_id(value: Option<&serde_json::Value>) -> Option<u64> {
    value
        .and_then(serde_json::Value::as_u64)
        .filter(|value| *value > 0)
}

pub fn js_event_bridge(endpoint: &str) -> String {
    let endpoint = endpoint.replace('\\', "\\\\").replace('"', "\\\"");
    format!(
        r#"
(function() {{
  const endpoint = "{endpoint}";
  async function post(payload) {{
    try {{
      if (typeof window.__alyxHandleEvent === "function") {{
        window.__alyxHandleEvent(JSON.stringify(payload));
        return;
      }}
      await fetch(endpoint, {{
        method: "POST",
        headers: {{ "Content-Type": "application/json" }},
        body: JSON.stringify(payload),
      }});
    }} catch (error) {{
      console.error("Alyx event dispatch failed", error);
    }}
  }}
  function eventTargetIds(target) {{
    if (!target || !target.dataset) {{
      return {{}};
    }}
    const node = Number(target.dataset.nodeId || 0);
    const element = Number(target.dataset.elementId || 0);
    const ids = {{}};

    if (Number.isFinite(node) && node > 0) {{
      ids.node = node;
    }}
    if (Number.isFinite(element) && element > 0) {{
      ids.element = element;
    }}
    return {{
      ...ids,
    }};
  }}
  window.addEventListener("click", (event) => {{
    const ids = eventTargetIds(event.target);
    post({{"type":"click","x":event.clientX,"y":event.clientY,"node":ids.node,"element":ids.element}});
  }});
  window.addEventListener("pointerdown", (event) => {{
    const ids = eventTargetIds(event.target);
    post({{"type":"pointerdown","x":event.clientX,"y":event.clientY,"node":ids.node,"element":ids.element}});
  }});
  window.addEventListener("pointerup", (event) => {{
    const ids = eventTargetIds(event.target);
    post({{"type":"pointerup","x":event.clientX,"y":event.clientY,"node":ids.node,"element":ids.element}});
  }});
  window.addEventListener("pointermove", (event) => {{
    const ids = eventTargetIds(event.target);
    post({{"type":"pointermove","x":event.clientX,"y":event.clientY,"node":ids.node,"element":ids.element}});
  }});
  window.addEventListener("keydown", (event) => {{
    const ids = eventTargetIds(document.activeElement);
    post({{"type":"keydown","key":event.key,"node":ids.node,"element":ids.element}});
  }});
  window.addEventListener("keyup", (event) => {{
    const ids = eventTargetIds(document.activeElement);
    post({{"type":"keyup","key":event.key,"node":ids.node,"element":ids.element}});
  }});
  window.addEventListener("scroll", () => {{
    post({{"type":"scroll","x":window.scrollX,"y":window.scrollY,"delta_x":0,"delta_y":0}});
  }});
  window.addEventListener("focusin", (event) => {{
    const ids = eventTargetIds(event.target);
    post({{"type":"focus","node":ids.node,"element":ids.element}});
  }});
  window.addEventListener("focusout", (event) => {{
    const ids = eventTargetIds(event.target);
    post({{"type":"blur","node":ids.node,"element":ids.element}});
  }});
}})();
"#
    )
}

pub fn supports_aria(plan: &RenderingPlan, event_plan: &EventPlan) -> String {
    format!(
        "{} nodes, {} hit areas, {} focus order",
        plan.nodes.len(),
        event_plan.hit_areas.len(),
        event_plan.focus_order.len()
    )
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn browser_event_roundtrip_json() {
        let parsed =
            parse_browser_event(r#"{"type":"keydown","key":"Enter","node":12,"element":34}"#)
                .expect("parsed");
        assert!(matches!(
            parsed,
            BrowserEvent::KeyboardDown { ref key, .. } if key == "Enter"
        ));

        let as_json = browser_event_to_json(&parsed);
        let reparsed = parse_browser_event(&as_json).expect("reparsed");
        assert!(matches!(
            reparsed,
            BrowserEvent::KeyboardDown { ref key, .. } if key == "Enter"
        ));
    }

    #[test]
    fn browser_event_roundtrip_with_target_ids() {
        let parsed =
            parse_browser_event(r#"{"type":"submit","node":7,"element":8}"#).expect("parsed");
        assert!(matches!(
            parsed,
            BrowserEvent::Submit {
                node: 7,
                element: 8
            }
        ));
        let runtime = browser_event_to_runtime(&parsed);
        assert_eq!(runtime.node, Some(7));
        assert_eq!(runtime.element, Some(8));
    }

    #[test]
    fn browser_event_roundtrip_navigation_types() {
        let parsed_back = parse_browser_event(r#"{"type":"navigate_back"}"#).expect("parsed");
        assert!(matches!(parsed_back, BrowserEvent::NavigateBack));
        let parsed_forward = parse_browser_event(r#"{"type":"navigate_forward"}"#).expect("parsed");
        assert!(matches!(parsed_forward, BrowserEvent::NavigateForward));
    }

    #[test]
    fn rendering_plan_to_canvas_includes_accessibility_attrs() {
        let plan = RenderingPlan {
            nodes: vec![RpNode::Text(alyx_plan::RpText {
                x: 0.0,
                y: 0.0,
                width: 10.0,
                height: 12.0,
                content: "hello".to_string(),
                style: alyx_plan::RpTextStyle {
                    font: alyx_ir::Font {
                        family: "mono".to_string(),
                    },
                    size: 12.0,
                    color: alyx_ir::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    },
                },
                element: alyx_plan::RenderingElement {
                    node_id: alyx_ir::NodeId(1),
                    element_id: alyx_ir::ElementId(2),
                },
            })],
            accessibility: alyx_plan::AccessibilityPlan {
                entries: vec![alyx_plan::AccessibilityPlanEntry {
                    node_id: alyx_ir::NodeId(1),
                    element_id: alyx_ir::ElementId(2),
                    metadata: alyx_ir::AccessibilityMetadata::new()
                        .role(alyx_ir::Role::Button)
                        .label("Run"),
                    rect: alyx_ir::Rect {
                        x: 0.0,
                        y: 0.0,
                        width: 10.0,
                        height: 12.0,
                    },
                }],
            },
        };

        let html = rendering_plan_to_css_canvas(&plan);
        assert!(html.contains("role=\"button\""));
        assert!(html.contains("aria-label=\"Run\""));
    }

    #[test]
    fn browser_event_missing_ids_are_none() {
        let parsed = parse_browser_event(r#"{"type":"click","x":10,"y":20}"#).expect("parsed");
        assert!(matches!(
            parsed,
            BrowserEvent::Click {
                x: 10.0,
                y: 20.0,
                node: None,
                element: None
            }
        ));

        let parsed_zero =
            parse_browser_event(r#"{"type":"click","x":1,"y":2,"node":0,"element":0}"#)
                .expect("parsed_zero");
        assert!(matches!(
            parsed_zero,
            BrowserEvent::Click {
                node: None,
                element: None,
                ..
            }
        ));
    }

    #[test]
    fn runtime_event_maps_touch_types() {
        let runtime = browser_event_to_runtime(&BrowserEvent::PointerDown {
            x: 10.0,
            y: 20.0,
            node: None,
            element: None,
        });
        assert_eq!(runtime.event_type, EventType::PointerDown);
        assert_eq!(runtime.x, 10.0);
        assert_eq!(runtime.y, 20.0);
    }

    #[test]
    fn js_bridge_contains_click_listener() {
        let script = js_event_bridge("/__alyx_event");
        assert!(script.contains("addEventListener(\"click\""));
        assert!(script.contains("/__alyx_event"));
        assert!(script.contains("eventTargetIds"));
        assert!(script.contains("document.activeElement"));
        assert!(script.contains("__alyxHandleEvent"));
        assert!(script.contains("JSON.stringify"));
    }
}
