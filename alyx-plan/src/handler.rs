#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HandlerId(pub usize);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct HandlerTable<Msg> {
    handlers: Vec<Msg>,
}

impl<Msg> HandlerTable<Msg> {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn insert(&mut self, msg: Msg) -> HandlerId {
        let id = HandlerId(self.handlers.len());
        self.handlers.push(msg);
        id
    }

    pub fn get(&self, id: HandlerId) -> Option<&Msg> {
        self.handlers.get(id.0)
    }

    pub fn len(&self) -> usize {
        self.handlers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.handlers.is_empty()
    }
}
