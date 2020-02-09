use rltk::{Console, GameState, Rltk, RGB};
use specs::prelude::*;

#[macro_use]
extern crate specs_derive;

mod components;
mod map;
mod monster_ai_system;
mod player;
mod rect;
mod visibility_system;

pub use components::*;
pub use map::*;
use monster_ai_system::*;
use player::*;
use rect::*;
use visibility_system::*;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    PAUSED,
    RUNNING,
}

pub struct State {
    pub ecs: World,
    pub runstate: RunState,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        if self.runstate == RunState::RUNNING {
            self.run_systems();
            self.runstate = RunState::PAUSED;
        } else {
            self.runstate = player_input(self, ctx);
        }

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

fn main() {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Hello World.")
        .build();
    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::RUNNING,
    };

    // Register Components
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    let map = Map::new_map_rooms_and_corridors();

    gs.ecs
        .create_entity()
        .with(Position {
            x: map.rooms[0].center().0,
            y: map.rooms[0].center().1,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build();

    let mut rng = rltk::RandomNumberGenerator::new();
    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();
        let glyph: u8;

        match rng.roll_dice(1, 2) {
            1 => glyph = rltk::to_cp437('g'),
            _ => glyph = rltk::to_cp437('o'),
        }

        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Monster {})
            .build();
    }

    gs.ecs.insert(map);

    rltk::main_loop(context, gs);
}
