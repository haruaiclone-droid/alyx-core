use alyx_executor::RenderingPlanExecutor;
use alyx_plan::RenderingPlan;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, window};

#[derive(Debug)]
pub struct BrowserDomRenderer {
    selector: String,
}

impl Default for BrowserDomRenderer {
    fn default() -> Self {
        Self::new("main")
    }
}

impl BrowserDomRenderer {
    pub fn new(selector: impl Into<String>) -> Self {
        Self {
            selector: selector.into(),
        }
    }

    fn mount_root(&self) -> Option<HtmlElement> {
        let document = window()?.document()?;
        if let Ok(Some(root)) = document.query_selector(&self.selector) {
            return root.dyn_into::<HtmlElement>().ok();
        }
        document.body()
    }
}

impl RenderingPlanExecutor for BrowserDomRenderer {
    fn execute(&mut self, plan: &RenderingPlan) {
        let body_html = crate::rendering_plan_to_css_canvas(plan);
        let Some(root) = self.mount_root() else {
            return;
        };
        root.set_inner_html(&body_html);
    }
}
