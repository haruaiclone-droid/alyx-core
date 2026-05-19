pub mod compiler {
    pub use alyx_compiler::*;
}

pub mod ir {
    pub use alyx_ir::*;
}

pub mod plan {
    pub use alyx_plan::*;
}

pub mod prelude {
    pub use crate::compiler::compile;
    pub use crate::ir::*;
}

pub mod executor {
    #[allow(dead_code)]
    pub trait RenderPlanExecutor {
        fn run(&self);
    }

    #[allow(dead_code)]
    pub trait EventPlanExecutor {
        fn run(&self);
    }
}
