extern crate tcod;

use object::tcod::console::*;
use object::tcod::Color;
use super::map::Map;
use super::player::Player;
use super::components::{
                    RenderComponent,
                    TransformComponent,
                    StatsComponent,
                    AiComponent,
                };

#[derive(Clone)]
pub struct Object{
    renderer: RenderComponent,
    pub transform: TransformComponent,
    stats: Option<StatsComponent>,
    pub ai: Option<Box<AiComponent>>,
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
            stats: Option<StatsComponent>,
            ai: Option<Box<AiComponent>>,
            name: String,
            blocking: bool) -> Self {

        let renderer = RenderComponent::new(character,
                                            background_color,
                                            foreground_color);
        let transform = TransformComponent::new(x, y);

        Object {
            renderer,
            transform,
            stats,
            ai,
            name: name,
            blocking: blocking,
            alive: false,
        }
    }

    pub fn position(&self) -> (i32, i32) {
        self.transform.position()
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.transform.set_position(x,y);
    }

    pub fn distance_to(&self, target: (i32, i32)) -> f32 {
        self.transform.distance_to(target)
    }

    pub fn draw(&self, con: &mut Console) {
        self.renderer.draw(con, self.transform.position().0, self.transform.position().1);
    }

    pub fn clear(&self, con: &mut Console) {
        self.renderer.clear(con, self.transform.position().0, self.transform.position().1);
    }
}
