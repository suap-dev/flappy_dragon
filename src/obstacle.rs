use bracket_lib::prelude::*;

use crate::{consts::SCREEN_HEIGHT, player::Player};

/// Obstacle in game world
/// `x` - position in game world
/// `gap_y` - center of the gap
/// `gap_size` - size of the gap
pub struct Obstacle {
    pub x: i32,

    gap_y: i32,
    gap_size: i32,
    /*
       obstacle looks like this:
                                   |
                                   |
                                   |
                       gap_y _             } gap size
                                           } --------
                                   |
                                   |
                                   |
    */
}
impl Obstacle {
    pub fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Self {
            x,
            gap_y: random.range(10, 40),
            gap_size: i32::max(2, 20 - score),
        }
    }
    pub fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;

        for y in 0..(self.gap_y - self.gap_size / 2) {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        for y in (self.gap_y + self.gap_size / 2)..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }
    pub fn hit_obstacle(&mut self, player: &Player) -> bool {
        let gap_top = self.gap_y - self.gap_size / 2;
        let gap_bottom = self.gap_y + self.gap_size / 2;

        // if player is at the same X and NOT inside the gap:
        player.x == self.x && !(player.y < gap_top && player.y > gap_bottom)
    }
}
