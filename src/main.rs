#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use ggez::{
    glam::{vec2, Vec2},
    graphics::{self, Color},
    Context, GameResult,
};

fn main() -> GameResult {
    let (context, event_loop) = {
        let context_builder = ggez::ContextBuilder::new("Thing", "suap");
        context_builder.build()?
    };

    let thing = Thing::new(&context)?;

    ggez::event::run(context, event_loop, thing);
}

struct Thing {
    circle: graphics::Mesh,
    circle_position: Vec2,
}
impl Thing {
    fn new(context: &Context) -> GameResult<Self> {
        let circle = graphics::Mesh::new_circle(
            context,
            graphics::DrawMode::fill(),
            vec2(0.0, 0.0),
            50.0,
            0.2,
            Color::WHITE,
        )?;

        let circle_position = {
            let (width, height) = context.gfx.drawable_size();
            vec2(width / 2.0, height / 2.0)
        };

        Ok(Self {
            circle,
            circle_position,
        })
    }
}
impl ggez::event::EventHandler for Thing {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        canvas.draw(&self.circle, self.circle_position);

        canvas.finish(ctx)
    }
}
