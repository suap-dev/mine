use ggez::{
    glam::{Vec2, Vec3, Vec3Swizzles},
    graphics::{Color, DrawMode, Mesh},
    Context, GameResult,
};

pub struct Triangle {
    vertices: [Vec3; 3],
    vertices_changed: bool,
    origin: Vec3,
    projection: Mesh,
}
impl Triangle {
    pub fn new(ctx: &Context, v1: Vec3, v2: Vec3, v3: Vec3) -> GameResult<Self> {
        let origin = (v1 + v2 + v3) / 3.0;
        let v1 = v1 - origin;
        let v2 = v2 - origin;
        let v3 = v3 - origin;
        let vertices = [v1, v2, v3];

        let projection = Self::generate_projection(ctx, &vertices)?;

        Ok(Self {
            vertices,
            vertices_changed: false,
            origin,
            projection,
        })
    }

    pub fn get_projection(&mut self, ctx: &Context) -> GameResult<&Mesh> {
        if self.vertices_changed {
            self.projection = Self::generate_projection(ctx, &self.vertices)?;
            self.vertices_changed = false;
        }
        Ok(&self.projection)
    }

    fn truncate(vertices: &[Vec3; 3]) -> [Vec2; 3] {
        vertices.map(Vec3::truncate)
    }

    fn generate_projection(ctx: &Context, vertices: &[Vec3; 3]) -> GameResult<Mesh> {
        Mesh::new_polygon(
            ctx,
            DrawMode::stroke(3.0),
            &Self::truncate(vertices),
            Color::BLUE,
        )
    }

    pub const fn get_origin(&self) -> Vec3 {
        self.origin
    }

    pub fn translate(&mut self, vector: Vec3) {
        self.origin += vector;
    }

    // TODO: this is temporary, let's do a rotator and a proper pitch/yaw/roll, also matrices
    pub fn rotate_x(angle: f32) {}
    pub fn rotate_y(&mut self, angle: f32) {
        let mut v1 = self.vertices[0];
        let mut v2 = self.vertices[1];
        let mut v3 = self.vertices[2];

        let rotator_ish = Vec2::from_angle(angle);

        let v1_rotated = rotator_ish.rotate(v1.xz());
        v1.x = v1_rotated.x;
        v1.z = v1_rotated.y;

        
        let v2_rotated = rotator_ish.rotate(v2.xz());
        v2.x = v2_rotated.x;
        v2.z = v2_rotated.y;

        let v3_rotated = rotator_ish.rotate(v3.xz());
        v3.x = v3_rotated.x;
        v3.z = v3_rotated.y;

        self.vertices = [v1, v2, v3];
        self.vertices_changed = true;
    }

    pub fn rotate_z(&mut self, angle: f32) {
        let mut v1 = self.vertices[0];
        let mut v2 = self.vertices[1];
        let mut v3 = self.vertices[2];

        let rotator_ish = Vec2::from_angle(angle);
        v1 = rotator_ish.rotate(v1.xy()).extend(v1.z);
        v2 = rotator_ish.rotate(v2.xy()).extend(v2.z);
        v3 = rotator_ish.rotate(v3.xy()).extend(v3.z);

        self.vertices = [v1, v2, v3];
        self.vertices_changed = true;
    }
}
