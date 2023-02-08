#[test]
fn validity_test() {
    use std::collections::HashMap;

    use crate::{builder::LSystemBuilder, writer::write_expression};
    let e = LSystemBuilder::new("X", HashMap::from([('X', "F[X][+DX]-DX"), ('D', "F")]), 3);

    let s = write_expression(
        String::from("X"),
        HashMap::from([('X', "F[X][+DX]-DX"), ('D', "F")]),
        3,
    );

    assert!(e.zip(s.chars()).all(|(a, b)| a == b))
}

#[test]
fn speed_test() {
    use std::time::Instant;

    use crate::{
        builder::LSystemBuilder,
        cursor::Cursor,
        system::{Action, LSystem},
        writer::write_expression,
    };
    use std::collections::HashMap;

    fn time_system(system: &mut LSystem, name: &str) {
        let t0 = Instant::now();
        loop {
            if system.step().is_none() {
                break;
            }
        }
        println!("{name}: {:?}", Instant::now() - t0);
    }

    let e = LSystemBuilder::new("X", HashMap::from([('X', "F[X][+DX]-DX"), ('D', "F")]), 10);

    let s: Vec<char> = write_expression(
        String::from("X"),
        HashMap::from([('X', "F[X][+DX]-DX"), ('D', "F")]),
        10,
    )
    .chars()
    .collect();

    let actions = HashMap::from([
        ('F', Action::DrawForward(15.0)),
        ('X', Action::None),
        ('D', Action::Dot),
        ('+', Action::RotateRad(-1.04)),
        ('-', Action::RotateRad(1.04)),
        ('[', Action::PushCursor),
        (']', Action::PopCursor),
    ]);

    let cursor = Cursor::new((0.0, 0.0), (0.0, 1.0));
    let mut system_from_builder = LSystem::new(Box::new(e), actions.clone(), cursor);
    let mut system_from_vec = LSystem::new(Box::new(s.into_iter()), actions.clone(), cursor);

    time_system(&mut system_from_builder, "builder");
    time_system(&mut system_from_vec, "vector");
}
