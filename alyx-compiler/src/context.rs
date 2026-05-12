use alyx_plan::{EventPlan, HandlerTable};

pub(crate) struct CompileContext<Msg> {
    pub ep: EventPlan,
    pub handlers: HandlerTable<Msg>,
}

impl<Msg> CompileContext<Msg> {
    pub fn new() -> Self {
        Self {
            ep: EventPlan {
                hit_areas: Vec::new(),
            },
            handlers: HandlerTable::new(),
        }
    }
}
