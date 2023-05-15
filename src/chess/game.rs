use super::bit_manip::set_bit;
use super::{bitboard::Board, color::Color};

pub struct Game {
    pub to_play: Color,
    pub board: Board,
}

impl Game {
    pub fn setup_position(&mut self, fen: String) {
        let fen_sections: Vec<&str> = fen.split_whitespace().collect();

        let mut rank: u8 = 7;
        let mut file: u8 = 0;

        for i in fen_sections[0].chars() {
            let bit = rank * 8 + file;
            if !"pnbrqkPNBRQK".contains(i) {
                if "12345678".contains(i) {
                    file += i.to_string().parse::<u8>().unwrap()
                } else if i == '/' {
                    rank -= 1;
                    file = 0;
                }
                continue;
            }

            match i {
                'p' => {
                    self.board.pawns = set_bit(self.board.pawns, bit);
                    self.board.black = set_bit(self.board.black, bit);
                }
                'n' => {
                    self.board.knights = set_bit(self.board.knights, bit);
                    self.board.black = set_bit(self.board.black, bit);
                }
                'b' => {
                    self.board.bishops = set_bit(self.board.bishops, bit);
                    self.board.black = set_bit(self.board.black, bit);
                }
                'r' => {
                    self.board.rooks = set_bit(self.board.rooks, bit);
                    self.board.black = set_bit(self.board.black, bit);
                }
                'q' => {
                    self.board.queens = set_bit(self.board.queens, bit);
                    self.board.black = set_bit(self.board.black, bit);
                }
                'k' => {
                    self.board.kings = set_bit(self.board.kings, bit);
                    self.board.black = set_bit(self.board.black, bit);
                }

                'P' => {
                    self.board.pawns = set_bit(self.board.pawns, bit);
                    self.board.white = set_bit(self.board.white, bit);
                }
                'N' => {
                    self.board.knights = set_bit(self.board.knights, bit);
                    self.board.white = set_bit(self.board.white, bit);
                }
                'B' => {
                    self.board.bishops = set_bit(self.board.bishops, bit);
                    self.board.white = set_bit(self.board.white, bit);
                }
                'R' => {
                    self.board.rooks = set_bit(self.board.rooks, bit);
                    self.board.white = set_bit(self.board.white, bit);
                }
                'Q' => {
                    self.board.queens = set_bit(self.board.queens, bit);
                    self.board.white = set_bit(self.board.white, bit);
                }
                'K' => {
                    self.board.kings = set_bit(self.board.kings, bit);
                    self.board.white = set_bit(self.board.white, bit);
                }
                _ => {}
            }

            file += 1;
        }
        // we ignore the rest of the fen string (for now)
    }

    pub fn init(fen: String) -> Self {
        let mut out = Self {
            to_play: Color::White,
            board: Board::empty(),
        };

        out.setup_position(fen);

        out
    }
}
