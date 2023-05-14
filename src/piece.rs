//! Pieces as u8. In order to represent a piece, say a black pawn, you OR its components: i.e. PAWN | BLACK

pub const EMPTY: u8 = 0;

pub const PAWN: u8 = 1;
pub const BISHOP: u8 = 2;
pub const KNIGHT: u8 = 3;
pub const ROOK: u8 = 4;
pub const QUEEN: u8 = 5;
pub const KING: u8 = 6;

pub const WHITE: u8 = 8;
pub const BLACK: u8 = 16;
