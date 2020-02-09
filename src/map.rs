use rltk::{Console, RandomNumberGenerator, Rltk, RGB};
use std::cmp::{max, min};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    WALL,
    FLOOR,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::FLOOR; 80 * 50];

    // Make the boundary walls
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::WALL;
        map[xy_idx(x, 49)] = TileType::WALL;
    }
    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::WALL;
        map[xy_idx(79, y)] = TileType::WALL;
    }

    // Randomly place a bunch of walls.
    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);

        // Make sure not to place a wall on the player.
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::WALL;
        }
    }

    map
}

pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter() {
        match tile {
            TileType::FLOOR => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('.'),
                );
            }
            TileType::WALL => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0.0, 0.0, 0.0),
                    rltk::to_cp437('#'),
                );
            }
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
