use super::object::Object;
use super::map::Map;
use super::player::Player;
use super::components::AiComponent;

// Useful utility functions for all AI to use.
// TODO: think about module structure, this could be broken out
// into a utils file probably.

/// Gets the next location to move towards to reach the target.
fn move_towards(id: usize,
                target: (i32, i32),
                map: &Map,
                objects: &mut Vec<Object>) {
    // Get the distance to the target from the current positon
    let dx = target.0 - objects[id].position().0;
    let dy = target.1 - objects[id].position().1;
    let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

    // normalize the distance to length 1 (preserving direction)
    // and then round it and conver to integer as to restrict
    // movement ot hte map grid.
    let dx = (dx as f32 / distance).round() as i32;
    let dy = (dy as f32 / distance).round() as i32;
    // FIXME: PLEASE
    objects[id].transform().move_by(id as i32, dx, dy, map, objects);
}

#[derive(Clone)]
pub struct AiMonster;

impl AiComponent for AiMonster {
    fn take_turn(&self,
                 monster_id: usize,
                 map: &mut Map,
                 objects: &mut Vec<Object>,
                 player: &mut Player) {
        // a basic monster takes its turn. If you can see it, it can see you
        if map.is_in_fov(objects[monster_id].position()) {
            if objects[monster_id].distance_to(player.position()) >= 2.0 {
                // move towards player if far away
                move_towards(monster_id, player.position(), map, objects);
            } else if player.stats().hp > 0 {
                // close enough, attack! (if the player is still alive.)
                println!("The attack of the {} bounces off your shiny metal armor!", objects[monster_id].name);
            }
        }
    }

    fn box_clone(&self) -> Box<AiComponent> {
        Box::new((*self).clone())
    }
}

impl Clone for Box<AiComponent> {
    fn clone(&self) -> Box<AiComponent> {
        self.box_clone()
    }
}
