extern crate specs;
use super::{BlocksTile, Map, Position};
use specs::prelude::*;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blockers, entities) = data;

        map.populate_blocked();
        map.clear_content_index();

        for (entity, position) in (&entities, &position).join() {
            let idx = map.xy_idx(position.x, position.y);

            // If the entity is blocking update the blocking list
            if blockers.get(entity).is_some() {
                map.blocked[idx] = true;
            }

            // Push the entity into the appropriate index. It's a copy type so we don't need to clone it.
            map.tile_content[idx].push(entity);
        }
    }
}
