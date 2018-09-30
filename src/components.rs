extern crate tcod;

use components::tcod::console::Console;
use components::tcod::console::BackgroundFlag;
use components::tcod::colors::Color;

pub struct RenderComponent {
    character: char,
    background_color: Option<Color>,
    foreground_color: Option<Color>,
}

pub struct TransformComponent {
    x: i32,
    y: i32,
}

impl RenderComponent {
    pub fn new(character: char,
               background_color: Option<Color>,
               foreground_color: Option<Color>)
        -> Self {
        RenderComponent {
            character,
            background_color,
            foreground_color,
        }
    }

    pub fn draw(&self, con: &mut Console, x: i32, y: i32) {

        let mut flag = BackgroundFlag::None;

        if self.background_color.is_some() {
            con.set_default_background(self.background_color.unwrap());
            flag = BackgroundFlag::Set;
        }

        if self.foreground_color.is_some() {
            con.set_default_foreground(self.foreground_color.unwrap());
            con.put_char(x, y, self.character, flag);
        }

    }

    pub fn clear(&self, con: &mut Console, x: i32, y: i32) {
        con.put_char(x, y, ' ', BackgroundFlag::None);
    }
}

impl TransformComponent {
    pub fn new(x: i32, y: i32) -> Self {
        TransformComponent {
            x,
            y,
        }
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}
