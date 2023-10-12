use ggez::{
    glam::{vec2, Vec2},
    graphics::{self, Color},
    Context, GameResult,
};

pub struct Thing {
    circle: graphics::Mesh,
    circle_position: Vec2,
    direction_input: DirectionInput,
}
const MOVE_SPEED: f32 = 100.0;
#[derive(Default)]
struct DirectionInput {
    // TODO: try to do this with X and Y axis, or UP and RIGHT axis.
    up: f32,
    down: f32,
    left: f32,
    right: f32,
}
impl Thing {
    pub fn new(ctx: &Context) -> GameResult<Self> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2(0.0, 0.0),
            50.0,
            0.2,
            Color::WHITE,
        )?;

        let circle_position = {
            let (width, height) = ctx.gfx.drawable_size();
            vec2(width / 2.0, height / 2.0)
        };

        Ok(Self {
            circle,
            circle_position,
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
        // TODO: diagonaly it's gonna go too fast - fix it
        self.circle_position += self.input_vector() * MOVE_SPEED * ctx.time.delta().as_secs_f32();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        canvas.draw(&self.circle, self.circle_position);

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
