use crate::types::{State, StateIter, StateToIterFn};
use std::collections::VecDeque;

pub struct Conjunction {
    main_iterator: StateIter,
    f: StateToIterFn,
    curr_iters: VecDeque<StateIter>,
    next_iters: VecDeque<StateIter>,
}

impl Iterator for Conjunction {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_iters.is_empty() {
            let next_gen = self.main_iterator.next();

            match next_gen {
                Some(gen) => {
                    self.next_iters.push_front((self.f)(gen));
                }
                None => (),
            }

            std::mem::swap(&mut self.curr_iters, &mut self.next_iters);
        }

        if self.curr_iters.is_empty() {
            return None;
        }

        let mut curr_gen = self.curr_iters.pop_front().unwrap();

        let mut result = None;

        let next_el = curr_gen.next();

        match next_el {
            Some(state) => {
                result = Some(state);
                self.next_iters.push_back(curr_gen);
            }
            None => (),
        }

        result
    }
}

impl Conjunction {
    pub fn new(iters: StateIter, f: StateToIterFn) -> Self {
        Conjunction {
            main_iterator: iters,
            f,
            curr_iters: VecDeque::new(),
            next_iters: VecDeque::new(),
        }
    }
}
