extern crate tcod;

use object::tcod::console::*;
use object::tcod::Color;
use super::map::Map;
use super::components::{
                    RenderComponent,
                    TransformComponent
                };

pub struct Object {
    renderer: RenderComponent,
    transform: TransformComponent,
    pub name: String,
    pub blocking: bool,
    pub alive: bool,
}

impl Object {
    pub fn new(
            x: i32,
            y: i32,
            character: char,
            background_color: Option<Color>,
            foreground_color: Option<Color>,
            name: String,
            blocking: bool) -> Self {

        let renderer = RenderComponent::new(character,
                                            background_color,
                                            foreground_color);
        let transform = TransformComponent::new(x, y);

        Object {
            renderer,
            transform,
            name: name,
            blocking: blocking,
            alive: false,
        }
    }

    pub fn attack(&mut self, other: usize, objects: &mut Vec<Object>) {
        println!("{} attacks {}", self.name, objects[other].name);
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
}
