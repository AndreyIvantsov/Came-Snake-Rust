use piston_window::graphics::{rectangle, Context, };
use piston_window::graphics::types::Color;
use piston_window::G2d;

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}