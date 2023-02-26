use std::{collections::HashMap, fmt::Display};

use crate::rng::SystemRng;
use rand::{seq::SliceRandom, SeedableRng};
use rustc_hash::FxHashMap;

/// The basic description of an L-System. Used to generate or iterate over its strings.
/// ```
/// # use lindenmayer::LSystem;
/// let axiom = String::from("X");
/// let rules = [('X', "F[X][+DX]-DX"), ('D', "F")];
/// let system = LSystem::new(axiom, &rules);
///
/// let depth = 2;
/// assert_eq!("F[F[X][+DX]-DX][+FF[X][+DX]-DX]-FF[X][+DX]-DX", system.string(depth));
/// ```
#[derive(Debug, Clone)]
pub struct LSystem {
    pub axiom: String,
    pub rules: FxHashMap<char, String>,
}

impl LSystem {
    pub fn new<S: ToString>(axiom: String, rules: &[(char, S)]) -> LSystem {
        let mut map = FxHashMap::default();
        for rule in rules {
            map.insert(rule.0, rule.1.to_string());
        }
        LSystem { axiom, rules: map }
    }

    /// Return the rewrite rule for a given character or None if the character is a terminal
    pub fn get(&self, c: &char) -> Option<&String> {
        self.rules.get(c)
    }

    /// Construct a memory efficient iterator over the L-System at a given depth. This is most useful if one wants to try many different L-Systems or generate them dynamically at runtime.
    pub fn builder(&self, depth: usize) -> LSystemBuilder {
        LSystemBuilder::new(&self, depth)
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

impl Display for LSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = format!("Axiom: {}\nRules:\n", self.axiom);
        for i in self
            .rules
            .iter()
            .map(|(k, v)| format!("  {} => {}\n", k, v))
        {
            out.push_str(&i);
        }
        write!(f, "{}", out)
    }
}

#[derive(Debug, Clone)]
pub struct LSystemBuilder<'a> {
    system: &'a LSystem,
    depth: usize,
    layers: Vec<std::str::Chars<'a>>,
    active_layer: usize,
}

