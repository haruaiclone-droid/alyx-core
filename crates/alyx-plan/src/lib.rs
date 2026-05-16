mod event;
mod handler;
mod rendering;

pub use event::{EventPlan, EventType, ResolvedHitArea};
pub use handler::{HandlerId, HandlerTable};
pub use rendering::{RenderingPlan, RpImage, RpNode, RpText};

#[derive(Clone, Debug, PartialEq)]
pub struct CompilerOutput<Msg> {
    pub rp: RenderingPlan,
    pub ep: EventPlan,
    pub handlers: HandlerTable<Msg>,
}
