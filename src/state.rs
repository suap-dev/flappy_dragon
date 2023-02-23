use crate::consts::{SCREEN_WIDTH, FRAME_DURATION, SCREEN_HEIGHT};
use crate::obstacle::Obstacle;
use crate::player::Player;
use bracket_lib::prelude::*;

#[derive(Debug)]
enum GameMode {
    Menu,
    Playing,
    End,
}

pub struct State {
    mode: GameMode,
    frame_time: f32,

    player: Player,
    obstacle: Obstacle,
    score: i32,
}
impl State {
    pub fn new() -> Self {
        Self {
            mode: GameMode::Menu,
            frame_time: 0.0,

            player: Player::new(5, 25),
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        println!("{:?}", self.mode);

        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon!");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit GAme");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn dead(&mut self, ctx: &mut BTerm) {
        println!("{:?}", self.mode);

        ctx.cls();
        ctx.print_centered(5, "You died.");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit GAme");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn play(&mut self, ctx: &mut BTerm) {
        println!("{:?}", self.mode);

        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
        }

        self.player.gravity_and_move();
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }
        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }

        if ctx.key == Some(VirtualKeyCode::Space) {
            self.player.flap();
        }

        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, &format!("Score: {}", self.score));
        self.player.render(ctx);
        self.obstacle.render(ctx, self.player.x);
    }
    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
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
