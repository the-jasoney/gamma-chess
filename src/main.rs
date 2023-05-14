use std::{env, path};

use ggez::{event, ContextBuilder};
use gui::gui::Game;

pub mod gui;
pub mod piece;

fn main() {
    let mut cb = ContextBuilder::new("", "Cool Game Author");

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        cb = cb.add_resource_path(path);
    }

    // Make a Context.
    let (mut ctx, event_loop) = cb.build().expect("Could not create ggez context");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = Game::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}
