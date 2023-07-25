#![allow(dead_code)]

mod conjunction;
mod disjunction;
mod functions;
mod parser;
mod types;

#[cfg(test)]
mod tests {
    use crate::conjunction::Conjunction;
    use crate::disjunction::Disjunction;
    use crate::functions::{and, eq, or, unify};
    use crate::types::Variable::{Literal, VarName};
    use crate::types::{Data, State, StateIter};

    use std::collections::VecDeque;
    use std::sync::Arc;

    use im::hashmap;

    #[test]
    fn unify_test() {
        let st = State::new();

        let unif = unify(&st, &VarName("x".into()), &VarName("y".into()))
            .next()
            .unwrap();

        let unif = unify(&unif, &VarName("y".into()), &Literal(Data::Int(5)))
            .next()
            .unwrap();

        assert_eq!(
            unif,
            State::from_map(
                hashmap! {"x".into() => VarName("y".into()), "y".into()=> Literal(Data::Int(5)),}
            )
        )
    }

    #[test]
    fn literal_test() {
        let st = State::new();

        let mut unif = unify(&st, &Literal(Data::Int(5)), &Literal(Data::Int(6)));

        assert_eq!(unif.next(), None);
    }

    #[test]
    fn disjunction_test() {
        let st1 = State::from_map(hashmap! {"x".into() => Literal(Data::Int(5)) });
        let st2 = State::from_map(hashmap! {"x".into() => VarName("y".into()) });

        let iter1 = vec![st1.clone(), st2.clone()];
        let iter2 = vec![st1.clone(), st2.clone(), st2.clone()];

        let mut iters = VecDeque::new();
        iters.push_back(Box::new(iter1.into_iter()) as _);
        iters.push_back(Box::new(iter2.into_iter()) as _);

        let mut disjunction = Disjunction::new(iters);

        assert_eq!(disjunction.next(), Some(st1.clone()));
        assert_eq!(disjunction.next(), Some(st1));

        assert_eq!(disjunction.next(), Some(st2.clone()));
        assert_eq!(disjunction.next(), Some(st2.clone()));
        assert_eq!(disjunction.next(), Some(st2));

        assert_eq!(disjunction.next(), None);
    }

    #[test]
    fn or_test() {
        fn iter_1(state: State) -> StateIter {
            Box::new(vec![state.clone(), state].into_iter())
        }

        fn iter_2(_state: State) -> StateIter {
            Box::new(vec![State::new(), State::new(), State::new()].into_iter())
        }

        let iter_3_fn = or(Arc::new(iter_1), Arc::new(iter_2));

        let st = State::from_map(hashmap! {"x".into() => Literal(Data::Int(1))});
        let mut iter_3 = iter_3_fn(st.clone());

        assert_eq!(iter_3.next(), Some(st.clone()));
        assert_eq!(iter_3.next(), Some(State::new()));
        assert_eq!(iter_3.next(), Some(st.clone()));
        assert_eq!(iter_3.next(), Some(State::new()));
        assert_eq!(iter_3.next(), Some(State::new()));

        assert_eq!(iter_3.next(), None);
    }

    #[test]
    fn conjunction_test() {
        let mut iter_fn = Conjunction::new(
            Box::new(unify(
                &State::new(),
                &VarName("x".into()),
                &Literal(Data::Int(1)),
            )),
            eq(VarName("y".into()), Literal(Data::Int(1))),
        );

        assert_eq!(
            iter_fn.next(),
            Some(State::from_map(
                hashmap! { "x".into() => Literal(Data::Int(1)), "y".into() => Literal(Data::Int(1))}
            ))
        );

        assert_eq!(iter_fn.next(), None);
    }

    #[test]
    fn and_test() {
        let and_fn = and(
            eq(VarName("x".into()), Literal(Data::Int(2))),
            eq(VarName("y".into()), Literal(Data::Int(5))),
        );
        let mut res_iter = and_fn(State::new());

        assert_eq!(
            res_iter.next(),
            Some(State::from_map(
                hashmap! { "x".into() => Literal(Data::Int(2)), "y".into() => Literal(Data::Int(5))}
            ))
        );

        assert_eq!(res_iter.next(), None);
    }
}
