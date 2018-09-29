extern crate tcod;

use super::object::Object;
use super::map::Map;
use core::tcod::console::*;
use core::tcod::input::Key;
use core::tcod::input::KeyCode::*;
use core::tcod::colors;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

pub struct Game {
    width: i32,
    height: i32,
    fps: i32,
    root: Root,
    console: Box<Console>,
    objects: Vec<Object>,
    map: Map,
}

impl Game {
    pub fn new() -> Self {
        let mut root = Root::initializer()
            .font("arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("Roguelike")
            .init();

        let console = Box::new(Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT));
        let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', colors::WHITE);
        let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', colors::YELLOW);
        let mut objects = vec![player, npc];
        let mut map = Map::new();

        Game {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            fps: LIMIT_FPS,
            root: root,
            console: console,
            objects: objects,
            map: map,
        }
    }

    pub fn run(&mut self) {
        while !self.root.window_closed() {

            self.draw_everything();

            self.root.flush();

            self.clear_objects();

            // handle keys and exit game if needed
            let exit = self.handle_keys();
            if exit {
                break
            }
        }
    }

    fn draw_everything(&mut self) {
        self.console.set_default_foreground(colors::WHITE);

        for i in 0..self.objects.len() {
            self.objects[i].draw(&mut self.console);
        }

        self.map.draw(&mut self.console);

        blit(&mut self.console, (0,0), (SCREEN_WIDTH, SCREEN_HEIGHT), &mut self.root, (0,0), 1.0, 1.0);
    }

    fn clear_objects(&mut self) {
        for i in 0..self.objects.len() {
            self.objects[0].clear(&mut self.console);
        }
    }

    fn handle_keys(&mut self) -> bool {
        let player = &mut self.objects[0];
        let key = self.root.wait_for_keypress(true);
        match key {
            Key { code: Enter, alt: true, .. } => {
                // Alt+Enter: toggle fullscreen
                let fullscreen = self.root.is_fullscreen();
                self.root.set_fullscreen(!fullscreen);
            }
            Key { code: Escape, .. } => return true,  // exit game

            // movement keys
            Key { code: Up, .. } => player.move_by(0, -1, &self.map),
            Key { code: Down, .. } => player.move_by(0, 1, &self.map),
            Key { code: Left, .. } => player.move_by(-1, 0, &self.map),
            Key { code: Right, .. } => player.move_by(1, 0, &self.map),

            _ => {},
        }

        false
    }
}
