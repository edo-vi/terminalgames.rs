

extern crate terminalgames;

use terminalgames::interface::{Interface};
use terminalgames::interface::renderer::Renderer;
use terminalgames::interface::input::Input;
use terminalgames::game::{Game};
use terminalgames::game::board::Dimensions;
use terminalgames::game::board::Tile;

fn main() {
    let a: u32 = 200;
    let keys: [char; 6] = ['w','a','s','d', '2', 'e'];
    let mut game = Game::new();
    game.set_board(vec![Tile::New(None);6*8], Dimensions(6,5));

    a.clone();
}

