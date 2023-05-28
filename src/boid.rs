use glam::*;

pub struct Boid {
    pub id: i32,
    pub position: Vec3,
    pub rotation: Mat4,
    pub speed: f32,
}

impl Boid {
    pub fn next_transform(&mut self) {
        let forward = Vec4::new(0.0, 1.0, 0.0, 0.0);
        let direction = self.rotation * forward;
        let direction = Vec3::new(direction.x, direction.y, direction.z);

        // Update the position
        self.position += direction * self.speed;

        // Check boundaries and wrap position if necessary
        if self.position.x < -2.0 {
            self.position.x = 2.0;
        } else if self.position.x > 2.0 {
            self.position.x = -2.0;
        }

        if self.position.y < -1.5 {
            self.position.y = 1.5;
        } else if self.position.y > 1.5 {
            self.position.y = -1.5;
        }
    }

}
