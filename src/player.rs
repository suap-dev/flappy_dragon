use bracket_lib::prelude::*;

const PLAYER_SPRITE_X: i32 = 0;

/// Player (Dragon) in game world.
///
/// `x`, `y` - coordinates in game world
/// `velocity_x`, `velocity_y` - velocity in game world
/// coordinates on the screen: 0, y
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub velocity_y: f32,
    pub velocity_x: f32,
}
impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            velocity_y: 0.0,
            velocity_x: 1.0,
        }
    }
    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(PLAYER_SPRITE_X, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    pub fn gravity_and_move(&mut self) {
        if self.velocity_y < 2.0 {
            self.velocity_y += 0.2;
        }
        #[allow(clippy::cast_possible_truncation)]
        {
            self.x += self.velocity_x as i32;
            self.y += self.velocity_y as i32;
        }
        if self.y < 0 {
            self.y = 0;
        }
    }

    pub fn flap(&mut self) {
        self.velocity_y = -2.0;
    }
}
