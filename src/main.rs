#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

mod thing;

use ggez::GameResult;
use thing::Thing;

fn main() -> GameResult {
    let (context, event_loop) = {
        let window_setup = ggez::conf::WindowSetup::default()
            .title("Thing")
            .vsync(false);

        let window_mode = ggez::conf::WindowMode::default()
            .dimensions(1920.0, 1080.0)
            .maximized(false);

        ggez::ContextBuilder::new("Thing", "suap")
            .window_setup(window_setup)
            .window_mode(window_mode)
            .build()?
    };

    let thing = Thing::new(&context)?;

    ggez::event::run(context, event_loop, thing);
}
