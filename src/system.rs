use std::collections::HashMap;

use glam::Vec2;

use crate::{cursor::Cursor, segment::Segment};

/// Actions when reading the L-System
#[derive(Debug, Copy, Clone)]
pub enum Action {
    /// Do nothing
    None,
    /// Move the Cursor forward
    MoveForward(f32),
    /// Move the Cursor forward and save a Segment representing a line between the positions to self.segments
    DrawForward(f32),
    /// Rotate the Cursor by an angle given in radians
    RotateRad(f32),
    /// Rotate the Cursor by an angle given in degrees
    RotateDeg(f32),
    /// Push a copy of the Cursor to the cursor stack
    PushCursor,
    /// Pop the top item of the cursor stack and replace the Cursor with it
    PopCursor,
    /// Save the position of the Cursor to self.dots
    Dot,
}

///
pub struct LSystem {
    expression: Box<dyn Iterator<Item = char>>,
    actions: HashMap<char, Action>,
    cursor_stack: Vec<Cursor>,
    pub segments: Vec<Segment>,
    pub dots: Vec<Vec2>,
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
            cursor_stack: Vec::new(),
            segments: Vec::new(),
            dots: Vec::new(),
            cursor,
        }
    }

    /// Read the next character of the expression, perform the corresponding action, and then report the action
    /// Returns None if the expression has been read completely
    pub fn step(&mut self) -> Option<Action> {
        if let Some(c) = self.expression.next() {
            if let Some(a) = self.actions.get(&c) {
                match a {
                    Action::None => (),
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
                    Action::PushCursor => self.cursor_stack.push(self.cursor),
                    Action::PopCursor => {
                        self.cursor = self.cursor_stack.pop().expect("pop from empty stack")
                    }
                    Action::Dot => self.dots.push(self.cursor.get_position()),
                }
                Some(*a)
            } else {
                panic!("unknown character encountered in expression: {c}")
            }
        } else {
            None
        }
    }
}
