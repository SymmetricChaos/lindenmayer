use std::collections::HashMap;

use rand::{seq::SliceRandom, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

enum OneOrMany<'a> {
    One(char),
    Many(std::str::Chars<'a>),
}

/// Efficient L-System constructor that generate symbols one by one rather than building the entire String in memory. This is most useful if one wants to try many different L-Systems or generate them dynamically at runtime.
/// ```
/// # use std::collections::HashMap;
/// # use lindenmayer::builder::LSystemBuilder;
/// let axiom = "X";
/// let rules = HashMap::from([('X', "F[X][+DX]-DX"), ('D', "F")]);
/// let depth = 2;
/// let builder = LSystemBuilder::new(axiom, &rules, depth);
/// assert_eq!("F[F[X][+DX]-DX][+FF[X][+DX]-DX]-FF[X][+DX]-DX", builder.collect::<String>());
/// ```
#[derive(Debug, Clone)]
pub struct LSystemBuilder<'a> {
    // Using FxHashMap and improved performance by about 15% but was a pain to make work
    rules: &'a HashMap<char, &'a str>,
    depth: usize,
    layers: Vec<std::str::Chars<'a>>,
    active_layer: usize,
}

impl<'a> LSystemBuilder<'a> {
    pub fn new(axiom: &'a str, rules: &'a HashMap<char, &'a str>, depth: usize) -> Self {
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
        if let Some(s) = self.rules.get(c) {
            OneOrMany::Many(s.chars())
        } else {
            OneOrMany::One(*c)
        }
        // // Stochastic version
        // if let Some(s) = self.rules.get(&c) {
        //     match s.choose_weighted(&mut self.rng, |item| item.1) {
        //         Ok(s) => OneOrMany::Many(s.0.chars()),
        //         Err(e) => panic!("{}", e.to_string()),
        //     }
        // } else {
        //     OneOrMany::One(*c)
        // }
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
                    // If it is a terminal symbol then we can short circuit and just return it
                    // Otherwise load the iterator before it and move the pointer to that position
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

/// Efficient stochastic L-System constructor that generate symbols one by one rather than building the entire String in memory. This is most useful if one wants to try many different L-Systems or generate them dynamically at runtime.
/// ```
/// # use std::collections::HashMap;
/// # use lindenmayer::builder::LSystemBuilderStochastic;
/// use rand::SeedableRng;
/// use rand_xoshiro::Xoshiro256PlusPlus;
/// let axiom = "X";
/// let rules = HashMap::from([
///     ('X', vec![("F[X][+DX]-DX", 1.0)]),
///     ('D', vec![("F", 2.0), ("FF", 1.0), ("D", 1.0)])
/// ]);
/// let depth = 2;
/// let rng = Some(Xoshiro256PlusPlus ::seed_from_u64(19251989));
/// let builder = LSystemBuilderStochastic::new(axiom, &rules, depth, rng);
/// ```
#[derive(Debug, Clone)]
pub struct LSystemBuilderStochastic<'a> {
    rules: &'a HashMap<char, Vec<(&'a str, f32)>>,
    depth: usize,
    layers: Vec<std::str::Chars<'a>>,
    active_layer: usize,
    rng: Xoshiro256PlusPlus,
}

impl<'a> LSystemBuilderStochastic<'a> {
    pub fn new(
        axiom: &'a str,
        rules: &'a HashMap<char, Vec<(&'a str, f32)>>,
        depth: usize,
        seed: Option<u64>,
    ) -> Self {
        let mut layers = vec!["".chars(); depth + 1];
        layers[depth] = axiom.chars();
        let rng = match seed {
            Some(n) => Xoshiro256PlusPlus::seed_from_u64(n),
            None => Xoshiro256PlusPlus::from_entropy(),
        };
        Self {
            rules,
            depth,
            layers,
            active_layer: depth - 1,
            rng,
        }
    }

    fn chars_from_rules(&mut self, c: &char) -> OneOrMany<'a> {
        if let Some(s) = self.rules.get(c) {
            match s.choose_weighted(&mut self.rng, |item| item.1) {
                Ok(s) => OneOrMany::Many(s.0.chars()),
                Err(e) => panic!("{}", e.to_string()),
            }
        } else {
            OneOrMany::One(*c)
        }
    }
}

impl<'a> Iterator for LSystemBuilderStochastic<'_> {
    type Item = char;

    // Same logic as for LSystemBuilder
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.active_layer > self.depth {
                return None;
            } else {
                if let Some(c) = self.layers[0].next() {
                    return Some(c);
                } else {
                    if let Some(c) = self.layers[self.active_layer].next() {
                        match self.chars_from_rules(&c) {
                            OneOrMany::One(c) => return Some(c),
                            OneOrMany::Many(cs) => self.layers[self.active_layer - 1] = cs,
                        }
                        self.active_layer -= 1
                    } else {
                        self.active_layer += 1
                    }
                }
            }
        }
    }
}

#[test]
fn validity_test() {
    use std::collections::HashMap;

    use crate::{builder::LSystemBuilder, writer::write_lsystem};

    let axiom = "X";
    let rules = HashMap::from([('X', "F[X][+DX]-DX"), ('D', "F")]);
    let depth = 3;

    let s = write_lsystem(axiom, &rules, depth);
    let e = LSystemBuilder::new(axiom, &rules, depth);

    assert!(e.zip(s.chars()).all(|(a, b)| a == b))
}

// #[test]
// fn time_test() {
//     use std::collections::HashMap;

//     use crate::{builder::LSystemBuilder, writer::write_lsystem};
//     use std::time::Instant;

//     let axiom = "X";
//     let rules = HashMap::from([('X', "F[X][+DX]-DX"), ('D', "F")]);
//     let depth = 12;

//     println!("starting to write L-System string");
//     let t0 = Instant::now();
//     let s = write_lsystem(axiom, &rules, depth);
//     println!("finished in {:?}", Instant::now() - t0);
//     println!("reading symbols from string");
//     let t0 = Instant::now();
//     for _ in s.chars() {
//         continue;
//     }
//     println!("finished in {:?}\n\n", Instant::now() - t0);

//     println!("running constructor for L-System builder struct");
//     let t0 = Instant::now();
//     let e = LSystemBuilder::new(axiom, &rules, depth);
//     println!("finished in {:?}", Instant::now() - t0);
//     println!("reading symbols from struct");
//     let t0 = Instant::now();
//     for _ in e {
//         continue;
//     }
//     println!("finished in {:?}", Instant::now() - t0);
// }
