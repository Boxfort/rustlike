extern crate tcod;

use tile::tcod::{Color, colors};

#[derive(Clone)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
    pub character: char,
    pub color: Color,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
            character: ' ',
            color: colors::BLACK,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
            character: '#',
            color: colors::GREY,
        }
    }
}

