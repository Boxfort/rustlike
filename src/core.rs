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
    fov_recompute: bool,
}

impl Game {
    pub fn new() -> Self {
        let root = Root::initializer()
            .font("arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("Roguelike")
            .init();

        let console = Box::new(Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT));
        let player = Object::new(25, 23, '@', colors::WHITE);
        let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', colors::YELLOW);
        let objects = vec![player, npc];
        let map = Map::new();

        Game {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            fps: LIMIT_FPS,
            root: root,
            console: console,
            objects: objects,
            map: map,
            fov_recompute: true,
        }
    }

    pub fn run(&mut self) {
        let mut prev_player_position = (-1, -1);
        let start = self.map.generate_map();
        self.objects[0].x = start.0;
        self.objects[0].y = start.1;

        while !self.root.window_closed() {

            self.draw_everything();
            self.root.flush();
            self.clear_objects();

            // if the player has moved
            let current_player_position = (self.objects[0].x, self.objects[0].y);
            if prev_player_position != current_player_position {
                // Recalculate fov and update previous position
                self.fov_recompute = true;
                prev_player_position = current_player_position;
            }

            // handle keys and exit game if needed
            let exit = self.handle_keys();
            if exit {
                break
            }
        }
    }

    fn draw_everything(&mut self) {
        if self.fov_recompute {
            self.map.calculate_fov(self.objects[0].x, self.objects[0].y, 10i32);
            self.map.draw(&mut self.console);
        }

        for i in 0..self.objects.len() {
            // Only draw the object if we can see it.
            if self.map.is_in_fov(self.objects[i].x, self.objects[i].y) {
                self.objects[i].draw(&mut self.console);
            }
        }

        blit(&mut self.console, (0,0), (SCREEN_WIDTH, SCREEN_HEIGHT), &mut self.root, (0,0), 1.0, 1.0);
    }

    fn clear_objects(&mut self) {
        for i in 0..self.objects.len() {
            self.objects[i].clear(&mut self.console);
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
