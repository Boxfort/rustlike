extern crate tcod;

use player::tcod::console::*;
use player::tcod::colors::Color;
use player::tcod::colors;
use super::map::Map;
use super::object::Object;
use super::components::{
                    RenderComponent,
                    TransformComponent,
                    StatsComponent,
                };

const PLAYER_CHAR: char = '@';
const PLAYER_COLOR: Color = colors::WHITE;

pub struct Player {
    renderer: RenderComponent,
    transform: TransformComponent,
    stats: StatsComponent,
    x: i32,
    y: i32,
    alive: bool,
}

impl Player {
    pub fn new(x: i32, y: i32, stats: StatsComponent) -> Self {
        let renderer = RenderComponent::new(PLAYER_CHAR, None, Some(PLAYER_COLOR));
        let transform = TransformComponent::new(x, y);

        Player {
            renderer,
            transform,
            stats,
            x,
            y,
            alive: true,
        }
    }

    pub fn move_or_attack(&mut self, dx: i32, dy: i32, map: &Map, objects: &mut Vec<Object>) {
        let x = self.position().0 + dx;
        let y = self.position().1 + dy;

        let target_id = objects.iter().position(|object| {
            match object.is_alive() {
                true => object.position() == (x,y),
                false => false,
            }
        });

        match target_id {
            Some(target_id) => self.attack(&mut objects[target_id]),
            None => self.transform.move_by(dx, dy, map, objects),
        }
    }

    pub fn position(&self) -> (i32, i32) {
        self.transform.position()
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.transform.set_position(x,y);
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.stats.hp -= damage;

        if self.stats.hp <= 0 {
            self.on_death();
        }
    }

    pub fn attack(&self, object: &mut Object) {
        if object.stats().is_some() {
            // Calculate damage
            let damage = self.stats.power - object.stats().as_ref().unwrap().defence;

            // Apply damage.
            object.take_damage(damage);
        }
    }

    pub fn stats(&self) -> &StatsComponent {
        &self.stats
    }

    pub fn draw(&self, con: &mut Console) {
        self.renderer.draw(con, self.transform.position().0, self.transform.position().1);
    }

    pub fn clear(&self, con: &mut Console) {
        self.renderer.clear(con, self.transform.position().0, self.transform.position().1);
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    fn on_death(&mut self) {
        self.alive = false;
        println!("Player dead.")
    }
}
