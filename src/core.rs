extern crate tcod;
extern crate rand;

use super::object::Object;
use super::map::Map;
use super::player_action::PlayerAction;
use super::player::Player;
use super::components::{
                    StatsComponent,
                    AiComponent
                };
use core::tcod::console::*;
use core::tcod::input::Key;
use core::tcod::input::KeyCode::*;
use core::tcod::colors;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;
const PLAYER: usize = 0;

pub struct Game {
    width: i32,
    height: i32,
    fps: i32,
    root: Root,
    console: Box<Console>,
    player: Player,
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
        let player = Player::new(25, 23, StatsComponent::new(30,30,2,5));
        let map = Map::new();

        Game {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            fps: LIMIT_FPS,
            root: root,
            console: console,
            player,
            objects: vec![],
            map: map,
            fov_recompute: true,
        }
    }

    /// Runs the main game loop
    pub fn run(&mut self) {
        let mut prev_player_position = (-1, -1);
        let start = self.map.generate_map(&mut self.objects);
        self.player.set_position(start.0, start.1);

        while !self.root.window_closed() {
            self.root.set_default_background(colors::DARKEST_SEPIA);
            self.root.set_background_flag(BackgroundFlag::Set);

            self.draw_everything();
            self.root.flush();
            self.clear_objects();

            // if the player has moved
            let current_player_position = self.objects[PLAYER].position();
            if prev_player_position != current_player_position {
                // Recalculate fov and update previous position
                self.fov_recompute = true;
                prev_player_position = current_player_position;
            }

            // handle keys and exit game if needed
            let player_action = self.handle_keys();
            if player_action == PlayerAction::Exit {
                break
            }

            if self.player.is_alive() && player_action != PlayerAction::DidntTakeTurn {
                for i in 0..self.objects.len() {
                    if self.objects[i].ai.is_some() {
                        self.objects[i].clone()
                                       .ai
                                       .as_ref()
                                       .unwrap()
                                       .take_turn(i as usize,
                                                  &mut self.map,
                                                  &mut self.objects,
                                                  &mut self.player);
                        println!("Running ai for {} - {}", i, self.objects[i].name)
                    }
                }
            }
        }
    }

    /// Draws the map, player and all objects
    fn draw_everything(&mut self) {
        if self.fov_recompute {
            self.map.calculate_fov(self.player.position(), 10i32);
            self.map.draw(&mut self.console);
        }

        for i in 0..self.objects.len() {
            // Only draw the object if we can see it.
            if self.map.is_in_fov(self.objects[i].position()) {
                self.objects[i].draw(&mut self.console);
            }
        }

        self.player.draw(&mut self.console);

        blit(&mut self.console, (0,0), (SCREEN_WIDTH, SCREEN_HEIGHT), &mut self.root, (0,0), 1.0, 1.0);
    }

    /// Clears the player and all objects
    fn clear_objects(&mut self) {
        for i in 0..self.objects.len() {
            self.objects[i].clear(&mut self.console);
        }

        self.player.clear(&mut self.console);
    }

    /// Handle the key inputs and perform actions
    fn handle_keys(&mut self) -> PlayerAction {
        let key = self.root.wait_for_keypress(true);
        match (key, self.player.is_alive()) {
            (Key { code: Enter, alt: true, .. }, _) => {
                // Alt+Enter: toggle fullscreen
                let fullscreen = self.root.is_fullscreen();
                self.root.set_fullscreen(!fullscreen);
                PlayerAction::DidntTakeTurn
            }
            (Key { code: Escape, .. }, _) => PlayerAction::Exit,  // exit game

            // movement keys
            (Key { code: Up, .. }, true) => {
                self.player.move_or_attack(0, -1, &self.map, &mut self.objects);
                PlayerAction::TookTurn
            },
            (Key { code: Down, .. }, true) => {
                self.player.move_or_attack(0, 1, &self.map, &mut self.objects);
                PlayerAction::TookTurn
            },
            (Key { code: Left, .. }, true) => {
                self.player.move_or_attack(-1, 0, &self.map, &mut self.objects);
                PlayerAction::TookTurn
            },
            (Key { code: Right, .. }, true) => {
                self.player.move_or_attack(1, 0, &self.map, &mut self.objects);
                PlayerAction::TookTurn
            },

            _ => { PlayerAction::DidntTakeTurn },
        }
    }
}
