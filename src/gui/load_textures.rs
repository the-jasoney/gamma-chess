use std::collections::HashMap;
use macroquad::prelude::*;
use crate::piece::*;

macro_rules! img {
    ($path:literal) => {
        Image::from_file_with_format(
            include_bytes!($path),
            None
        )
    };
}

macro_rules! empty_img {
    () => {
        Image::gen_image_color(
            128,
            128,
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
        )
    };
}

macro_rules! map(
    { $($key:expr => $value:expr,) + } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

pub fn load_textures() -> HashMap<u8, Image> {
    map! {
        EMPTY => empty_img!(),
        PAWN | WHITE_ => img!("../../assets/white-pawn.png"),
        BISHOP | WHITE_ => img!("../../assets/white-bishop.png"),
        KNIGHT | WHITE_ => img!("../../assets/white-knight.png"),
        ROOK | WHITE_ => img!("../../assets/white-rook.png"),
        QUEEN | WHITE_ => img!("../../assets/white-queen.png"),
        KING | WHITE_ => img!("../../assets/white-king.png"),

        PAWN | BLACK_ => img!("../../assets/white-pawn.png"),
        BISHOP | BLACK_ => img!("../../assets/white-bishop.png"),
        KNIGHT | BLACK_ => img!("../../assets/white-knight.png"),
        ROOK | BLACK_ => img!("../../assets/white-rook.png"),
        QUEEN | BLACK_ => img!("../../assets/white-queen.png"),
        KING | BLACK_ => img!("../../assets/white-king.png"),
    }
}
