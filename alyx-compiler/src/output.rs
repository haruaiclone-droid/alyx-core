use alyx_plan::{EventPlan, HandlerTable, RenderingPlan};

#[derive(Clone, Debug, PartialEq)]
pub struct CompilerOutput<Msg> {
    pub rp: RenderingPlan,
    pub ep: EventPlan,
    pub handlers: HandlerTable<Msg>,
}
