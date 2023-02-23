use bracket_lib::prelude::*;

use crate::{consts::SCREEN_HEIGHT, player::Player};

/// Obstacle in game world
/// `x` - position in game world
/// `gap_y` - center of the gap
/// `gap_size` - size of the gap
pub struct Obstacle {
    pub x: i32,

    // gap_y: i32,
    // gap_size: i32,
    gap_top: i32,
    gap_bottom: i32,
    obstacle: Vec<i32>,
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
        let gap_y = random.range(10, 40);
        let gap_size = i32::max(2, 20 - score);

        let gap_top = gap_y - gap_size / 2;
        let gap_bottom = gap_y + gap_size / 2;

        let obstacle = (0..gap_top)
            .into_iter()
            .chain((gap_bottom..SCREEN_HEIGHT).into_iter())
            .collect();

        Self {
            x,
            gap_top,
            gap_bottom,
            obstacle,
        }
    }
    pub fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;

        for &y in &self.obstacle {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }
    pub fn was_hit(&mut self, player: &Player) -> bool {
        // if player is at the same X and NOT inside the gap:
        player.x == self.x && !(player.y < self.gap_top && player.y > self.gap_bottom)
    }
}
