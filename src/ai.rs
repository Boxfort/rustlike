use super::object::Object;
use super::map::Map;
use super::player::Player;
use super::components::AiComponent;

// Useful utility functions for all AI to use.
// TODO: think about module structure, this could be broken out
// into a utils file probably.

/// Gets the next location to move towards to reach the target.
fn move_towards(object: &mut Object,
                target: (i32, i32),
                map: &Map,
                objects: &mut Vec<Object>) {
    // Get the distance to the target from the current positon
    let dx = target.0 - object.position().0;
    let dy = target.1 - object.position().1;
    let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

    // normalize the distance to length 1 (preserving direction)
    // and then round it and conver to integer as to restrict
    // movement ot hte map grid.
    let dx = (dx as f32 / distance).round() as i32;
    let dy = (dy as f32 / distance).round() as i32;
    object.move_by(dx, dy, map, objects);
}

#[derive(Clone)]
pub struct AiMonster;

impl AiComponent for AiMonster {
    fn take_turn(&self,
                 object: &mut Object,
                 map: &mut Map,
                 objects: &mut Vec<Object>,
                 player: &mut Player) {
        // a basic monster takes its turn. If you can see it, it can see you
        if map.is_in_fov(object.position()) {
            if object.distance_to(player.position()) >= 2.0 {
                // move towards player if far away
                move_towards(object, player.position(), map, objects);
            } else if player.stats().hp > 0 {
                // close enough, attack! (if the player is still alive.)
                object.attack(player);
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
