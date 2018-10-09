extern crate tcod;

use item::tcod::Console;
use item::tcod::Color;
use super::components::{
                    RenderComponent,
                    TransformComponent,
                };

#[derive(Clone)]
pub struct Item {
    renderer: RenderComponent,
    transform: TransformComponent,
    pub name: String,
}

impl Item {
    pub fn new(x: i32,
               y: i32,
               character: char,
               background_color: Option<Color>,
               foreground_color: Option<Color>,
               name: String)
        -> Self {

            let renderer = RenderComponent::new(character, background_color, foreground_color);
            let transform = TransformComponent::new(x, y);

        Item {
            renderer,
            transform,
            name,
        }
    }

    pub fn position(&self) -> (i32, i32) {
        self.transform.position()
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.transform.set_position(x,y);
    }

    pub fn draw(&self, console: &mut Console) {
        self.renderer.draw(console, self.transform.position().0, self.transform.position().1);
    }

    pub fn clear(&self, console: &mut Console) {
        self.renderer.clear(console, self.transform.position().0, self.transform.position().1);
    }
}
