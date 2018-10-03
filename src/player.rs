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
        let renderer = RenderComponent::new(PLAYER_CHAR, Some(colors::DESATURATED_RED), Some(PLAYER_COLOR));
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

    fn attack(&mut self, other: usize, objects: &mut Vec<Object>) {
        println!("Player attacks {}", objects[other].name);
    }

    pub fn move_or_attack(&mut self, dx: i32, dy: i32, map: &Map, objects: &mut Vec<Object>) {
        let x = self.position().0 + dx;
        let y = self.position().1 + dy;

        let target_id = objects.iter().position(|object| {
            object.position() == (x,y)
        });

        match target_id {
            Some(target_id) => self.attack(target_id, objects),
            None => self.transform.move_by(-1, dx, dy, map, objects),
        }
    }

    pub fn position(&self) -> (i32, i32) {
        self.transform.position()
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.transform.set_position(x,y);
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
}
