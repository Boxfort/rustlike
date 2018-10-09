extern crate tcod;
extern crate rand;

use super::gui::Gui;
use super::object::Object;
use super::map::Map;
use super::item::Item;
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
use core::tcod::Color;

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
    gui: Gui,
    messages: Vec<(String, Color)>,
    player: Player,
    objects: Vec<Object>,
    objects_next: Vec<Object>,
    items: Vec<Item>,
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
        let gui = Gui::new(SCREEN_WIDTH, SCREEN_HEIGHT, &player);
        let messages: Vec<(String, Color)> = vec![("Welcome to rustlike!".to_string(), colors::WHITE)];

        Game {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            fps: LIMIT_FPS,
            root,
            console,
            gui,
            messages,
            player,
            objects: vec![],
            objects_next: vec![],
            items: vec![],
            map: map,
            fov_recompute: true,
        }
    }

    /// Runs the main game loop
    pub fn run(&mut self) {
        let mut prev_player_position = (-1, -1);
        let start = self.map.generate_map(&mut self.objects, &mut self.items);

        // Copy the state of the objects to the next
        self.objects_next.clone_from(&self.objects);

        self.player.set_position(start.0, start.1);

        while !self.root.window_closed() {
            self.root.set_default_background(colors::DARKEST_SEPIA);
            self.root.set_background_flag(BackgroundFlag::Set);

            self.gui.update(&self.player, &self.messages);
            self.draw_everything();
            self.root.flush();
            self.clear_everything();

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
                for (i, object) in self.objects.iter_mut().enumerate() {
                    object.take_turn(&mut self.map,
                                     &mut self.objects_next,
                                     &mut self.player,
                                     &mut self.messages);

                    // Copy the updated object into the next state.
                    self.objects_next[i].clone_from(object);
                    println!("Running ai for {} - {}", i, object.name)
                }

                // Copy back the changed states.
                self.objects.clone_from(&self.objects_next);
            }
        }
    }

    /// Draws the map, player and all objects
    fn draw_everything(&mut self) {
        if self.fov_recompute {
            self.map.calculate_fov(self.player.position(), 10i32);
            self.map.draw(&mut self.console);
        }

        for item in &self.items {
            if self.map.is_in_fov(item.position()) {
                item.draw(&mut self.console);
            }
        }

        // Collect the objects which can be seen
        let mut to_draw: Vec<_> = self.objects.iter()
                                                   .filter(|o| self.map.is_in_fov(o.position()))
                                                   .collect();

        // Sort the objects so that blocking objects are drawn last
        to_draw.sort_by(|o1, o2| { o1.blocking.cmp(&o2.blocking) });

        for object in &to_draw {
            object.draw(&mut self.console);
        }

        self.player.draw(&mut self.console);

        blit(&mut self.console, (0,0), (SCREEN_WIDTH, SCREEN_HEIGHT), &mut self.root, (0,0), 1.0, 1.0);
        // Draw GUI
        self.gui.draw(&mut self.root);
    }

    /// Clears the player and all objects
    fn clear_everything(&mut self) {
        for i in 0..self.objects.len() {
            self.objects[i].clear(&mut self.console);
        }

        for i in 0..self.items.len() {
            self.items[i].clear(&mut self.console);
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
                self.player.move_or_attack(0, -1, &self.map, &mut self.objects, &mut self.messages);
                PlayerAction::TookTurn
            },
            (Key { code: Down, .. }, true) => {
                self.player.move_or_attack(0, 1, &self.map, &mut self.objects, &mut self.messages);
                PlayerAction::TookTurn
            },
            (Key { code: Left, .. }, true) => {
                self.player.move_or_attack(-1, 0, &self.map, &mut self.objects, &mut self.messages);
                PlayerAction::TookTurn
            },
            (Key { code: Right, .. }, true) => {
                self.player.move_or_attack(1, 0, &self.map, &mut self.objects, &mut self.messages);
                PlayerAction::TookTurn
            },
            (Key { printable: 'g', .. }, true) => {
            // pick up an item
            let item_id = self.items.iter().position(|object| {
                object.position() == self.player.position()
            });
            if let Some(item_id) = item_id {
                if self.player.pick_item_up(self.items[item_id].clone(), &mut self.messages) {
                    self.items.remove(item_id);
                }
            }
            PlayerAction::DidntTakeTurn
        }

            _ => { PlayerAction::DidntTakeTurn },
        }
    }
}
