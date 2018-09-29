mod object;
mod core;
mod tile;
mod map;

fn main() {
    let mut game = core::Game::new();

    game.run();
}
