use ggez::glam::Vec3;

use crate::triangle3::Triangle;

struct Shape {
    triangles: Vec<Triangle>,
}
impl Shape {
    pub const fn empty() -> Self {
        Self {
            triangles: Vec::new(),
        }
    }

    pub fn add_triangle(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
    }

    pub fn add_pitch(&mut self, angle: f32) {
        for triangle in &mut self.triangles {
            triangle.add_pitch(angle);
        }
    }

    pub fn add_roll(&mut self, angle: f32) {
        for triangle in &mut self.triangles {
            triangle.add_roll(angle);
        }
    }

    pub fn add_yaw(&mut self, angle: f32) {
        for triangle in &mut self.triangles {
            triangle.add_yaw(angle);
        }
    }

    pub fn translate(&mut self, vector: Vec3) {
        for triangle in &mut self.triangles {
            triangle.translate(vector);
        }
    }
}
