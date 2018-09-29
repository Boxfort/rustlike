mod object;
mod core;
mod tile;
mod map;

extern crate tcod;

use tcod::console::Console;

pub trait Drawable {
    fn draw(&self, &mut Console);
    fn clear(&self, &mut Console);
}
