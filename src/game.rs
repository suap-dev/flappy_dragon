use crate::consts::{FRAME_DURATION, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::obstacle::Obstacle;
use crate::player::Player;
use bracket_lib::prelude::*;

#[derive(Debug)]
enum GameMode {
    Menu,
    Playing,
    Dead,
}

pub struct FlappyDragon {
    mode: GameMode,
    frame_time: f32,

    player: Player,
    obstacle: Obstacle,
    score: i32,
}
impl GameState for FlappyDragon {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.menu(ctx),
            GameMode::Playing => self.playing(ctx),
            GameMode::Dead => self.dead(ctx),
        }
    }
}
impl FlappyDragon {
    pub(crate) fn new() -> Self {
        let score = 0;
        Self {
            mode: GameMode::Menu,
            frame_time: 0.0,
            player: Player::new(5, 25),
            obstacle: Obstacle::new(SCREEN_WIDTH, score),
            score,
        }
    }

    fn new_game(&mut self) {
        self.player = Player::new(5, 25);
        self.score = 0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, self.score);

        self.frame_time = 0.0;
        
        self.mode = GameMode::Playing;        
    }
    fn quit_game(ctx: &mut BTerm) {
        ctx.quitting = true;
    }

    fn menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon!");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit GAme");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.new_game(),
                VirtualKeyCode::Q => Self::quit_game(ctx),
                _other => {}
            }
        }
    }
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You died.");
        ctx.print_centered(6, format!("Your score: {}", self.score));
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit GAme");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.new_game(),
                VirtualKeyCode::Q => Self::quit_game(ctx),
                _other => {}
            }
        }
    }
    fn playing(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
        }

        self.player.gravity_and_move();
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
            // transformed x coordinate
        }
        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::Dead;
        }

        if ctx.key == Some(VirtualKeyCode::Space) {
            self.player.flap();
        }

        self.player.render(ctx);
        self.obstacle.render(ctx, self.player.x);
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, format!("Score: {}", self.score));
    }
}
