// #[test]
// fn speed_test() {
//     use crate::{
//         builder::LSystemBuilder,
//         cursor::Cursor,
//         system::{Action, LSystemReader},
//         writer::write_lsystem,
//     };
//     //use get_size::GetSize;
//     use std::collections::HashMap;
//     use std::time::Instant;

//     fn time_system(system: &mut LSystemReader, name: &str) {
//         let t0 = Instant::now();
//         loop {
//             if system.step().is_none() {
//                 break;
//             }
//         }
//         println!("{name}: {:?}", Instant::now() - t0);
//     }

//     let axiom = "X";
//     let rules = HashMap::from([('X', "F[X][+DX]-DX"), ('D', "F")]);
//     let depth = 12;

//     let s: Vec<char> = write_lsystem("X", &rules, depth).chars().collect();
//     //let s_memory_usage = s.get_heap_size();

//     let e = LSystemBuilder::new(axiom, rules, depth);
//     //let e_memory_usage = e.get_heap_size();

//     let actions = HashMap::from([
//         ('F', Action::DrawForward(15.0)),
//         ('X', Action::None),
//         ('D', Action::PushPosition),
//         ('+', Action::RotateRad(-1.04)),
//         ('-', Action::RotateRad(1.04)),
//         ('[', Action::PushCursor),
//         (']', Action::PopCursor),
//     ]);

//     let cursor = Cursor::new((0.0, 0.0), (0.0, 1.0));
//     let mut system_from_builder = LSystemReader::new(Box::new(e), actions.clone(), cursor);
//     let mut system_from_vec = LSystemReader::new(Box::new(s.into_iter()), actions.clone(), cursor);

//     time_system(&mut system_from_builder, "builder");
//     time_system(&mut system_from_vec, "vector");
//     println!("bytes on heap: {s_memory_usage}");
// }
