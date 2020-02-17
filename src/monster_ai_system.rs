extern crate specs;
use super::{Map, Monster, Name, Point, Position, Viewshed};
use specs::prelude::*;

extern crate rltk;
use rltk::console;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, player_pos, mut viewshed, monster, name, mut position) = data;

        for (mut viewshed, _monster, name, mut pos) in
            (&mut viewshed, &monster, &name, &mut position).join()
        {
            // If the monster can see the player
            if viewshed.visible_tiles.contains(&*player_pos) {
                let distance =
                    rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);

                if distance < 1.5 {
                    console::log(&format!("{} shouts insults", name.name));
                } else {
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
}
