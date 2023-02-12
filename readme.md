L-System Builder
================

[Lindenmayer systems](https://en.wikipedia.org/wiki/L-system) (or L-Systems) are simple formal grammars that that work using a set of *symbols* and a set of *rules* that are applied iteratively to rewrite them. It is often interesting to interpret the symbols of a L-System as a series of *actions* which can then be visualized.

As an introduction consider the original L-System which uses only the symbols "A" and "B" along with two rules which say "A" ⇒ "AB" (A becomes AB) and "B"' ⇒ "A" (B becomes A). Now starting from some initial string of symbols called the axiom we can apply these rules. Say we start with just "A" then we get the following sequence of strings.

0. A
1. AB
2. ABA
3. ABAAB
4. ABAABABA
5. ABAABABAABAAB

We can represent this system using `lindenmayer` as follows.

```rust
use lindenmayer::LSystemBuilder;
let axiom = "A";
let rules = HashMap::from([
    ('A', "AB"), ('B', "A")
    ]);
let system = LSystemBuilder::new(axiom, rules, depth: 5);
```

The `LSystemBuilder` struct is an iterator that will produce the symbols from line 5 from the sequence.

More complex L-Systems have more symbols and more rules. Importantly these systems can contain *terminal symbols* which are symbols that do not change when the rules are applied. Due to how `LSystemBuilder` is implemented it is best to simply not include a rule for these symbols and instead leave their status as terminal symbols implied. Consider a more complex L-System in which there are many more symbols than rules.

```rust
let axiom = "X";
let rules = HashMap::from([
    ('X', "F[X][+FX]-FX"), 
]);
let system = LSystemBuilder::new(axiom, rules, depth: 3);
```

Here the symbols "F", "[", "]", "+", "-" are all terminals, they do not change when the rules are applied although new ones can be added by other rules. The string that is produced by this system is quite long.

- F[F[F[X][+FX]-FX][+FF[X][+FX]-FX]-FF[X][+FX]-FX][+FF[F[X][+FX]-FX][+FF[X][+FX]-FX]-FF[X][+FX]-FX]-FF[F[X][+FX]-FX][+FF[X][+FX]-FX]-FF[X][+FX]-FX

Suitably interpreted this string can produce an image that looks a bit like a tree.

https://github.com/SymmetricChaos/lindenmayer/blob/master/tree.png