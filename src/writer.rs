use std::collections::HashMap;

use rand::{seq::SliceRandom, thread_rng};

pub fn write_expression(axiom: String, rules: HashMap<char, &str>, depth: usize) -> String {
    let mut expression = axiom;
    for _ in 0..depth {
        let mut new = String::new();
        for c in expression.chars() {
            if let Some(s) = rules.get(&c) {
                new.push_str(s)
            } else {
                new.push(c)
            }
        }
        expression = new;
    }
    expression
}

pub fn write_expression_stochastic(
    axiom: String,
    rules: HashMap<char, Vec<(&str, f32)>>,
    depth: usize,
) -> String {
    let mut expression = axiom;
    let mut rng = thread_rng();
    for _ in 0..depth {
        let mut new = String::new();
        for c in expression.chars() {
            if let Some(s) = rules.get(&c) {
                match s.choose_weighted(&mut rng, |item| item.1) {
                    Ok(s) => new.push_str(s.0),
                    Err(e) => panic!("{}", e.to_string()),
                }
            } else {
                new.push(c)
            }
        }
        expression = new;
    }
    expression
}
