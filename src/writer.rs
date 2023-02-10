use std::collections::HashMap;

use rand::{seq::SliceRandom, Rng};

pub fn write_expression(axiom: &str, rules: &HashMap<char, &str>, depth: usize) -> String {
    let mut expression = String::from(axiom);
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

pub fn write_expression_stochastic<R: Rng>(
    axiom: &str,
    rules: &HashMap<char, Vec<(&str, f32)>>,
    depth: usize,
    rng: &mut R,
) -> String {
    let mut expression = String::from(axiom);
    for _ in 0..depth {
        let mut new = String::new();
        for c in expression.chars() {
            if let Some(s) = rules.get(&c) {
                match s.choose_weighted(rng, |item| item.1) {
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
