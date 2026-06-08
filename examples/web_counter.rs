mod shared_counter;

use alyx_core::ir::Size;
#[cfg(target_arch = "wasm32")]
use alyx_core::plan::EventType;
use alyx_core::runtime::HeadlessRuntime;
#[cfg(target_arch = "wasm32")]
use alyx_core::web::BrowserDomRenderer;
#[cfg(target_arch = "wasm32")]
use alyx_core::web::{browser_event_to_runtime, parse_browser_event};
#[cfg(not(target_arch = "wasm32"))]
use alyx_executor::MemoryRenderer;
#[cfg(target_arch = "wasm32")]
use js_sys;
use shared_counter::CounterApp;
use shared_counter::{VIEW_HEIGHT, VIEW_WIDTH};
#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, JsValue, prelude::*};
#[cfg(target_arch = "wasm32")]
use web_sys::window;

#[cfg(target_arch = "wasm32")]
thread_local! {
    static RUNTIME: RefCell<Option<WasmCounterRuntime>> = const { RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
struct WasmCounterRuntime {
    runtime: HeadlessRuntime<CounterApp>,
    renderer: BrowserDomRenderer,
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let mut runtime = HeadlessRuntime::new(
        CounterApp,
        Size {
            width: VIEW_WIDTH,
            height: VIEW_HEIGHT,
        },
    );
    let mut renderer = MemoryRenderer::default();
    runtime.step(&mut renderer);
}

#[cfg(target_arch = "wasm32")]
fn with_runtime<F, T>(handler: F) -> Option<T>
where
    F: FnOnce(&mut HeadlessRuntime<CounterApp>, &mut BrowserDomRenderer) -> T,
{
    RUNTIME.with(|runtime| {
        runtime
            .borrow_mut()
            .as_mut()
            .map(|runtime| handler(&mut runtime.runtime, &mut runtime.renderer))
    })
}

#[cfg(target_arch = "wasm32")]
fn apply_browser_event(raw: &str) {
    let Some(event) = parse_browser_event(raw) else {
        return;
    };
    let runtime_event = browser_event_to_runtime(&event);

    with_runtime(|runtime, renderer| {
        let _ = match runtime_event.event_type {
            EventType::Click => {
                if let (Some(node), Some(element)) = (runtime_event.node, runtime_event.element) {
                    runtime.dispatch_pointer_event_with_ids(
                        runtime_event.x,
                        runtime_event.y,
                        EventType::Click,
                        Some(node),
                        Some(element),
                        renderer,
                    )
                } else {
                    runtime.dispatch_click(runtime_event.x, runtime_event.y, renderer)
                }
            }
            EventType::PointerDown => {
                if let (Some(node), Some(element)) = (runtime_event.node, runtime_event.element) {
                    runtime.dispatch_pointer_event_with_ids(
                        runtime_event.x,
                        runtime_event.y,
                        EventType::PointerDown,
                        Some(node),
                        Some(element),
                        renderer,
                    )
                } else {
                    runtime.dispatch_pointer_down(runtime_event.x, runtime_event.y, renderer)
                }
            }
            EventType::PointerUp => {
                if let (Some(node), Some(element)) = (runtime_event.node, runtime_event.element) {
                    runtime.dispatch_pointer_event_with_ids(
                        runtime_event.x,
                        runtime_event.y,
                        EventType::PointerUp,
                        Some(node),
                        Some(element),
                        renderer,
                    )
                } else {
                    runtime.dispatch_pointer_up(runtime_event.x, runtime_event.y, renderer)
                }
            }
            EventType::PointerMove => {
                if let (Some(node), Some(element)) = (runtime_event.node, runtime_event.element) {
                    runtime.dispatch_pointer_event_with_ids(
                        runtime_event.x,
                        runtime_event.y,
                        EventType::PointerMove,
                        Some(node),
                        Some(element),
                        renderer,
                    )
                } else {
                    runtime.dispatch_pointer_move(runtime_event.x, runtime_event.y, renderer)
                }
            }
            EventType::KeyDown => {
                if let (Some(node), Some(element)) = (runtime_event.node, runtime_event.element) {
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
            EventType::KeyUp => {
                if let (Some(node), Some(element)) = (runtime_event.node, runtime_event.element) {
                    runtime
                        .dispatch_keyup_by_ids(node, element, renderer)
                        .or_else(|| {
                            runtime.dispatch_keyup(runtime_event.x, runtime_event.y, renderer)
                        })
                } else {
                    runtime.dispatch_keyup(runtime_event.x, runtime_event.y, renderer)
                }
            }
            EventType::Focus => {
                if let (Some(node), Some(element)) = (runtime_event.node, runtime_event.element) {
                    runtime.dispatch_focus_by_ids(node, element, renderer)
                } else {
                    runtime.dispatch_focus(runtime_event.x, runtime_event.y, renderer)
                }
            }
            EventType::Blur => {
                if let (Some(node), Some(element)) = (runtime_event.node, runtime_event.element) {
                    runtime.dispatch_blur_by_ids(node, element, renderer)
                } else {
                    runtime.dispatch_blur(runtime_event.x, runtime_event.y, renderer)
                }
            }
            EventType::Submit => {
                if let (Some(node), Some(element)) = (runtime_event.node, runtime_event.element) {
                    runtime.dispatch_submit_by_ids(node, element, renderer)
                } else {
                    runtime.dispatch_submit(runtime_event.x, runtime_event.y, renderer)
                }
            }
            EventType::Hover => runtime.dispatch_hover(runtime_event.x, runtime_event.y, renderer),
            EventType::Scroll => {
                runtime.dispatch_scroll(runtime_event.x, runtime_event.y, renderer)
            }
            EventType::NavigateBack | EventType::NavigateForward => None,
        };
        runtime.step(renderer);
    });
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn alyx_handle_event(payload: String) {
    apply_browser_event(&payload);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_start() {
    let mut runtime = HeadlessRuntime::new(
        CounterApp,
        Size {
            width: VIEW_WIDTH,
            height: VIEW_HEIGHT,
        },
    );
    let mut renderer = BrowserDomRenderer::default();

    runtime.step(&mut renderer);
    RUNTIME.with(|runtime_slot| {
        *runtime_slot.borrow_mut() = Some(WasmCounterRuntime { runtime, renderer });
    });

    if let Some(window) = window() {
        let callback = Closure::wrap(Box::new(|payload: String| {
            alyx_handle_event(payload);
        }) as Box<dyn Fn(String)>);
        let _ = js_sys::Reflect::set(
            window.unchecked_ref(),
            &JsValue::from_str("__alyxHandleEvent"),
            callback.as_ref().unchecked_ref(),
        );
        callback.forget();
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {}
