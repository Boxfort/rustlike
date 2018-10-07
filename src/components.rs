extern crate tcod;

use components::tcod::console::Console;
use components::tcod::console::BackgroundFlag;
use components::tcod::colors::Color;
use super::object::Object;
use super::map::Map;
use super::player::Player;

#[derive(Clone)]
pub struct RenderComponent {
    pub character: char,
    pub background_color: Option<Color>,
    pub foreground_color: Option<Color>,
}

#[derive(Clone)]
pub struct TransformComponent {
    x: i32,
    y: i32,
}

#[derive(Clone)]
pub struct StatsComponent {
    pub max_hp: i32,
    pub hp: i32,
    pub defence: i32,
    pub power: i32,
}

pub trait AiComponent {
    fn take_turn(&self,
                 object: &mut Object,
                 map: &mut Map,
                 objects: &mut Vec<Object>,
                 player: &mut Player,
                 messages: &mut Vec<(String, Color)>);

    fn box_clone(&self) -> Box<AiComponent>;
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

    /// Draws a character to the specified console
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

    /// Draws a blank character to the specified console
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

    /// Returns the position as a tuple
    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    /// Sets the transform position
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn distance_to(&self, other: (i32, i32)) -> f32 {
        let dx = other.0 - self.x;
        let dy = other.1 - self.y;
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }

    pub fn move_by(&mut self, dx: i32, dy: i32, map: &Map, objects: &Vec<Object>) {
        if !self.is_blocked(self.position().0 + dx,
                            self.position().1 + dy,
                            map,
                            objects) {
            let position = self.position();
            self.set_position(position.0 + dx,
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
}

impl StatsComponent {
    pub fn new(max_hp: i32, hp: i32, defence: i32, power: i32) -> Self {
        StatsComponent {
            max_hp,
            hp,
            defence,
            power,
        }
    }
}
