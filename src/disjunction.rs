use crate::types::State;

use std::collections::VecDeque;

pub struct Disjunction {
    items: VecDeque<Box<dyn Iterator<Item = State>>>,
}

impl Iterator for Disjunction {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        let iter = self.items.pop_front();

        let mut result = None;

        match iter {
            Some(mut iter) => {
                match iter.next() {
                    Some(res) => {
                        result = Some(res);
                        self.items.push_back(iter);
                    }
                    None => {
                        // potentially bad for performance, idk tho
                        return self.next();
                    }
                }
            }
            None => (),
        };

        return result;
    }
}

impl Disjunction {
    pub fn new(iters: VecDeque<Box<dyn Iterator<Item = State>>>) -> Self {
        Disjunction { items: iters }
    }
}
