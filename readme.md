L-System Builder
================

[Lindenmayer systems](https://en.wikipedia.org/wiki/L-system) (or L-Systems) are simple context free formal grammars that that work using a set of *symbols* and a set of *rules* that are applied iteratively to rewrite them. It is often interesting to interpret the symbols of a L-System as a series of *actions* which can then be visualized.

As an introduction consider the original L-System which uses only the symbols "A" and "B" along with two rules which say "A" ⇒ "AB" (A becomes AB) and "B"' ⇒ "A" (B becomes A). Now starting from some initial string of symbols called the *axiom* we can apply these rules. Say we start with just "A" then we get the following sequence of strings.

0. A
1. AB
2. ABA
3. ABAAB
4. ABAABABA
5. ABAABABAABAAB

We can represent this system using lindenmayer as follows.

```rust
use lindenmayer::LSystem;
let axiom = String::from("A");
let rules = [
    ('A', "AB"), ('B', "A")
    ];
let system = LSystem::new(axiom, &rules);
```

The `LSystem` struct can then be used to produce either a string or an iterator that will produce the symbols.

```rust
let depth = 5;
println!("{}",system.string(depth));
// ABAABABAABAAB
println!("{}",system.builder(depth).collect::<String>());
// ABAABABAABAAB
```

More complex L-Systems have more symbols and more rules. Importantly these systems can contain *terminal symbols* which are symbols that do not change when the rules are applied or, equivalently, have a rule that turns them in themselves like "C" ⇒ "C" (C becomes C). Due to how the methods on `LSystem` are implemented it is best to simply not include a rule for these symbols at all, this signals to the program that it can immediately return the symbol. Consider a more complex L-System in which there are many more symbols than rules.

```rust
let axiom = String::from("X");
let rules = [
    ('X', "F[X][+FX]-FX"), 
];
let system = LSystem::new(axiom, &rules);
```

Here the symbols "F", "[", "]", "+", "-" are all implicitly terminals because no rules exists for them. Although they are added to the resulting string by the "X" rule, once they are introduced they never turn into anything else. The string that is produced by this system is fairly long.

```rust
let depth = 3;
println!("{}",system.string(depth));
// F[F[F[X][+FX]-FX][+FF[X][+FX]-FX]-FF[X][+FX]-FX][+FF[F[X][+FX]-FX][+FF[X][+FX]-FX]-FF[X][+FX]-FX]-FF[F[X][+FX]-FX][+FF[X][+FX]-FX]-FF[X][+FX]-FX
```

If the depth argument is set very high the length of the resulting string becomes arbitrarily large and the rate of increase can be quite high. For a depth of 12 the system above will have to allocate three megabytes and the memory usage exceeds a gigabyte at a depth of 16. When using the `.builder` method the only memory usage needed is a pair of pointers per level of recursion. This is can be preferable if the string does not need to be saved.

One usage for the iterator is iterpreting the system as a sequence of instructions. Suitably interpreted this string can produce an image that looks a bit like a tree.

![created with lindenmayer and nannou](https://github.com/SymmetricChaos/lindenmayer/blob/master/tree.png)

It is not required that L-Systems be deterministic. In a stochastic L-System each symbol is rewritten by a rule chosen randomly from a set. These can be created with the `LSystemStochastic` struct with replacement part of the rules specified by `Vec<(&str,f32)>` with each entry containing the possible replacement and its probability relative to the other options. The interface is otherwise similar to the one for `LSystem` except that an `Option<u64>` is needed to seed the `.string()` and `.builder()` methods.

```rust
use lindenmayer::builder::LSystemStochastic;

let axiom = String::from("X");
let rules = [
    ('X', &vec![("F[X][+DX]-DX", 1.0)]),
    ('D', &vec![("F", 2.0), ("FF", 1.0), ("D", 1.0)])
];

let system = LSystemStochastic::new(axiom, &rules);

let depth = 2;
let seed = Some(19251989);

println!("{}", system.string(depth, seed));
//F[F[X][+DX]-DX][+FFF[X][+DX]-DX]-FF[X][+DX]-DX
```