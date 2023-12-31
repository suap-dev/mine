mod spatial;

use std::f32::consts::TAU;
use ggez::{
    glam::{vec2, vec3, Vec2, Vec3},
    graphics::{self, Color},
    winit::event::VirtualKeyCode,
    Context, GameResult,
};

use crate::spatial::Shape;

#[derive(Default)]
struct Direction {
    // TODO: try to do this with X and Y axis, or UP and RIGHT axis.
    up: f32,
    down: f32,
    left: f32,
    right: f32,
}

#[derive(Default)]
struct Rotation {
    pitch: f32,
    yaw: f32,
    roll: f32,
}

pub struct Thing {
    shape: Shape,
    light_direction: Vec3,
    direction_input: Direction,
    rotation_input: Rotation,
}
impl Thing {
    pub fn new(ctx: &Context) -> Self {
        let screen_size = ctx.gfx.window().inner_size();

        #[allow(clippy::cast_precision_loss)]
        let (height, width) = (screen_size.height as f32, screen_size.width as f32);

        let mut shape = Shape::empty(vec3(width / 2.0, height / 2.0, 0.0));
        let v1 = vec3(1.0, 1.0, 1.0) * height / 4.0;
        let v2 = vec3(-1.0, -1.0, 1.0) * height / 4.0;
        let v3 = vec3(-1.0, 1.0, -1.0) * height / 4.0;
        let v4 = vec3(1.0, -1.0, -1.0) * height / 4.0;
        shape.push_triangle([v1, v2, v4]);
        shape.push_triangle([v1, v3, v2]);
        shape.push_triangle([v2, v3, v4]);
        shape.push_triangle([v1, v4, v3]);

        let light_direction: Vec3 = vec3(1.0, 1.0, 0.0).normalize();

        Self {
            shape,
            light_direction,
            direction_input: Direction::default(),
            rotation_input: Rotation::default(),
        }
    }

    fn input_vector(&self) -> Vec2 {
        let y = self.direction_input.down - self.direction_input.up;
        let x = self.direction_input.right - self.direction_input.left;

        vec2(x, y).clamp_length(0.0, 1.0)
    }
}

impl ggez::event::EventHandler for Thing {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const MOVE_SPEED: f32 = 500.0;
        const ROTATION_SPEED: f32 = TAU / 3.0;

        let dt32 = ctx.time.delta().as_secs_f32();

        let movement_vector = self.input_vector().extend(0.0) * MOVE_SPEED * dt32;
        self.shape.translate(movement_vector);

        self.shape
            .add_pitch(ROTATION_SPEED * dt32 * self.rotation_input.pitch);
        self.shape
            .add_roll(ROTATION_SPEED * dt32 * self.rotation_input.roll);
        self.shape
            .add_yaw(ROTATION_SPEED * dt32 * self.rotation_input.yaw);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        let origin = self.shape.get_origin().truncate();

        for triangle in &mut self.shape.triangles {
            if triangle.is_visible() {
                // if let triangle = &triangle.get_projection(ctx)?;
                if let Some(triangle) = triangle.get_projection(ctx, false, self.light_direction)? {
                    canvas.draw(triangle, origin);
                }
            }
        }

        canvas.finish(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeat: bool,
    ) -> GameResult {
        match input.scancode {
            17 => self.direction_input.up = 1.0,    // W(QWERTY) or Z(AZERTY)
            31 => self.direction_input.down = 1.0,  // S
            30 => self.direction_input.left = 1.0,  // A(QWERTY) or Q(AZERTY)
            32 => self.direction_input.right = 1.0, // D

            _ => (),
        }

        match input.keycode {
            Some(VirtualKeyCode::R) => self.rotation_input.roll = 1.0,
            Some(VirtualKeyCode::F) => self.rotation_input.yaw = 1.0,
            Some(VirtualKeyCode::V) => self.rotation_input.pitch = 1.0,
            _ => (),
        }

        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
    ) -> GameResult {
        match input.scancode {
            17 => self.direction_input.up = 0.0,    // W(QWERTY) or Z(AZERTY)
            31 => self.direction_input.down = 0.0,  // S
            30 => self.direction_input.left = 0.0,  // A(QWERTY) or Q(AZERTY)
            32 => self.direction_input.right = 0.0, // D

            _ => (),
        }

        match input.keycode {
            Some(VirtualKeyCode::R) => self.rotation_input.roll = 0.0,
            Some(VirtualKeyCode::F) => self.rotation_input.yaw = 0.0,
            Some(VirtualKeyCode::V) => self.rotation_input.pitch = 0.0,
            _ => (),
        }

        Ok(())
    }
}
