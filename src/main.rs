#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub mod game;
pub mod player;
pub mod obstacle;
pub mod consts;

use bracket_lib::prelude::*;
use game::FlappyDragonGame;


fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, FlappyDragonGame::new())
}