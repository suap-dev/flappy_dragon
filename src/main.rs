#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use bracket_lib::prelude::*;

#[derive(Debug)]
enum GameMode {
    Menu,
    Playing,
    End,
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}

struct State {
    mode: GameMode,
}
impl State {
    const fn new() -> Self {
        Self {
            mode: GameMode::Menu,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        println!("{:?}", self.mode);
        // TODO

        self.mode = GameMode::Playing;
    }
    fn dead(&self, ctx: &mut BTerm) {
        println!("{:?}", self.mode);
        // TODO
        ctx.quitting = true;
    }
    fn play(&mut self, ctx: &mut BTerm) {
        println!("{:?}", self.mode);
        // TODO

        self.mode = GameMode::End;
    }
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}
