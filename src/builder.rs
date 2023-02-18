use std::collections::HashMap;

use crate::rng::InnerRng;
use rand::{seq::SliceRandom, SeedableRng};
use rustc_hash::FxHashMap;

enum OneOrMany<'a> {
    One(char),
    Many(std::str::Chars<'a>),
}

/// The basic description of an L-System. Used to generate or iterate over its strings.
/// ```
/// # use lindenmayer::LSystem;
/// let axiom = String::from("X");
/// let rules = [('X', "F[X][+DX]-DX"), ('D', "F")];
/// let depth = 2;
/// let system = LSystem::new(axiom, &rules);
/// assert_eq!("F[F[X][+DX]-DX][+FF[X][+DX]-DX]-FF[X][+DX]-DX", system.builder(depth).collect::<String>());
/// assert_eq!("F[F[X][+DX]-DX][+FF[X][+DX]-DX]-FF[X][+DX]-DX", system.string(depth));
/// ```
pub struct LSystem<'a> {
    axiom: String,
    rules: FxHashMap<char, &'a str>,
}

impl<'a> LSystem<'_> {
    pub fn new(axiom: String, rules: &[(char, &'a str)]) -> LSystem<'a> {
        let mut map = FxHashMap::default();
        for rule in rules {
            map.insert(rule.0, rule.1);
        }
        LSystem { axiom, rules: map }
    }

    /// Construct a memory efficient iterator over the L-System at a given depth. This is most useful if one wants to try many different L-Systems or generate them dynamically at runtime.
    pub fn builder(&self, depth: usize) -> LSystemBuilder {
        LSystemBuilder::new(&self.axiom, &self.rules, depth)
    }

    /// Write the L-System, at the given depth, to a String. This is faster than using the builder but may result in a very large allocation.
    pub fn string(&self, depth: usize) -> String {
        let mut expression = self.axiom.clone();
        for _ in 0..depth {
            let mut new = String::new();
            for c in expression.chars() {
                if let Some(s) = self.rules.get(&c) {
                    new.push_str(s)
                } else {
                    new.push(c)
                }
            }
            expression = new;
        }
        expression
    }
}

#[derive(Debug, Clone)]
pub struct LSystemBuilder<'a> {
    rules: &'a FxHashMap<char, &'a str>,
    depth: usize,
    layers: Vec<std::str::Chars<'a>>,
    active_layer: usize,
}

impl<'a> LSystemBuilder<'a> {
    pub fn new(axiom: &'a str, rules: &'a FxHashMap<char, &'a str>, depth: usize) -> Self {
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

/// The basic description of a stochastic L-System. Used to generate or iterate over its strings.
/// Warning: Using the same seed for the .builder() and .string() methods does not guarantee identical results because the order of iteration is different.
/// ```
/// # use lindenmayer::builder::LSystemStochastic;
/// let axiom = String::from("X");
/// let rules = [
///     ('X', &vec![("F[X][+DX]-DX", 1.0)]),
///     ('D', &vec![("F", 2.0), ("FF", 1.0), ("D", 1.0)])
/// ];
/// let depth = 2;
/// let seed = Some(19251989);
///
/// let system = LSystemStochastic::new(axiom, &rules);
/// let builder = system.builder(depth, seed);
/// let string = system.string(depth, seed);
/// ```
pub struct LSystemStochastic<'a> {
    axiom: String,
    rules: HashMap<char, &'a Vec<(&'a str, f32)>>,
}

impl<'a> LSystemStochastic<'_> {
    pub fn new(axiom: String, rules: &[(char, &'a Vec<(&'a str, f32)>)]) -> LSystemStochastic<'a> {
        let mut map = HashMap::with_capacity(rules.len());
        for rule in rules {
            map.insert(rule.0, rule.1);
        }
        LSystemStochastic { axiom, rules: map }
    }

    /// Construct a memory efficient iterator over the L-System at a given depth. This is most useful if one wants to try many different L-Systems or generate them dynamically at runtime.
    pub fn builder(&self, depth: usize, seed: Option<u64>) -> LSystemBuilderStochastic {
        LSystemBuilderStochastic::new(&self.axiom, &self.rules, depth, seed)
    }

    /// Write the L-System, at the given depth, to a String. This is faster than using the builder but may result in a very large allocation.
    pub fn string(&self, depth: usize, seed: Option<u64>) -> String {
        let mut expression = self.axiom.clone();
        let mut rng = match seed {
            Some(n) => InnerRng::seed_from_u64(n),
            None => InnerRng::from_entropy(),
        };
        for _ in 0..depth {
            let mut new = String::new();
            for c in expression.chars() {
                if let Some(s) = self.rules.get(&c) {
                    match s.choose_weighted(&mut rng, |item| item.1) {
                        Ok(pair) => new.push_str(pair.0),
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
}

/// Efficient stochastic L-System constructor that generate symbols one by one rather than building the entire String in memory. This is most useful if one wants to try many different L-Systems or generate them dynamically at runtime.
#[derive(Debug, Clone)]
pub struct LSystemBuilderStochastic<'a> {
    rules: &'a HashMap<char, &'a Vec<(&'a str, f32)>>,
    depth: usize,
    layers: Vec<std::str::Chars<'a>>,
    active_layer: usize,
    rng: InnerRng,
}

impl<'a> LSystemBuilderStochastic<'a> {
    pub fn new(
        axiom: &'a str,
        rules: &'a HashMap<char, &'a Vec<(&'a str, f32)>>,
        depth: usize,
        seed: Option<u64>,
    ) -> Self {
        let mut layers = vec!["".chars(); depth + 1];
        layers[depth] = axiom.chars();
        let rng = match seed {
            Some(n) => InnerRng::seed_from_u64(n),
            None => InnerRng::from_entropy(),
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
    use crate::builder::LSystem;

    let axiom = String::from("X");
    let rules = [('X', "F[X][+DX]-DX"), ('D', "F")];
    let depth = 3;

    let system = LSystem::new(axiom, &rules);

    let s = system.string(depth);
    let e = system.builder(depth);

    assert!(e.zip(s.chars()).all(|(a, b)| a == b))
}

#[test]
fn stochastic_test() {
    let axiom = String::from("X");
    let rules = [
        ('X', &vec![("F[X][+DX]-DX", 1.0)]),
        ('D', &vec![("F", 2.0), ("FF", 1.0), ("D", 1.0)]),
    ];
    let depth = 2;
    let seed = Some(19251989);

    let system = LSystemStochastic::new(axiom, &rules);
    println!("{}", system.string(depth, seed));
}

// #[test]
// fn time_test() {
//     use crate::builder::LSystem;
//     use std::time::Instant;

//     let axiom = String::from("X");
//     let rules = [('X', "F[X][+DX]-DX"), ('D', "F")];
//     let depth = 12;

//     let system = LSystem::new(axiom, &rules);

//     println!("starting to write L-System string");
//     let t0 = Instant::now();
//     let s = system.string(depth);
//     println!("finished in {:?}", Instant::now() - t0);
//     println!("reading symbols from string");
//     let t0 = Instant::now();
//     for _ in s.chars() {
//         continue;
//     }
//     println!("finished in {:?}\n\n", Instant::now() - t0);

//     println!("running constructor for L-System builder struct");
//     let t0 = Instant::now();
//     let e = system.builder(depth);
//     println!("finished in {:?}", Instant::now() - t0);
//     println!("reading symbols from struct");
//     let t0 = Instant::now();
//     for _ in e {
//         continue;
//     }
//     println!("finished in {:?}", Instant::now() - t0);
// }
