mod object;
mod core;
mod tile;
mod map;
mod player_action;
mod player;
mod components;
mod ai;

fn main() {
    let mut game = core::Game::new();
    game.run();
}
