use std::collections::HashMap;

enum OneOrMany<'a> {
    One(char),
    Many(std::str::Chars<'a>),
}

/// Memory efficient L-System constructor.
/// ```
/// # use std::collections::HashMap;
/// # use lindenmayer::builder::LSystemBuilder;
/// let builder = LSystemBuilder::new("X", HashMap::from([('X', "F[X][+DX]-DX"), ('D', "F")]), 2);
/// assert_eq!(builder.collect::<String>(),"F[F[X][+DX]-DX][+FF[X][+DX]-DX]-FF[X][+DX]-DX");
/// ```
#[derive(Debug, Clone)]
pub struct LSystemBuilder<'a> {
    rules: HashMap<char, &'a str>,
    depth: usize,
    layers: Vec<std::str::Chars<'a>>,
    active_layer: usize,
}

impl<'a> LSystemBuilder<'a> {
    pub fn new(axiom: &'a str, rules: HashMap<char, &'a str>, depth: usize) -> Self {
        let mut layers = vec!["".chars(); depth + 1];
        layers[depth] = axiom.chars();

        Self {
            rules,
            depth,
            layers,
            active_layer: depth - 1,
        }
    }

    fn chars_from_rules(&self, c: &char) -> OneOrMany<'a> {
        if let Some(s) = self.rules.get(&c) {
            OneOrMany::Many(s.chars())
        } else {
            OneOrMany::One(*c)
        }
    }
}

impl<'a> Iterator for LSystemBuilder<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // If the pointer has moved too far then we're out of characters
            if self.active_layer > self.depth {
                return None;
            } else {
                // If the first iterator has more characters use them
                if let Some(c) = self.layers[0].next() {
                    return Some(c);
                } else {
                    // Otherwise check the iterator pointed to amd try to get the next character
                    // If it is a ternimal symbol then we can short circuit and just return it
                    // Otherwise load the iterator before it and move the pointer back
                    if let Some(c) = self.layers[self.active_layer].next() {
                        match self.chars_from_rules(&c) {
                            OneOrMany::One(c) => return Some(c),
                            OneOrMany::Many(cs) => self.layers[self.active_layer - 1] = cs,
                        }
                        self.active_layer -= 1
                    // If the iterator is empty move the pointer up
                    } else {
                        self.active_layer += 1
                    }
                }
            }
        }
    }
}

// /// Memory efficient stochastic L-System constructor.
// pub struct LSystemBuilderStochastic<'a> {
//     rules: HashMap<char, Vec<(&'a str, f32)>>,
//     depth: usize,
//     layers: Vec<std::str::Chars<'a>>,
//     rng: Xoroshiro128StarStar,
// }

// impl<'a> LSystemBuilderStochastic<'a> {
//     pub fn new(axiom: &'a str, rules: HashMap<char, Vec<(&'a str, f32)>>, depth: usize) -> Self {
//         let mut layers = vec!["".chars(); depth + 1];
//         layers[depth] = axiom.chars();
//         let rng = Xoroshiro128StarStar::from_entropy();

//         Self {
//             rules,
//             depth,
//             layers,
//             rng,
//         }
//     }

//     pub fn new_with_seed_from_u64(
//         axiom: &'a str,
//         rules: HashMap<char, Vec<(&'a str, f32)>>,
//         depth: usize,
//         seed: u64,
//     ) -> Self {
//         let mut layers = vec!["".chars(); depth + 1];
//         layers[depth] = axiom.chars();
//         let rng = Xoroshiro128StarStar::seed_from_u64(seed);

//         Self {
//             rules,
//             depth,
//             layers,
//             rng,
//         }
//     }

//     fn chars_from_rules(&mut self, c: &char) -> OneOrMany<'a> {
//         if let Some(s) = self.rules.get(&c) {
//             match s.choose_weighted(&mut self.rng, |item| item.1) {
//                 Ok(s) => OneOrMany::Many(s.0.chars()),
//                 Err(e) => panic!("{}", e.to_string()),
//             }
//         } else {
//             OneOrMany::One(*c)
//         }
//     }
// }

// impl<'a> Iterator for LSystemBuilderStochastic<'_> {
//     type Item = char;

//     fn next(&mut self) -> Option<Self::Item> {
//         let mut self.active_layer = 1_usize;

//         loop {
//             if self.active_layer > self.depth {
//                 return None;
//             } else {
//                 if let Some(c) = self.layers[0].next() {
//                     return Some(c);
//                 } else {
//                     if let Some(c) = self.layers[self.active_layer].next() {
//                         match self.chars_from_rules(&c) {
//                             OneOrMany::One(c) => return Some(c),
//                             OneOrMany::Many(cs) => self.layers[self.active_layer - 1] = cs,
//                         }
//                         self.active_layer -= 1
//                     } else {
//                         self.active_layer += 1
//                     }
//                 }
//             }
//         }
//     }
// }

#[test]
fn validity_test() {
    use std::collections::HashMap;

    use crate::{builder::LSystemBuilder, writer::write_lsystem};

    let axiom = "X";
    let rules = HashMap::from([('X', "F[X][+DX]-DX"), ('D', "F")]);
    let depth = 3;

    let s = write_lsystem(axiom, &rules, depth);
    let e = LSystemBuilder::new(axiom, rules, depth);

    assert!(e.zip(s.chars()).all(|(a, b)| a == b))
}

// #[test]
// fn validity_test_stochastic() {
//     use std::collections::HashMap;

//     use crate::{builder::LSystemBuilderStochastic, writer::write_expression_stochastic};

//     let axiom = "X";
//     let rules = HashMap::from([
//         ('X', vec![("F[X][+DX]-DX", 1.0)]),
//         ('D', vec![("F", 2.0), ("D", 1.0), ("FF", 1.0)]),
//     ]);
//     let depth = 3;
//     let seed = 35453;

//     let s = write_expression_stochastic(
//         axiom,
//         &rules,
//         depth,
//         &mut Xoroshiro128StarStar::seed_from_u64(seed),
//     );
//     let e = LSystemBuilderStochastic::new_with_seed_from_u64(axiom, rules, depth, seed);

//     println!("{}\n", s);
//     println!("{}", e.collect::<String>());
//     //assert!(e.zip(s.chars()).all(|(a, b)| a == b))
// }
