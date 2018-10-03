extern crate tcod;
extern crate rand;

use super::tile::Tile;
use super::object::Object;
use super::components::{StatsComponent};
use super::ai::*;
use map::tcod::console::*;
use map::tcod::map::{Map as FovMap, FovAlgorithm};
use map::tcod::{Color, colors};
use map::rand::distributions::{Distribution, Uniform};
use std::cmp;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;
const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const FOV_LIGHT_WALLS: bool = true;
const UNSEEN_COLOR: Color = colors::DARKER_SEPIA;
const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;
const MAX_ROOM_MONSTERS: i32 = 3;

pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub width: i32,
    pub height: i32,
    fov_map: FovMap,
}

struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect { x1: x, y1: y, x2: x + w, y2: y + h }
    }

    /// Gets the x and y coordinates of the center of the rect
    /// as a tuple.
    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;

        (center_x, center_y)
    }

    /// Returns true if this rectancle intersects with other.
    pub fn intersects_with(&self, other: &Rect) -> bool {
        (self.x1 <= other.x2) && (self.x2 >= other.x1) &&
            (self.y1 <= other.y2) && (self.y2 >= other.y1)
    }
}

impl Map {
    pub fn new() -> Self {
        let mut map = Map {
            tiles: vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize],
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            fov_map: FovMap::new(MAP_WIDTH, MAP_HEIGHT),
        };

        map.generate_fov_map();

        map
    }

    pub fn draw(&mut self, con: &mut Console) {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let visible = self.fov_map.is_in_fov(x, y);
                let tile = &mut self.tiles[x as usize][y as usize];

                let (bg_color, fg_color) = match visible {
                    true => {
                        tile.explored = true;
                        (tile.background_color, tile.foreground_color)
                    },
                    false => (UNSEEN_COLOR, UNSEEN_COLOR),
                };

                if tile.explored {
                    con.set_char_background(x, y, bg_color, BackgroundFlag::Set);
                }
            }
        }
    }

    pub fn calculate_fov(&mut self, pos: (i32, i32), radius: i32) {
        self.fov_map.compute_fov(pos.0, pos.1, radius, FOV_LIGHT_WALLS, FOV_ALGO);
    }

    /// Returns true if the specified position is in the active field of view
    pub fn is_in_fov(&self, pos: (i32, i32)) -> bool {
        self.fov_map.is_in_fov(pos.0, pos.1)
    }

    /// Randomly generates a new map.
    ///
    /// Generates a map of rectangular rooms between
    /// ROOM_MIN_SIZE and ROOM_MAX_SIZE size and up to
    /// MAX_ROOMS rooms.
    pub fn generate_map(&mut self, objects: &mut Vec<Object>) -> (i32, i32) {
        self.tiles = vec![vec![Tile::wall(); self.height as usize]; self.width as usize];

        let mut rooms : Vec<Rect> = vec![];
        let mut rng = rand::thread_rng();

        let dimen = Uniform::new(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let mut starting_position = (0,0);

        for _ in 0..MAX_ROOMS {
            let w = dimen.sample(&mut rng);
            let h = dimen.sample(&mut rng);

            let rand_x = Uniform::new(0,MAP_HEIGHT - w);
            let rand_y = Uniform::new(0,MAP_HEIGHT - h);

            let x = rand_x.sample(&mut rng);
            let y = rand_y.sample(&mut rng);

            let room = Rect::new(x, y, w, h);
            let failed = rooms.iter().any(|other| room.intersects_with(other));

            if !failed {
                self.create_room(&room);
                let (new_x, new_y) = room.center();
                if rooms.is_empty() {
                    starting_position = (new_x, new_y);
                } else {
                    let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                    if rand::random() {
                        self.create_h_tunnel(prev_x, new_x, prev_y);
                        self.create_v_tunnel(prev_y, new_y, new_x);
                    } else {
                        self.create_v_tunnel(prev_y, new_y, prev_x);
                        self.create_h_tunnel(prev_x, new_x, new_y);
                    }

                    self.place_objects(&room, objects);
                }
                rooms.push(room);
            }
        }

        self.generate_fov_map();
        starting_position
    }

    /// Randomly places objects into the specified room
    fn place_objects(&mut self, room: &Rect, objects: &mut Vec<Object>) {
        let mut rng = rand::thread_rng();
        let rand_monsters = Uniform::new(0, MAX_ROOM_MONSTERS + 1);
        let rand_x = Uniform::new(room.x1 + 1, room.x2);
        let rand_y = Uniform::new(room.y1 + 1, room.y2);

        for _ in 0..rand_monsters.sample(&mut rng) {
            let x = rand_x.sample(&mut rng);
            let y = rand_y.sample(&mut rng);

            let mut monster = if rand::random::<f32>() < 0.8 {
                Object::new(x,
                            y,
                            'o',
                            None,
                            Some(colors::DESATURATED_GREEN),
                            Some(StatsComponent::new(10,10,0,3)),
                            Some(Box::new(AiMonster)),
                            "Orc".to_string(),
                            true)
            } else {
                Object::new(x,
                            y,
                            'T',
                            None,
                            Some(colors::DARKER_GREEN),
                            Some(StatsComponent::new(10,10,0,3)),
                            Some(Box::new(AiMonster)),
                            "Troll".to_string(),
                            true)
            };

            objects.push(monster);
        }
    }

    /// Updates the FOV_MAP so that the fov can be correctly
    /// calculated for sight blocking tiles.
    fn generate_fov_map(&mut self) {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                self.fov_map.set(x, y,
                !self.tiles[x as usize][y as usize].block_sight,
                !self.tiles[x as usize][y as usize].blocked);
            }
        }
    }

    /// Add the specified rect to the map tiles
    fn create_room(&mut self, room: &Rect) {
        for x in (room.x1 + 1)..room.x2 {
            for y in (room.y1 + 1)..room.y2 {
                self.tiles[x as usize][y as usize] = Tile::empty();
            }
        }
    }

    /// Create a horizontal tunnel at height y, from x1, to x2
    fn create_h_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
            self.tiles[x as usize][y as usize] = Tile::empty();
        }
    }

    /// Create a vertical tunnel at width x, from y1, to y2
    fn create_v_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
            self.tiles[x as usize][y as usize] = Tile::empty();
        }
    }
}

