extern crate rltk;
use super::Rect;
use rltk::{Algorithm2D, BaseMap, Point};
use rltk::{Console, RandomNumberGenerator, Rltk, RGB};
use specs::prelude::*;
use std::cmp::{max, min};

pub const MAPWIDTH: usize = 80;
pub const MAPHEIGHT: usize = 43;
pub const MAPCOUNT: usize = MAPWIDTH * MAPHEIGHT;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    WALL,
    FLOOR,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub tile_content: Vec<Vec<Entity>>,
}

impl Map {
    /// Determines if the tile at (x, y) is valid to be travelled to.
    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if !self.is_in_bounds(x, y) {
            return false;
        }

        let idx = self.xy_idx(x, y);
        !self.blocked[idx]
    }

    /// Returns true if the (x,y) coordinate is within the bounds of the map
    pub fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }

        true
    }

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::WALL
        }
    }

    /// Convert a usize index into an (x, y) point on the map
    pub fn idx_to_xy(&self, idx: usize) -> (i32, i32) {
        (idx as i32 % self.width, idx as i32 / self.width)
    }

    /// Convert an (x, y) position into an index for the map
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        ((y * self.width) + x) as usize
    }

    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }

    pub fn new_map_rooms_and_corridors() -> Self {
        let mut map = Map {
            tiles: vec![TileType::WALL; MAPCOUNT],
            rooms: Vec::new(),
            width: MAPWIDTH as i32,
            height: MAPHEIGHT as i32,
            revealed_tiles: vec![false; MAPCOUNT],
            visible_tiles: vec![false; MAPCOUNT],
            blocked: vec![false; MAPCOUNT],
            tile_content: vec![Vec::new(); MAPCOUNT],
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        'outer: for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width - w - 1) - 1;
            let y = rng.roll_dice(1, map.height - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    // Skip to the next room
                    continue 'outer;
                }
            }
            map.apply_room_to_map(&new_room);

            if !map.rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                if rng.range(0, 2) == 1 {
                    map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                    map.apply_vertical_tunnel(prev_y, new_y, new_x);
                } else {
                    map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                }
            }

            map.rooms.push(new_room);
        }

        map
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::FLOOR;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < MAPCOUNT {
                self.tiles[idx as usize] = TileType::FLOOR;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < MAPCOUNT {
                self.tiles[idx as usize] = TileType::FLOOR;
            }
        }
    }
}

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let mut y = 0;
    let mut x = 0;
    for (idx, tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[idx] {
            let glyph;
            let mut fg;
            match tile {
                TileType::FLOOR => {
                    glyph = rltk::to_cp437('.');
                    fg = RGB::from_f32(0.0, 0.5, 0.5);
                }
                TileType::WALL => {
                    glyph = rltk::to_cp437('#');
                    fg = RGB::from_f32(0.0, 1.0, 0.0);
                }
            }

            if !map.visible_tiles[idx] {
                fg = fg.to_greyscale()
            }
            ctx.set(x, y, fg, RGB::from_f32(0.0, 0.0, 0.0), glyph);
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::WALL
    }

    fn get_available_exits(&self, idx: usize) -> Vec<(usize, f32)> {
        let mut exits: Vec<(usize, f32)> = Vec::new();
        let (x, y) = self.idx_to_xy(idx);
        let w = self.width as usize;

        // Cardinal Directions
        // Left
        if self.is_exit_valid(x - 1, y) {
            exits.push((idx - 1, 1.0))
        };
        // Right
        if self.is_exit_valid(x + 1, y) {
            exits.push((idx + 1, 1.0))
        };
        // Up
        if self.is_exit_valid(x, y - 1) {
            exits.push((idx - w, 1.0))
        };
        // Down
        if self.is_exit_valid(x, y + 1) {
            exits.push((idx + w, 1.0))
        };

        // Diagonals
        // Up Left
        if self.is_exit_valid(x - 1, y - 1) {
            exits.push((idx - w, 1.45))
        };
        // Up Right
        if self.is_exit_valid(x + 1, y - 1) {
            exits.push((idx - w, 1.45))
        };
        // Down Left
        if self.is_exit_valid(x - 1, y + 1) {
            exits.push((idx + w, 1.45))
        };
        // Down Right
        if self.is_exit_valid(x + 1, y + 1) {
            exits.push((idx + w, 1.45))
        };

        exits
    }
}
