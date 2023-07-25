use crate::disjunction::Disjunction;
use crate::types::{State, StateToIterFn, Variable};
use std::collections::VecDeque;
use std::sync::Arc;

pub fn walk(state: &State, x: &Variable) -> Variable {
    let mut last = x;
    loop {
        match last {
            Variable::Variable(s) => {
                if state.map.contains_key(s.as_ref()) {
                    last = &state.map[s.as_ref()];
                } else {
                    break;
                }
            }
            Variable::Literal(_) => {
                break;
            }
        }
    }

    return last.clone();
}

pub fn unify(state: &State, l: &Variable, r: &Variable) -> Option<State> {
    let walked_l = walk(&state, &l);
    let walked_r = walk(&state, &r);

    match (walked_l, walked_r) {
        (Variable::Variable(name), Variable::Variable(_)) => Some(state.update(name, r.clone())),
        (Variable::Variable(name), Variable::Literal(_)) => Some(state.update(name, r.clone())),
        (Variable::Literal(_), Variable::Variable(name)) => Some(state.update(name, l.clone())),
        (Variable::Literal(l_val), Variable::Literal(r_val)) => {
            if l_val != r_val {
                None
            } else {
                Some(state.clone())
            }
        }
    }
}

pub fn or(f: StateToIterFn, g: StateToIterFn) -> StateToIterFn {
    let _or = move |state: State| -> Box<dyn Iterator<Item = State>> {
        Box::new(Disjunction::new(VecDeque::from([
            f(state.clone()),
            g(state),
        ])))
    };

    return Arc::new(_or);
}
