use std::collections::HashMap;

use rand::{seq::SliceRandom, thread_rng};

pub enum OneOrMany<'a> {
    One(char),
    Many(std::str::Chars<'a>),
}

pub struct LSystemBuilder<'a> {
    rules: HashMap<char, &'a str>,
    depth: usize,
    layers: Vec<std::str::Chars<'a>>,
}

impl<'a> LSystemBuilder<'a> {
    pub fn new(axiom: &'a str, rules: HashMap<char, &'a str>, depth: usize) -> Self {
        let mut layers = vec!["".chars(); depth + 1];
        layers[depth] = axiom.chars();

        Self {
            rules,
            depth,
            layers,
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
        let mut ptr = 1_usize;

        loop {
            // If the pointer has moved too far then we're out of characters
            if ptr > self.depth {
                return None;
            } else {
                // If the first iterator has more characters use them
                if let Some(c) = self.layers[0].next() {
                    return Some(c);
                } else {
                    // Otherwise check the iterator pointed to amd try to get the next character
                    // If it is a ternimal symbol then we can short circuit and just return it
                    // Otherwise load the iterator before it and move the pointer back
                    if let Some(c) = self.layers[ptr].next() {
                        match self.chars_from_rules(&c) {
                            OneOrMany::One(c) => return Some(c),
                            OneOrMany::Many(cs) => self.layers[ptr - 1] = cs,
                        }
                        ptr -= 1
                    // If the iterator is empty move the pointer up
                    } else {
                        ptr += 1
                    }
                }
            }
        }
    }
}

pub struct LSystemBuilderStochastic<'a> {
    rules: HashMap<char, Vec<(&'a str, f32)>>,
    depth: usize,
    layers: Vec<std::str::Chars<'a>>,
}

impl<'a> LSystemBuilderStochastic<'a> {
    pub fn new(axiom: &'a str, rules: HashMap<char, Vec<(&'a str, f32)>>, depth: usize) -> Self {
        let mut layers = vec!["".chars(); depth + 1];
        layers[depth] = axiom.chars();

        Self {
            rules,
            depth,
            layers,
        }
    }

    fn chars_from_rules(&self, c: &char) -> OneOrMany<'a> {
        let mut rng = thread_rng();
        if let Some(s) = self.rules.get(&c) {
            match s.choose_weighted(&mut rng, |item| item.1) {
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

    fn next(&mut self) -> Option<Self::Item> {
        let mut ptr = 1_usize;

        loop {
            if ptr > self.depth {
                return None;
            } else {
                if let Some(c) = self.layers[0].next() {
                    return Some(c);
                } else {
                    if let Some(c) = self.layers[ptr].next() {
                        match self.chars_from_rules(&c) {
                            OneOrMany::One(c) => return Some(c),
                            OneOrMany::Many(cs) => self.layers[ptr - 1] = cs,
                        }
                        ptr -= 1
                    } else {
                        ptr += 1
                    }
                }
            }
        }
    }
}
