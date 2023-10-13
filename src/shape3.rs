use ggez::{glam::Vec3, Context, GameResult};

use crate::triangle3::Triangle;

pub struct Shape {
    origin: Vec3,
    pub triangles: Vec<Triangle>,
}
impl Shape {
    pub const fn empty(origin: Vec3) -> Self {
        Self {
            origin,
            triangles: Vec::new(),
        }
    }

    pub const fn get_origin(&self) -> Vec3 {
        self.origin
    }

    pub fn push_triangle(&mut self, ctx: &Context, vertices: [Vec3; 3]) -> GameResult {
        self.triangles
            .push(Triangle::new(ctx, self.origin, vertices)?);
        Ok(())
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
        self.origin += vector;
    }
}
