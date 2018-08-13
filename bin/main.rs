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
    game.set_board(vec![Tile::New(None);6*5], Dimensions(6,5));

    let handle= game.begin_rendering(a, keys.clone());
    let dur = time::Duration::from_millis(10);
    loop {
        game.change_random_tile();
        thread::sleep(dur)
    }

}


