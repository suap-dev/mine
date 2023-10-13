use std::f32::consts::TAU;

use ggez::{
    glam::{vec2, vec3, Vec2},
    graphics::{self, Color},
    winit::event::VirtualKeyCode,
    Context, GameResult,
};

use crate::triangle3::Triangle;

const MOVE_SPEED: f32 = 100.0;
const ROTATION_SPEED: f32 = TAU / 12.0;
#[derive(Default)]
struct DirectionInput {
    // TODO: try to do this with X and Y axis, or UP and RIGHT axis.
    up: f32,
    down: f32,
    left: f32,
    right: f32,
}
struct Rotation {
    pitch: f32,
    yaw: f32,
    roll: f32,
}

pub struct Thing {
    triangle: Triangle,
    direction_input: DirectionInput,
}
impl Thing {
    pub fn new(ctx: &Context) -> GameResult<Self> {
        let screen_size = ctx.gfx.window().inner_size();

        let height = screen_size.height as f32;
        let width = screen_size.width as f32;

        let triangle =
            Triangle::equilateral(ctx, vec3(width / 2.0, height / 2.0, 0.0), height / 3.0)?;

        Ok(Self {
            triangle,
            // mesh_position: circle_position,
            direction_input: DirectionInput::default(),
        })
    }

    fn input_vector(&self) -> Vec2 {
        let y = self.direction_input.down - self.direction_input.up;
        let x = self.direction_input.right - self.direction_input.left;

        vec2(x, y).clamp_length(0.0, 1.0)
    }
}
impl ggez::event::EventHandler for Thing {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt32 = ctx.time.delta().as_secs_f32();
        let movement_vector = self.input_vector().extend(0.0) * MOVE_SPEED * dt32;
        self.triangle.translate(movement_vector);
        self.triangle.rotate_x(ROTATION_SPEED * dt32);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        let origin = self.triangle.get_origin().truncate();

        if self.triangle.is_visible() {
            canvas.draw(self.triangle.get_projection(ctx)?, origin);
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
            Some(VirtualKeyCode::Z) => println!("z"),
            Some(VirtualKeyCode::X) => println!("x"),
            Some(VirtualKeyCode::C) => println!("c"),
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
        Ok(())
    }
}
