pub mod compiler {
    pub use alyx_compiler::*;
}

pub mod executor {
    #[allow(dead_code)]
    pub trait RenderPlanExecutor {
        fn run(&self) {}
    }

    #[allow(dead_code)]
    pub trait EventPlanExecutor {
        fn run(&self) {}
    }

    impl<T> RenderPlanExecutor for T where T: alyx_executor::RenderingPlanExecutor {}
    impl<T> EventPlanExecutor for T {}

    pub use alyx_executor::{
        EventPlanExecutor as RuntimeEventPlanExecutor, MemoryRenderer, RenderingPlanExecutor,
        TraceRenderer,
    };
}

pub mod ir {
    pub use alyx_ir::*;
}

pub mod plan {
    pub use alyx_plan::*;
}

pub mod runtime {
    pub use alyx_runtime::*;
}

pub fn run<A>(app: A, size: ir::Size) -> runtime::HeadlessRuntime<A>
where
    A: runtime::App,
    A::Message: Send,
{
    runtime::HeadlessRuntime::new(app, size)
}

pub mod web {
    pub use alyx_web::*;
}

pub mod host {
    pub use alyx_host::*;
}

pub mod widgets {
    pub use alyx_widgets::*;
}

pub mod native {
    pub use alyx_native::*;
}

pub mod prelude {
    pub use crate::compiler::compile;
    pub use crate::executor::{
        EventPlanExecutor, MemoryRenderer, RenderPlanExecutor, RenderingPlanExecutor,
        RuntimeEventPlanExecutor, TraceRenderer,
    };
    pub use crate::ir::*;
    pub use crate::run;
    pub use crate::runtime::{App, Command, HeadlessRuntime, ViewMetrics};
    pub use crate::widgets::*;
    pub use crate::widgets::{button, column, row, text};
}
