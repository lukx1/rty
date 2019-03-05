#![feature(duration_float)]
#![feature(nll)]

extern crate winapi;
extern crate multiinput;

pub mod game;
pub mod keyboard;
pub mod keys;
pub mod game_map;
pub mod sound;

pub use keyboard::*;
pub use game::*;
pub use keys::*;
pub use game_map::*;
pub use sound::*;


fn main() {
    let mut game = Game::new();
    game.play(game_map::GameMap::from_file("E:\\Projects\\Programming\\Workspace\\Rust\\rty\\res\\senbonzakura.rty"));
    //sound::beep_map(&);
}
