#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use ggez::{
    glam::vec2,
    graphics::{self, Color},
    Context, GameResult,
};

fn main() {
    let (context, event_loop) = {
        let context_builder = ggez::ContextBuilder::new("Thing", "suap");
        context_builder
            .build()
            .expect("Failed to create ggex context.")
    };

    let thing = Thing {
        circle: graphics::Mesh::new_circle(
            &context,
            graphics::DrawMode::fill(),
            vec2(0.0, 0.0),
            50.0,
            0.5,
            Color::WHITE,
        ).unwrap(),
    };

    ggez::event::run(context, event_loop, thing);
}

struct Thing {
    circle: graphics::Mesh,

}
impl ggez::event::EventHandler for Thing {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        canvas.draw(&self.circle, vec2(100.0, 100.0));

        canvas.finish(ctx)
    }
}
