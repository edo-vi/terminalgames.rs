extern crate terminalgames;

use terminalgames::game::Game;
use terminalgames::game::board::Tile;
use terminalgames::game::board::Dimensions;
use std::time;
use std::thread;
use terminalgames::interface::input::PlayerInput;

fn main() {
    let a: u32 = 32;
    let keys: [char; 4] = ['w','a','s','d'];
    let mut game = Game::new();
    game.set_board(vec![Tile::Empty(None); 50*20], Dimensions(50, 20));

    game.begin_rendering(a, keys.clone());
    let dur = time::Duration::from_millis(32);
    loop {
        game.change_random_tile();
        thread::sleep(dur);
        match game.listen() {
            PlayerInput::Character(c) => game.erase_board(),
            _ => ()
        }

    }

}


