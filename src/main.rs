use rltk::{console, Console, GameState, Point, Rltk, RGB};
use specs::prelude::*;

#[macro_use]
extern crate specs_derive;

mod components;
mod damage_system;
mod gamelog;
mod gui;
mod map;
mod map_indexing_system;
mod melee_combat_system;
mod monster_ai_system;
mod player;
mod rect;
mod visibility_system;

pub use components::*;
use damage_system::*;
use gamelog::*;
use gui::*;
pub use map::*;
use map_indexing_system::*;
use melee_combat_system::*;
use monster_ai_system::*;
use player::*;
use rect::*;
use visibility_system::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
    Examining,
}

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);
        let mut map_idx = MapIndexingSystem {};
        map_idx.run_now(&self.ecs);
        let mut damage = DamageSystem {};
        damage.run_now(&self.ecs);
        let mut melee = MeleeCombatSystem {};
        melee.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        let mut current_runstate: RunState;
        {
            let runstate = self.ecs.fetch::<RunState>();
            current_runstate = *runstate;
        }

        match current_runstate {
            RunState::PreRun => {
                self.run_systems();
                current_runstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput | RunState::Examining => {
                self.run_systems();
                current_runstate = player_input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                current_runstate = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                current_runstate = RunState::AwaitingInput;
            }
        }

        {
            let mut runstate = self.ecs.write_resource::<RunState>();
            *runstate = current_runstate
        }

        DamageSystem::delete_the_dead(&mut self.ecs);

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

        gui::draw_ui(&self.ecs, ctx);
    }
}

fn main() {
    use rltk::RltkBuilder;
    let mut context = RltkBuilder::simple80x50().with_title("Rustlike").build();

    let mut gs = State { ecs: World::new() };

    // Register Components
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();

    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    let player_entity = gs
        .ecs
        .create_entity()
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
        .build();

    let mut rng = rltk::RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();
        let glyph: u8;
        let name: String;

        match rng.roll_dice(1, 2) {
            1 => {
                glyph = rltk::to_cp437('g');
                name = "Goblin".to_string();
            }
            _ => {
                glyph = rltk::to_cp437('o');
                name = "Orc".to_string();
            }
        }

        gs.ecs
            .create_entity()
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
                name: format!("{} #{}", &name, i),
            })
            .with(CombatStats {
                max_hp: 9,
                hp: 9,
                defence: 1,
                power: 4,
            })
            .build();
    }

    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(Cursor { x: 0, y: 0 });
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::PreRun);
    gs.ecs.insert(gamelog::GameLog {
        entries: vec!["Welcome to Rustlike".to_string()],
    });

    rltk::main_loop(context, gs);
}
