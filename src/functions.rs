use crate::conjunction::Conjunction;
use crate::disjunction::Disjunction;
use crate::types::{State, StateIter, StateToIterFn, Variable};

use std::collections::VecDeque;
use std::sync::Arc;

pub fn walk(state: &State, x: &Variable) -> Variable {
    let mut last = x;
    loop {
        match last {
            Variable::VarName(s) => {
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

pub fn unify(state: &State, l: &Variable, r: &Variable) -> StateIter {
    let walked_l = walk(&state, &l);
    let walked_r = walk(&state, &r);

    match (walked_l, walked_r) {
        (Variable::VarName(name), Variable::VarName(_)) => {
            Box::new(std::iter::once(state.update(name, r.clone())))
        }
        (Variable::VarName(name), Variable::Literal(_)) => {
            Box::new(std::iter::once(state.update(name, r.clone())))
        }
        (Variable::Literal(_), Variable::VarName(name)) => {
            Box::new(std::iter::once(state.update(name, l.clone())))
        }
        (Variable::Literal(l_val), Variable::Literal(r_val)) => {
            if l_val != r_val {
                Box::new(std::iter::empty())
            } else {
                Box::new(std::iter::once(state.clone()))
            }
        }
    }
}

pub fn eq(x: Variable, y: Variable) -> StateToIterFn {
    let _eq = move |state: State| -> StateIter { Box::new(unify(&state, &x, &y)) };

    return Arc::new(_eq);
}

pub fn or(f: StateToIterFn, g: StateToIterFn) -> StateToIterFn {
    let _or = move |state: State| -> StateIter {
        Box::new(Disjunction::new(VecDeque::from([
            f(state.clone()),
            g(state),
        ])))
    };

    return Arc::new(_or);
}

pub fn and(f: StateToIterFn, g: StateToIterFn) -> StateToIterFn {
    let _and = move |state: State| -> StateIter { Box::new(Conjunction::new(f(state), g.clone())) };

    return Arc::new(_and);
}
