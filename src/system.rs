use std::collections::HashMap;

use glam::Vec2;

use crate::{cursor::Cursor, segment::Segment};

/// Actions when reading the L-System
#[derive(Debug, Copy, Clone)]
pub enum Action {
    /// Do nothing
    None,
    /// Do nothing but report that symbol isn't recognized
    Unknown,
    /// Move the Cursor forward the specified distance
    MoveForward(f32),
    /// Move the Cursor forward and save a Segment representing a line between the positions to self.segments
    DrawForward(f32),
    /// Rotate the Cursor by an angle given in radians
    RotateRad(f32),
    /// Rotate the Cursor by an angle given in degrees
    RotateDeg(f32),
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

/// A Lindenmayer System that can be interpreted as a series of actions in 2D space
pub struct LSystem {
    expression: Box<dyn Iterator<Item = char>>,
    actions: HashMap<char, Action>,
    pub segments: Vec<Segment>,
    pub cursors: Vec<Cursor>,
    pub positions: Vec<Vec2>,
    pub angles: Vec<Vec2>,
    cursor: Cursor,
}

impl LSystem {
    pub fn new(
        expression: Box<dyn Iterator<Item = char>>,
        actions: HashMap<char, Action>,
        cursor: Cursor,
    ) -> Self {
        LSystem {
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
                        let mut new_cursor = self.cursor;
                        new_cursor.forward(*dist);
                        self.segments.push(Segment::from((
                            self.cursor.get_position(),
                            new_cursor.get_position(),
                        )));
                        self.cursor = new_cursor;
                    }
                    Action::MoveForward(dist) => self.cursor.forward(*dist),
                    Action::RotateRad(radians) => self.cursor.rotate(*radians),
                    Action::RotateDeg(degrees) => self.cursor.rotate_degrees(*degrees),
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
                    _ => (),
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
