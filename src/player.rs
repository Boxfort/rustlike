extern crate tcod;

use player::tcod::console::*;
use player::tcod::colors::Color;
use player::tcod::colors;
use super::map::Map;
use super::object::Object;
use super::components::{
                    RenderComponent,
                    TransformComponent
                };

const PLAYER_CHAR: char = '@';
const PLAYER_COLOR: Color = colors::WHITE;

pub struct Player {
    renderer: RenderComponent,
    transform: TransformComponent,
    x: i32,
    y: i32,
    alive: bool,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        let renderer = RenderComponent::new(PLAYER_CHAR, None, Some(PLAYER_COLOR));
        let transform = TransformComponent::new(x, y);

        Player {
            renderer,
            transform,
            x,
            y,
            alive: true,
        }
    }

    pub fn attack(&mut self, other: usize, objects: &mut Vec<Object>) {
        println!("Player attacks {}", objects[other].name);
    }

    pub fn move_by(&mut self, dx: i32, dy: i32, map: &Map, objects: &Vec<Object>) {
        if !self.is_blocked(self.transform.position().0 + dx,
                            self.transform.position().1 + dy,
                            map,
                            objects) {
            let position = self.transform.position();
            self.transform.set_position(position.0 + dx,
                                        position.1 + dy);
        }
    }

    fn is_blocked(&self, x: i32, y: i32, map: &Map, objects: &Vec<Object>) -> bool {
        if map.tiles[x as usize][y as usize].blocked {
            return true;
        }

        objects.iter().any(|object| {
            object.blocking && object.position() == (x, y)
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.transform.position()
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.transform.set_position(x,y);
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
