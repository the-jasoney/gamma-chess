use super::{bitboard::Board, color::Color};

pub struct Game {
    pub to_play: Color,
    pub board: Board,
}

impl Game {
    pub fn setup_position(&mut self, fen: String) {

    }
}
