use rltk::{Console, GameState, Point, Rltk};
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
mod spawner;
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
use rltk::RltkBuilder;
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
    let context = RltkBuilder::simple80x50().with_title("Rustlike").build();

    let mut gs = State { ecs: World::new() };

    register_components(&mut gs.ecs);

    gs.ecs.insert(rltk::RandomNumberGenerator::new());

    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    let player_entity = spawner::player(&mut gs.ecs, player_x, player_y);

    for room in map.rooms.iter().skip(1) {
        spawner::populate_room(&mut gs.ecs, room);
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

/// Register all the components that we need with the ECS
fn register_components(ecs: &mut World) {
    ecs.register::<Position>();
    ecs.register::<Renderable>();
    ecs.register::<Monster>();
    ecs.register::<Player>();
    ecs.register::<Viewshed>();
    ecs.register::<Name>();
    ecs.register::<BlocksTile>();
    ecs.register::<CombatStats>();
    ecs.register::<WantsToMelee>();
    ecs.register::<SufferDamage>();
    ecs.register::<Item>();
    ecs.register::<Potion>();
}
