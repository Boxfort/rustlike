extern crate tcod;

use super::tile::Tile;
use map::tcod::console::*;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn new() -> Self {
        let mut map = Map {
            tiles: vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize],
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
        };

        map.tiles[30][22] = Tile::wall();
        map.tiles[50][22] = Tile::wall();

        map
    }

    pub fn draw(&self, con: &mut Console) {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let wall = &self.tiles[x as usize][y as usize];
                con.set_char_background(x, y, wall.color, BackgroundFlag::Set);
            }
        }
    }
}

