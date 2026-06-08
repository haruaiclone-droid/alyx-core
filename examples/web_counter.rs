mod shared_counter;

#[cfg(not(target_arch = "wasm32"))]
use alyx_executor::MemoryRenderer;
use alyx_core::runtime::HeadlessRuntime;
use alyx_core::ir::Size;
use shared_counter::{VIEW_HEIGHT, VIEW_WIDTH};
use shared_counter::CounterApp;
#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use web_sys::window;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::*, JsCast, JsValue};
#[cfg(target_arch = "wasm32")]
use alyx_plan::EventType;
#[cfg(target_arch = "wasm32")]
use alyx_web::wasm::BrowserDomRenderer;
#[cfg(target_arch = "wasm32")]
use js_sys;

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
    RUNTIME.with(|runtime| runtime.borrow_mut().as_mut().map(|runtime| handler(&mut runtime.runtime, &mut runtime.renderer)))
}

#[cfg(target_arch = "wasm32")]
fn apply_browser_event(raw: &str) {
    let Some(event) = alyx_web::parse_browser_event(raw) else {
        return;
    };
    let runtime_event = alyx_web::browser_event_to_runtime(&event);

    with_runtime(|runtime, renderer| {
        let _ = match runtime_event.event_type {
            EventType::Click => {
                if let (Some(node), Some(element)) = (runtime_event.node, runtime_event.element) {
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
            EventType::PointerDown => {
                if let (Some(node), Some(element)) = (runtime_event.node, runtime_event.element) {
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
            EventType::PointerUp => {
                if let (Some(node), Some(element)) = (runtime_event.node, runtime_event.element) {
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
            EventType::PointerMove => {
                if let (Some(node), Some(element)) = (runtime_event.node, runtime_event.element) {
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
                        .or_else(|| runtime.dispatch_keyup(runtime_event.x, runtime_event.y, renderer))
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
            EventType::Scroll => runtime.dispatch_scroll(runtime_event.x, runtime_event.y, renderer),
            EventType::NavigateBack | EventType::NavigateForward => None,
        };
        runtime.step(renderer);
    });
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn __alyxHandleEvent(payload: String) {
    apply_browser_event(&payload);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    let mut runtime = HeadlessRuntime::new(
        CounterApp,
        Size {
            width: VIEW_WIDTH,
            height: VIEW_HEIGHT,
        },
    );
    let mut renderer = BrowserDomRenderer::default();

    runtime.step(&mut renderer);
    RUNTIME.with(|runtime| {
        *runtime.borrow_mut() = Some(WasmCounterRuntime { runtime, renderer });
    });

    if let Some(window) = window() {
        let callback = Closure::wrap(Box::new(|payload: String| {
            __alyxHandleEvent(payload);
        }) as Box<dyn Fn(String)>);
        let _ = js_sys::Reflect::set(
            window.unchecked_ref(),
            &JsValue::from_str("__alyxHandleEvent"),
            callback.as_ref().unchecked_ref(),
        );
        callback.forget();
    }
}
