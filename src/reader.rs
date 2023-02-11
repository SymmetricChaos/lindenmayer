use std::collections::HashMap;

use glam::Vec2;

use crate::{builder::LSystemBuilder, cursor::Cursor, segment::Segment};

/// Actions when reading the L-System
#[derive(Debug, Copy, Clone)]
pub enum Action {
    /// Do nothing
    None,
    /// Do nothing but report that symbol isn't recognized
    Unknown,
    /// Custom action
    Custom(&'static str),
    /// Move the Cursor forward the specified distance
    MoveForward(f32),
    /// Move the Cursor forward and save a Segment representing a line between the positions to self.segments
    DrawForward(f32),
    /// Move the Cursor of the specificed location
    MoveTo(Vec2),
    /// Move the Cursor of the specificed location and save a Segment representing a line between the positions to self.segments
    DrawTo(Vec2),
    /// Rotate the Cursor by an angle given in radians
    RotateRad(f32),
    /// Rotate the Cursor by an angle given in degrees
    RotateDeg(f32),
    /// Set the Cursor angle to the given value, which is normalized automatically
    SetAngle(Vec2),
    /// Push a copy of the Cursor to self.cursors
    PushCursor,
    /// Pop the top item of self.cursors and replace the Cursor with it
    PopCursor,
    /// Save the position of the Cursor to self.positions
    PushPosition,
    /// Pop the top item of self.cursors and replace the Cursor's position with it
    PopPosition,
    /// Save the angle of the Cursor to self.angles
    PushAngle,
    /// Pop the top item of self.angles and replace the Cursor's angle with it
    PopAngle,
}

/// Interpret a sequence of symbols as actions in 2D space.
pub struct LSystemReader<'a> {
    expression: LSystemBuilder<'a>,
    actions: HashMap<char, Action>,
    pub segments: Vec<Segment>,
    pub cursors: Vec<Cursor>,
    pub positions: Vec<Vec2>,
    pub angles: Vec<Vec2>,
    pub cursor: Cursor,
}

impl<'a> LSystemReader<'a> {
    pub fn new(
        expression: LSystemBuilder<'a>,
        actions: HashMap<char, Action>,
        cursor: Cursor,
    ) -> Self {
        LSystemReader {
            expression,
            actions,
            segments: Vec::new(),
            cursors: Vec::new(),
            positions: Vec::new(),
            angles: Vec::new(),
            cursor,
        }
    }

    /// Read the next character of the expression, perform the corresponding action, and then report the action
    /// Returns None if the expression has been read completely
    pub fn step(&mut self) -> Option<Action> {
        if let Some(c) = self.expression.next() {
            if let Some(a) = self.actions.get(&c) {
                match a {
                    Action::DrawForward(dist) => {
                        let old_pos = self.cursor.get_position();
                        self.cursor.forward(*dist);
                        self.segments
                            .push(Segment::from((old_pos, self.cursor.get_position())));
                    }
                    Action::MoveForward(dist) => self.cursor.forward(*dist),
                    Action::DrawTo(pos) => {
                        let old_pos = self.cursor.get_position();
                        self.cursor.set_position(*pos);
                        self.segments.push(Segment::from((old_pos, *pos)));
                    }
                    Action::MoveTo(pos) => self.cursor.set_position(*pos),
                    Action::RotateRad(radians) => self.cursor.rotate(*radians),
                    Action::RotateDeg(degrees) => self.cursor.rotate_degrees(*degrees),
                    Action::SetAngle(angle) => self.cursor.set_angle(*angle),
                    Action::PushCursor => self.cursors.push(self.cursor),
                    Action::PopCursor => {
                        self.cursor = self
                            .cursors
                            .pop()
                            .expect("tried to pop from self.cursors when it was empty")
                    }
                    Action::PushPosition => self.positions.push(self.cursor.get_position()),
                    Action::PopPosition => self.cursor.set_position(
                        self.positions
                            .pop()
                            .expect("tried to pop from self.positions when it was empty"),
                    ),
                    Action::PushAngle => self.angles.push(self.cursor.get_angle()),
                    Action::PopAngle => self.cursor.set_angle(
                        self.angles
                            .pop()
                            .expect("tried to pop from self.angles when it was empty"),
                    ),
                    Action::None | Action::Unknown | Action::Custom(_) => (),
                }
                Some(*a)
            } else {
                Some(Action::Unknown)
            }
        } else {
            None
        }
    }
}

#[test]
fn from_builder() {
    use std::collections::HashMap;

    use crate::builder::LSystemBuilder;

    let axiom = "X";
    let rules = HashMap::from([('X', "F[X][+DX]-DX"), ('D', "F")]);
    let depth = 3;

    let e = LSystemBuilder::new(axiom, rules, depth);

    let actions = HashMap::from([
        ('X', Action::None),
        ('D', Action::PushPosition),
        ('F', Action::DrawForward(40.0)),
        ('+', Action::RotateDeg(-25.0)),
        ('-', Action::RotateDeg(25.0)),
        ('[', Action::PushCursor),
        (']', Action::PopCursor),
    ]);
    let cursor = Cursor::new((0.0, -200.0), (0.0, 1.0));

    let _ = LSystemReader::new(e, actions, cursor);
}