impl<'a> LSystemBuilder<'a> {
    pub fn new(system: &'a LSystem, depth: usize) -> Self {
        let mut layers = vec!["".chars(); depth + 1];
        layers[depth] = system.axiom.chars();

        Self {
            system,
            depth,
            layers,
            active_layer: depth - 1,
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
                        match self.system.get(&c) {
                            None => return Some(c),
                            Some(cs) => self.layers[self.active_layer - 1] = cs.chars(),
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
///     ('X', vec![("F[X][+DX]-DX", 1.0_f32)]),
///     ('D', vec![("F", 2.0), ("FF", 1.0), ("D", 1.0)])
/// ];
///
/// let system = LSystemStochastic::new(axiom, &rules);
///
/// let depth = 2;
/// let seed = Some(19251989);
///
/// assert_eq!("F[F[X][+DX]-DX][+FFF[X][+DX]-DX]-FF[X][+DX]-DX", system.string(depth, seed))
/// ```
#[derive(Debug, Clone)]
pub struct LSystemStochastic {
    pub axiom: String,
    pub rules: HashMap<char, Vec<(String, f32)>>,
}

impl LSystemStochastic {
    pub fn new<S: ToString>(axiom: String, rules: &[(char, Vec<(S, f32)>)]) -> LSystemStochastic {
        let mut map = HashMap::with_capacity(rules.len());
        for rule in rules {
            map.insert(
                rule.0,
                rule.1.iter().map(|(s, n)| (s.to_string(), *n)).collect(),
            );
        }
        LSystemStochastic { axiom, rules: map }
    }

    pub fn get(&self, c: &char, rng: &mut SystemRng) -> Option<&String> {
        if let Some(s) = self.rules.get(c) {
            match s.choose_weighted(rng, |item| item.1) {
                Ok(s) => Some(&s.0),
                Err(e) => panic!("{}", e.to_string()),
            }
        } else {
            None
        }
    }

    /// Construct a memory efficient iterator over the L-System at a given depth. This is most useful if one wants to try many different L-Systems or generate them dynamically at runtime.
    pub fn builder(&self, depth: usize, seed: Option<u64>) -> LSystemBuilderStochastic {
        LSystemBuilderStochastic::new(&self, depth, seed)
    }

    /// Write the L-System, at the given depth, to a String. This is faster than using the builder but may result in a very large allocation.
    pub fn string(&self, depth: usize, seed: Option<u64>) -> String {
        let mut expression = self.axiom.clone();
        let mut rng = match seed {
            Some(n) => SystemRng::seed_from_u64(n),
            None => SystemRng::from_entropy(),
        };
        for _ in 0..depth {
            let mut new = String::new();
            for c in expression.chars() {
                if let Some(s) = self.rules.get(&c) {
                    match s.choose_weighted(&mut rng, |item| item.1) {
                        Ok(pair) => new.push_str(&pair.0),
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

impl Display for LSystemStochastic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = format!("Axiom: {}\nRules", self.axiom);
        let mut rule_strings = Vec::new();
        for (c, replacements) in self.rules.iter() {
            let mut s = String::new();
            if replacements.len() == 1 {
                let rep = &replacements[0].0;
                s.push_str(&format!("\n  {c} => {rep}"));
            } else {
                s.push_str(&format!("\n  {c} =>"));
                let norm: f32 = replacements.iter().map(|x| x.1).sum();
                for (rep, prob) in replacements {
                    let normed_prob = prob / norm;
                    s.push_str(&format!("\n       {rep} ({normed_prob:.2})"));
                }
            }
            rule_strings.push(s);
        }
        rule_strings.sort();
        for s in rule_strings {
            out.push_str(&s)
        }

        write!(f, "{}", out)
    }
}

/// Efficient stochastic L-System constructor that generate symbols one by one rather than building the entire String in memory. This is most useful if one wants to try many different L-Systems or generate them dynamically at runtime.
#[derive(Debug, Clone)]
pub struct LSystemBuilderStochastic<'a> {
    system: &'a LSystemStochastic,
    depth: usize,
    layers: Vec<std::str::Chars<'a>>,
    active_layer: usize,
    rng: SystemRng,
}

impl<'a> LSystemBuilderStochastic<'a> {
    pub fn new(system: &'a LSystemStochastic, depth: usize, seed: Option<u64>) -> Self {
        let mut layers = vec!["".chars(); depth + 1];
        layers[depth] = system.axiom.chars();
        let rng = match seed {
            Some(n) => SystemRng::seed_from_u64(n),
            None => SystemRng::from_entropy(),
        };
        Self {
            system,
            depth,
            layers,
            active_layer: depth - 1,
            rng,
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
                        match self.system.get(&c, &mut self.rng) {
                            None => return Some(c),
                            Some(cs) => self.layers[self.active_layer - 1] = cs.chars(),
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
fn display_test() {
    use crate::builder::LSystem;

    let axiom = String::from("X");
    let rules = [('X', "F[X][+DX]-DX"), ('D', "F")];

    let system = LSystem::new(axiom, &rules);

    assert_eq!(
        "Axiom: X\nRules:\n  X => F[X][+DX]-DX\n  D => F\n",
        system.to_string()
    );
}

#[test]
fn validity_test() {
    use crate::builder::LSystem;

    let axiom = String::from("X");
    let rules = [('X', "F[X][+DX]-DX"), ('D', "F")];

    let system = LSystem::new(axiom, &rules);

    let depth = 3;

    let s = system.string(depth);
    let e = system.builder(depth);

    assert!(e.zip(s.chars()).all(|(a, b)| a == b))
}

#[test]
fn stochastic_display_test() {
    let axiom = String::from("X");
    let rules = [
        ('X', vec![("F[X][+DX]-DX", 1.0_f32)]),
        ('D', vec![("F", 2.0), ("FF", 1.0), ("D", 1.0)]),
    ];

    let system = LSystemStochastic::new(axiom, &rules);
    println!("{}", system)
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

// Previous speed results
// starting to write L-System string
// finished in 314.8995ms
// reading symbols from string
// finished in 92.878ms

// running constructor for L-System builder struct
// finished in 4.2µs
// reading symbols from struct
// finished in 448.6379ms
