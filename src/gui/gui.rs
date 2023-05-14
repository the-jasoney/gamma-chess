use std::collections::HashMap;

use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawParam, Drawable, Image, Rect};
use ggez::{Context, GameResult};

use map_macro::hash_map;

use crate::piece::*;

pub struct Game {
    piece_assets: HashMap<u8, Image>,
    board_asset: Image,
}

impl Game {
    pub fn new(ctx: &mut Context) -> Game {
        // Load/create resources such as images here.

        ctx.gfx.set_window_title("Γ-chess pre-alpha build");
        ctx.gfx.set_resizable(true).unwrap();

        Game {
            piece_assets: hash_map! {
                EMPTY => Image::from_path(ctx, "/empty.png").unwrap(),

                PAWN | WHITE => Image::from_path(ctx, "/white-pawn.png").unwrap(),
                BISHOP | WHITE => Image::from_path(ctx, "/white-bishop.png").unwrap(),
                KNIGHT | WHITE => Image::from_path(ctx, "/white-knight.png").unwrap(),
                ROOK | WHITE => Image::from_path(ctx, "/white-rook.png").unwrap(),
                QUEEN | WHITE => Image::from_path(ctx, "/white-queen.png").unwrap(),
                KING | WHITE => Image::from_path(ctx, "/white-king.png").unwrap(),

                PAWN | BLACK => Image::from_path(ctx, "/black-pawn.png").unwrap(),
                BISHOP | BLACK => Image::from_path(ctx, "/black-bishop.png").unwrap(),
                KNIGHT | BLACK => Image::from_path(ctx, "/black-knight.png").unwrap(),
                ROOK | BLACK => Image::from_path(ctx, "/black-rook.png").unwrap(),
                QUEEN | BLACK => Image::from_path(ctx, "/black-queen.png").unwrap(),
                KING | BLACK => Image::from_path(ctx, "/black-king.png").unwrap(),
            },
            board_asset: Image::from_path(ctx, "/board.png").unwrap(),
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::new(0.5, 0.5, 0.5, 1.0));

        let w = ctx.gfx.size().0;
        let h = ctx.gfx.size().1;
        let minwh = w.min(h);

        canvas.set_screen_coordinates(Rect::new(
            0.,
            0.,
            1024.0_f32.max(1024. * (w / minwh)),
            1024.0_f32.max(1024. * (h / minwh)),
        ));

        self.board_asset.draw(&mut canvas, DrawParam::default());
        self.piece_assets.get(&(PAWN | BLACK)).unwrap().draw(
            &mut canvas,
            DrawParam::default(),
        );

        canvas.finish(ctx)?;

        Ok(())
    }
}
