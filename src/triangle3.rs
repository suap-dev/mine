use std::f32::consts::TAU;

use ggez::{
    glam::{vec2, Vec2, Vec3, Vec3Swizzles},
    graphics::{Color, DrawMode, Mesh},
    Context, GameResult,
};

pub struct Triangle {
    vertices: [Vec3; 3],
    vertices_changed: bool,
    origin: Vec3,
    projection: Mesh,
    visible: bool,
    update_projection_if_not_visible: bool,
}
impl Triangle {
    pub fn from_screen_coords(
        ctx: &Context,
        origin: Vec3,
        v1: Vec3,
        v2: Vec3,
        v3: Vec3,
    ) -> GameResult<Self> {
        // let origin = (v1 + v2 + v3) / 3.0;
        let v1 = v1 - origin;
        let v2 = v2 - origin;
        let v3 = v3 - origin;
        let vertices = [v1, v2, v3];

        let projection = Self::generate_projection(ctx, &vertices)?;

        let visible = Self::determinant(&vertices) > 0.0;

        Ok(Self {
            vertices,
            vertices_changed: false,
            origin,
            projection,
            visible,
            update_projection_if_not_visible: false,
        })
    }

    pub fn equilateral(ctx: &Context, origin: Vec3, height: f32) -> GameResult<Self> {
        let rotator_ish = Vec2::from_angle(TAU / 3.0);

        let v1 = vec2(0.0, 1.0) * height * 2.0 / 3.0;
        let v2 = rotator_ish.rotate(v1);
        let v3 = rotator_ish.rotate(v2);

        let vertices = [v1.extend(0.0), v2.extend(0.0), v3.extend(0.0)];

        let projection = Self::generate_projection(ctx, &vertices)?;

        let visible = Self::determinant(&vertices) > 0.0;

        Ok(Self {
            vertices,
            vertices_changed: false,
            origin,
            projection,
            visible,
            update_projection_if_not_visible: false,
        })
    }

    pub fn get_projection(&mut self, ctx: &Context) -> GameResult<&Mesh> {
        if (self.update_projection_if_not_visible || self.visible) && self.vertices_changed {
            self.projection = Self::generate_projection(ctx, &self.vertices)?;
            self.vertices_changed = false;
        }
        Ok(&self.projection)
    }

    pub const fn get_origin(&self) -> Vec3 {
        self.origin
    }

    pub const fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn translate(&mut self, vector: Vec3) {
        self.origin += vector;
    }

    // TODO: this is temporary, let's do a rotator and a proper pitch/yaw/roll, also matrices
    // TODO: DRY
    pub fn rotate_x(&mut self, angle: f32) {
        let rotator_ish = Vec2::from_angle(angle);
        self.vertices.iter_mut().for_each(|v| {
            let v_rotated = rotator_ish.rotate(v.yz());
            v.y = v_rotated.x;
            v.z = v_rotated.y;
        });
        self.visible = Self::determinant(&self.vertices) > 0.0;
        self.vertices_changed = true;
    }

    pub fn rotate_y(&mut self, angle: f32) {
        let rotator_ish = Vec2::from_angle(angle);
        self.vertices.iter_mut().for_each(|v| {
            let v_rotated = rotator_ish.rotate(v.xz());
            v.x = v_rotated.x;
            v.z = v_rotated.y;
        });
        self.visible = Self::determinant(&self.vertices) > 0.0;
        self.vertices_changed = true;
    }

    pub fn rotate_z(&mut self, angle: f32) {
        let rotator_ish = Vec2::from_angle(angle);
        self.vertices.iter_mut().for_each(|v| {
            let v_rotated = rotator_ish.rotate(v.xy());
            v.x = v_rotated.x;
            v.y = v_rotated.y;
        });
        self.visible = Self::determinant(&self.vertices) > 0.0;
        self.vertices_changed = true;
    }

    fn truncate(vertices: &[Vec3; 3]) -> [Vec2; 3] {
        vertices.map(Vec3::truncate)
    }

    fn determinant(vertices: &[Vec3; 3]) -> f32 {
        (vertices[1].x - vertices[0].x).mul_add(
            vertices[2].y - vertices[0].y,
            -(vertices[2].x - vertices[0].x) * (vertices[1].y - vertices[0].y),
        )
    }

    fn generate_projection(ctx: &Context, vertices: &[Vec3; 3]) -> GameResult<Mesh> {
        Mesh::new_polygon(
            ctx,
            DrawMode::stroke(3.0),
            &Self::truncate(vertices),
            Color::BLUE,
        )
    }
}
