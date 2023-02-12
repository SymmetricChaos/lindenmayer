L-System Builder
================

[Lindenmayer systems](https://en.wikipedia.org/wiki/L-system) (or L-Systems) are simple formal grammars that that work using a set of symbols and a set of rules that are applied iteratively to rewrite them. It is often interesting to interpret the symbols of a L-System as a series of actions which can then be visualized.

As an introduction consider the original L-System which uses only the symbols "A" and "B" along with two rules which say "A" ⇒ "AB" (A becomes AB) and "B"' ⇒ "A" (B becomes A). Now starting from some initial string of symbols called the axiom we can apply these rules. Say we start with just "A" then we get the following sequence of strings.

0. A
1. AB
2. ABA
3. ABAAB
4. ABAABABA
5. ABAABABAABAAB

We can represent this system using `lindenmayer` as follows.

```rust
let axiom = "A";
let rules = HashMap::from([
    ('A', "AB"), ('B', "A")
    ]);
let system = LSystemBuilder::new(axiom, rules, depth: 5);
```

The `LSystemBuilder` struct is an iterator that will produce the symbols from line 5 from the sequence.