use glam::Vec2;

#[derive(Debug, Copy, Clone)]
pub struct Segment {
    pub start: Vec2,
    pub end: Vec2,
}

impl Segment {
    pub fn new(start: impl Into<Vec2>, end: impl Into<Vec2>) -> Self {
        Segment {
            start: Into::into(start),
            end: Into::into(end),
        }
    }
}

impl From<(Vec2, Vec2)> for Segment {
    fn from(value: (Vec2, Vec2)) -> Self {
        Self {
            start: value.0,
            end: value.1,
        }
    }
}
