use glam::Vec2;

/// A simple cursor with a position and direction in 2D space.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cursor {
    position: Vec2,
    angle: Vec2,
}

impl Cursor {
    const DEG_TO_RAD: f32 = std::f32::consts::PI / 180.0;

    /// Construct a Cursor. If the angle cannot be normalized (for instance if both components are zero) this method panics.
    pub fn new(position: impl Into<Vec2>, angle: impl Into<Vec2>) -> Self {
        Cursor {
            position: Into::into(position),
            angle: Into::into(angle)
                .try_normalize()
                .expect("unable to normalize angle during Cursor creation"),
        }
    }

    /// Return a copy of the Cursor's position.
    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    /// Return a copy of the Cursor's angle.
    pub fn get_angle(&self) -> Vec2 {
        self.angle
    }

    /// Set the Cursor's position.
    pub fn set_position(&mut self, position: Vec2) {
        self.position = position
    }

    /// Set the Cursor's angle. If the angle cannot be normalized (for instance if both components are zero) this method panics.
    pub fn set_angle(&mut self, angle: Vec2) {
        self.angle = angle
            .try_normalize()
            .expect("unable to normalize angle when setting Cursor angle")
    }

    /// Rotate the Cursor's angle by the specified number of radians.
    pub fn rotate(&mut self, radians: f32) {
        let rad_cos = radians.cos();
        let rad_sin = radians.sin();
        let x = self.angle.x * rad_cos - self.angle.y * rad_sin;
        let y = self.angle.x * rad_sin + self.angle.y * rad_cos;
        self.angle = glam::vec2(x, y)
            .try_normalize()
            .expect("unable to normalize angle during Cursor rotation")
    }

    /// Rotate the Cursor's angle by the specified number of degrees.
    pub fn rotate_degrees(&mut self, degrees: f32) {
        self.rotate(degrees * Self::DEG_TO_RAD)
    }

    /// Move the Cursor forward the specified distance along it angle.
    pub fn forward(&mut self, distance: f32) {
        self.position += self.angle * distance
    }
}
