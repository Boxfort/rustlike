extern crate specs;
use super::{Monster, Name, Point, Position, Viewshed};
use specs::prelude::*;

extern crate rltk;
use rltk::console;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadExpect<'a, Point>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_pos, viewshed, monster, name) = data;

        for (viewshed, _monster, name) in (&viewshed, &monster, &name).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                console::log(&format!("{} shouts insults", name.name));
            }
        }
    }
}
