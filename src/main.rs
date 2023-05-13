pub mod chess;
pub mod gui;
pub mod piece;

use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(WHITE);

        next_frame().await
    }
}
