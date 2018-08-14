extern crate terminalgames;

use terminalgames::game::Game;
use terminalgames::game::board::Tile;
use terminalgames::game::board::Dimensions;
use std::time;
use std::thread;

fn main() {
    let a: u32 = 200;
    let keys: [char; 2] = ['w','a'];
    let mut game = Game::new();
    game.set_board(vec![Tile::New(None);50*20], Dimensions(50,20));

    game.begin_rendering(a, keys.clone());
    let dur = time::Duration::from_millis(30);
    loop {
        game.change_random_tile();
        thread::sleep(dur);
        game.start_listening();
    }

}


