extern crate rltk;
extern crate specs;
use super::{
    BlocksTile, CombatStats, Map, Monster, Name, Player, Position, Rect, Renderable, Viewshed,
    MAPWIDTH,
};
use rltk::{console, RandomNumberGenerator, RGB};
use specs::prelude::*;

const MAX_MONSTERS: i32 = 4;
const MAX_ITEMS: i32 = 2;

pub fn player(ecs: &mut World, player_x: i32, player_y: i32) -> Entity {
    ecs.create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(Name {
            name: "Player".to_string(),
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(CombatStats {
            max_hp: 33,
            hp: 33,
            defence: 1,
            power: 4,
        })
        .build()
}

pub fn populate_room(ecs: &mut World, room: &Rect) {
    console::log(&format!(
        "Spawning for room: ({},{})({},{})",
        room.x1, room.y1, room.x2, room.y2
    ));
    let mut monster_spawn_points: Vec<usize> = Vec::new();

    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        let num_monsters = rng.roll_dice(1, MAX_MONSTERS + 2) - 3;

        for _i in 0..num_monsters {
            let mut added = false;
            // Loop until a valid point to spawn is found
            loop {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAPWIDTH) + x;
                if !monster_spawn_points.contains(&idx) {
                    console::log(&format!("Spawning monster at: ({},{})", x, y));
                    monster_spawn_points.push(idx);
                    // Valid spawn found, break out of loop
                    break;
                }
            }
        }
    }

    for idx in monster_spawn_points.iter() {
        let x = (*idx % MAPWIDTH) as i32;
        let y = (*idx / MAPWIDTH) as i32;
        random_monster(ecs, x, y);
    }
}

pub fn random_monster(ecs: &mut World, x: i32, y: i32) {
    let roll: i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1, 2);
    }
    match roll {
        1 => orc(ecs, x, y),
        _ => goblin(ecs, x, y),
    }
}

fn orc(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, rltk::to_cp437('o'), "Orc");
}

fn goblin(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, rltk::to_cp437('g'), "Goblin");
}

fn monster<S: ToString>(ecs: &mut World, x: i32, y: i32, glyph: u8, name: S) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph,
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
        })
        .with(BlocksTile {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Monster {})
        .with(Name {
            name: name.to_string(),
        })
        .with(CombatStats {
            max_hp: 9,
            hp: 9,
            defence: 1,
            power: 4,
        })
        .build();
}
