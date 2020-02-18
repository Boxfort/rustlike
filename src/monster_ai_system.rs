extern crate specs;
use super::{Map, Monster, Name, Point, Position, RunState, Viewshed, WantsToMelee};
use specs::prelude::*;

extern crate rltk;
use rltk::console;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Map>,
        ReadExpect<'a, Point>,
        ReadExpect<'a, Entity>,
        ReadExpect<'a, RunState>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, WantsToMelee>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            map,
            player_pos,
            player_entity,
            runstate,
            entities,
            mut viewshed,
            monster,
            mut position,
            mut wants_to_melee,
        ) = data;

        // Make sure AI only runs in the correct game state.
        if *runstate != RunState::MonsterTurn {
            return;
        }

        for (entity, mut viewshed, _monster, mut pos) in
            (&entities, &mut viewshed, &monster, &mut position).join()
        {
            let distance =
                rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);

            // If the monster is close enough to melee, then do that.
            if distance < 1.5 {
                wants_to_melee
                    .insert(
                        entity,
                        WantsToMelee {
                            target: *player_entity,
                        },
                    )
                    .expect("Unable to insert attack.");

            // If the monster can see the player
            } else if viewshed.visible_tiles.contains(&*player_pos) {
                let path = rltk::a_star_search(
                    map.xy_idx(pos.x, pos.y) as i32,
                    map.xy_idx(player_pos.x, player_pos.y) as i32,
                    &*map,
                );

                // If we found a way to the player and we're not right next
                if path.success && path.steps.len() > 1 {
                    // Move to the first position in the path
                    let (x, y) = map.idx_to_xy(path.steps[1]);

                    // Set the monsters new position
                    pos.x = x;
                    pos.y = y;

                    // The monster has moved, recalculate it's sight.
                    viewshed.dirty = true;
                }
            }
        }
    }
}
