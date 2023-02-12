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

We can represent this system using lindenmayer as follows.

```rust
use lindenmayer::LSystemBuilder;
let axiom = "A";
let rules = HashMap::from([
    ('A', "AB"), ('B', "A")
    ]);
let system = LSystemBuilder::new(axiom, rules, depth: 5);
```

The `LSystemBuilder` struct is an iterator that will produce the symbols from line 5 from the sequence.

More complex L-Systems have more symbols and more rules. Importantly these systems can contain *terminal symbols* which are symbols that do not change when the rules are applied or, equivalently, have a rule that turns them in themselves like "C" ⇒ "C" (C becomes C). Due to how `LSystemBuilder` is implemented it is best to simply not include a rule for these symbols at all. Consider a more complex L-System in which there are many more symbols than rules.

```rust
let axiom = "X";
let rules = HashMap::from([
    ('X', "F[X][+FX]-FX"), 
]);
let system = LSystemBuilder::new(axiom, rules, depth: 3);
```

Here the symbols "F", "[", "]", "+", "-" are all implicitly terminals because no rules exists for them. Although they are added to the resulting string by the "X" rule, once they are introduced they never turn into anything else. The string that is produced by this system is quite long.

- F[F[F[X][+FX]-FX][+FF[X][+FX]-FX]-FF[X][+FX]-FX][+FF[F[X][+FX]-FX][+FF[X][+FX]-FX]-FF[X][+FX]-FX]-FF[F[X][+FX]-FX][+FF[X][+FX]-FX]-FF[X][+FX]-FX

Suitably interpreted this string can produce an image that looks a bit like a tree.

![created with lindenmayer and nannou](https://github.com/SymmetricChaos/lindenmayer/blob/master/tree.png)

To faciliate this lindenmayer includes the `LSystemReader` struct which takes in the builder, actions to interpret the symbols, and [a cursor](https://en.wikipedia.org/wiki/Turtle_graphics). The `LSystemReader` will then follow the instructions to move the cursor through 2D space according to the actions described. The image above was generated using the reader below and then drawn using nannou.

```rust
use lindenmayer::{Action, LSystemReader, Cursor};

let actions = HashMap::from([
    ('X', Action::None),
    ('F', Action::DrawForward(60.0)),
    ('+', Action::RotateDeg(-25.0)),
    ('-', Action::RotateDeg(25.0)),
    ('[', Action::PushCursor),
    (']', Action::PopCursor),
]);
let cursor = Cursor::new((0.0, -200.0), (0.0, 1.0));
let reader = LSystemReader::new(system, actions, cursor)
```

