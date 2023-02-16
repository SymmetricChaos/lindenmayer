use std::collections::HashMap;

//use rand::{seq::SliceRandom, Rng};

/// Apply the rules the number of times specified and return the resulting String.
pub fn write_lsystem(axiom: &str, rules: &HashMap<char, &str>, depth: usize) -> String {
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

/// Apply the rules the number of times specified and return a Vec with the String generated at each step, along with the axiom
pub fn write_lsystem_sequence(
    axiom: &str,
    rules: &HashMap<char, &str>,
    depth: usize,
) -> Vec<String> {
    let mut out = Vec::with_capacity(depth + 1);
    out.push(axiom.to_string());
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
        out.push(new.clone());
        expression = new;
    }
    out
}

#[test]
fn validation_test() {
    use std::collections::HashMap;

    let axiom = "A";
    let rules = HashMap::from([('A', "AB"), ('B', "A")]);
    let depth = 5;

    assert_eq!("ABAABABAABAAB", write_lsystem(axiom, &rules, depth))
}

#[test]
fn size_test() {
    use std::collections::HashMap;

    let axiom = "X";
    let rules = HashMap::from([('X', "F[X][+DX]-DX"), ('D', "F")]);
    let depth = 10;

    let s = write_lsystem(axiom, &rules, depth);

    println!("depth: {}\n{} kilobytes", depth, s.len() / 1000)
}

// pub fn write_lsystem_stochastic<R: Rng>(
//     axiom: &str,
//     rules: &HashMap<char, Vec<(&str, f32)>>,
//     depth: usize,
//     rng: &mut R,
// ) -> String {
//     let mut expression = String::from(axiom);
//     for _ in 0..depth {
//         let mut new = String::new();
//         for c in expression.chars() {
//             if let Some(s) = rules.get(&c) {
//                 match s.choose_weighted(rng, |item| item.1) {
//                     Ok(s) => new.push_str(s.0),
//                     Err(e) => panic!("{}", e.to_string()),
//                 }
//             } else {
//                 new.push(c)
//             }
//         }
//         expression = new;
//     }
//     expression
// }
