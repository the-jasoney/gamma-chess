use super::{bitboard::Board, color::Color};

pub struct Game {
    pub to_play: Color,
    pub board: Board,
}

impl Game {
    pub fn setup_position(&mut self, fen: String) {
        let fen_sections = fen.split_whitespace();

        let mut rank = 7;
        let mut file = 0;

        for i in fen_sections[0] {
            if !"pnbrqkPNBRQK".contains(i) {
                todo!();
                continue;
            }


        }
    }
}
