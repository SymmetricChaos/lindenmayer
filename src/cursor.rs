use glam::Vec2;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cursor {
    position: Vec2,
    angle: Vec2,
}

impl Cursor {
    const DEG_TO_RAD: f32 = std::f32::consts::PI / 180.0;

    pub fn new(position: impl Into<Vec2>, angle: impl Into<Vec2>) -> Self {
        Cursor {
            position: Into::into(position),
            angle: Into::into(angle)
                .try_normalize()
                .expect("unable to normalize angle during Cursor creation"),
        }
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn get_angle(&self) -> Vec2 {
        self.angle
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position
    }

    pub fn set_angle(&mut self, angle: Vec2) {
        self.angle = angle
            .try_normalize()
            .expect("unable to normalize angle when setting angle")
    }

    pub fn rotate(&mut self, radians: f32) {
        let rad_cos = radians.cos();
        let rad_sin = radians.sin();
        let x = self.angle.x * rad_cos - self.angle.y * rad_sin;
        let y = self.angle.x * rad_sin + self.angle.y * rad_cos;
        self.angle = glam::vec2(x, y)
            .try_normalize()
            .expect("unable to normalize angle during rotation")
    }

    pub fn rotate_degrees(&mut self, degrees: f32) {
        self.rotate(degrees * Self::DEG_TO_RAD)
    }

    pub fn forward(&mut self, distance: f32) {
        self.position += self.angle * distance
    }
}
