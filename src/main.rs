#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

mod thing;

use ggez::GameResult;
use thing::Thing;

fn main() -> GameResult {
    let (context, event_loop) = {
        let context_builder = ggez::ContextBuilder::new("Thing", "suap");
        context_builder.build()?
    };

    let thing = Thing::new(&context)?;

    ggez::event::run(context, event_loop, thing);
}
