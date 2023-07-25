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
    use crate::functions::{eq, or, unify};
    use crate::types::{Data, State, StateIter, Variable};

    use std::collections::VecDeque;
    use std::sync::Arc;

    use im::hashmap;

    #[test]
    fn unify_test() {
        let st = State::new();

        let unif = unify(
            &st,
            &Variable::Variable("x".into()),
            &Variable::Variable("y".into()),
        )
        .next()
        .unwrap();

        let unif = unify(
            &unif,
            &Variable::Variable("y".into()),
            &Variable::Literal(Data::Int(5)),
        )
        .next()
        .unwrap();

        assert_eq!(
            unif,
            State::from_map(
                hashmap! {"x".into() => Variable::Variable("y".into()), "y".into()=> Variable::Literal(Data::Int(5)),}
            )
        )
    }

    #[test]
    fn literal_test() {
        let st = State::new();

        let mut unif = unify(
            &st,
            &Variable::Literal(Data::Int(5)),
            &Variable::Literal(Data::Int(6)),
        );

        assert_eq!(unif.next(), None);
    }

    #[test]
    fn disjunction_test() {
        let st1 = State::from_map(hashmap! {"x".into() => Variable::Literal(Data::Int(5)) });
        let st2 = State::from_map(hashmap! {"x".into() => Variable::Variable("y".into()) });

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

        let st = State::from_map(hashmap! {"x".into() => Variable::Literal(Data::Int(1))});
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
                &Variable::Variable("x".into()),
                &Variable::Literal(Data::Int(1)),
            )),
            eq(
                Variable::Variable("y".into()),
                Variable::Literal(Data::Int(1)),
            ),
        );

        assert_eq!(
            iter_fn.next(),
            Some(State::from_map(
                hashmap! { "x".into() => Variable::Literal(Data::Int(1)), "y".into() => Variable::Literal(Data::Int(1))}
            ))
        );

        assert_eq!(iter_fn.next(), None);
    }

    #[test]
    fn and_test() {}
}
