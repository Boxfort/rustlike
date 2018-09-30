extern crate tcod;

use tile::tcod::{Color, colors};

#[derive(Clone)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
    pub character: char,
    pub background_color: Color,
    pub foreground_color: Color,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
            character: ' ',
            background_color: colors::BLACK,
            foreground_color: colors::BLACK,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
            character: '#',
            background_color: colors::GREY,
            foreground_color: colors::GREY,
        }
    }
}

