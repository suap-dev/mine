#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use ggez::{
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

    ggez::event::run(context, event_loop, Thing {});
}

struct Thing {}
impl ggez::event::EventHandler for Thing {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);        

        canvas.finish(ctx)
    }
}
