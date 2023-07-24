mod functions;
mod parser;
mod types;

#[cfg(test)]
mod tests {
    use crate::functions::unify;
    use crate::types::{Data, State, Variable};
    use im::{hashmap, HashMap};

    #[test]
    fn unify_test() {
        let st = State::new();

        let unif = unify(&st, &Variable::Variable("x".into()), &Variable::Variable("y".into())).unwrap();
        let unif = unify(&unif, &Variable::Variable("y".into()), &Variable::Literal(Data::Int(5))).unwrap();

        assert_eq!(unif, State::from_map(hashmap! {"x".into() => Variable::Variable("y".into()), "y".into()=> Variable::Literal(Data::Int(5)),}))
    }
}
