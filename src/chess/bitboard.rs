use core::fmt;
use std::fmt::Formatter;

use crate::{chess::bit_manip::get_bit, piece::*};

#[derive(Debug)]
pub struct Board {
    // piece masks
    pub pawns: u64,
    pub knights: u64,
    pub bishops: u64,
    pub rooks: u64,
    pub queens: u64,
    pub kings: u64,

    // color masks
    pub black: u64,
    pub white: u64,

    // special masks
    pub moved: u64,
    pub en_passant_capture: u64,
}

impl Board {
    pub fn empty() -> Self {
        Self {
            pawns: 0,
            knights: 0,
            bishops: 0,
            rooks: 0,
            queens: 0,
            kings: 0,
            black: 0,
            white: 0,
            moved: 0,
            en_passant_capture: 0,
        }
    }

    pub fn as_piece_array(&self) -> [u8; 64] {
        let mut output: [u8; 64] = [0; 64];
        for i in 0..64 {
            let mut piece = EMPTY;

            if get_bit(self.pawns, i) {
                piece |= PAWN;
            }

            if get_bit(self.knights, i) {
                piece |= KNIGHT;
            }

            if get_bit(self.bishops, i) {
                piece |= BISHOP;
            }

            if get_bit(self.rooks, i) {
                piece |= ROOK;
            }

            if get_bit(self.queens, i) {
                piece |= QUEEN;
            }

            if get_bit(self.kings, i) {
                piece |= KING;
            }

            if get_bit(self.white, i) {
                piece |= WHITE;
            }

            if get_bit(self.black, i) {
                piece |= BLACK;
            }

            output[i as usize] = piece;
        }

        output
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_piece_array())
    }
}
