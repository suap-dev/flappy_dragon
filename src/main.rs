#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub mod state;
pub mod player;
pub mod obstacle;
pub mod consts;

use bracket_lib::prelude::*;
use state::State;


fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}