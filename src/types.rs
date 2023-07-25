use im::HashMap;
use std::sync::Arc;

pub type StateIter = Box<dyn Iterator<Item = State>>;
pub type StateToIterFn = Arc<dyn Fn(State) -> Box<dyn Iterator<Item = State>>>;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum Variable {
    Literal(Data),
    Variable(Arc<str>),
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum Data {
    Str(Arc<str>),
    Int(i64),
    UInt(u64),
    Bool(bool),
}

#[derive(Clone, PartialEq, Debug)]
pub struct State {
    pub(crate) map: HashMap<Arc<str>, Variable>,
}

impl State {
    pub fn update(&self, k: Arc<str>, v: Variable) -> Self {
        State {
            map: self.map.update(k, v),
        }
    }

    pub fn new() -> Self {
        State {
            map: HashMap::new(),
        }
    }

    pub fn from_map(map: HashMap<Arc<str>, Variable>) -> Self {
        State { map }
    }
}
