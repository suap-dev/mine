use ggez::{
    glam::{Vec2, Vec3},
    graphics::{Color, DrawMode, Mesh},
    Context, GameResult,
};

pub struct Triangle {
    vertices: [Vec3; 3],
    pivot: Vec3,
    projection: Mesh,
}
impl Triangle {
    pub fn new(ctx: &Context, v1: Vec3, v2: Vec3, v3: Vec3) -> GameResult<Self> {
        let points = vec![v1.truncate(), v2.truncate(), v3.truncate()];
        let projection = Mesh::new_polygon(ctx, DrawMode::stroke(5.0), &points, Color::BLUE)?;

        let pivot = (v1 + v2 + v3) / 3.0;

        Ok(Self {
            vertices: [v1, v2, v3],
            pivot,
            projection,
        })
    }
    
    pub const fn get_projection(&self) -> &Mesh {
        &self.projection
    }

    // TODO: this is temporary, let's do a rotator and a proper pitch/yaw/roll, also matrices
    pub fn rotate_x(angle: f32) {}
    pub fn rotate_y(angle: f32) {}
    pub fn rotate_z(angle: f32) {}
}
