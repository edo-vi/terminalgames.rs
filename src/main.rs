

extern crate terminalgames;

use terminalgames::interface::{Interface};
use terminalgames::interface::renderer::Renderer;
use terminalgames::interface::input::Input;
use terminalgames::game::{Game};
use terminalgames::game::board::{Board, LockedArea, Tile, Dimensions};

fn main() {
    let a: u32 = 200;
    let keys: [char; 2] = ['w','a'];
    let mut game = Game::new();
    game.set_board(vec![Tile::New(None);6*5], Dimensions(6,5));

    let board: &mut Board<LockedArea> = game.mut_ref_board();
    board.set_border();

    
    game.begin_rendering(a, keys.clone());

    loop {
        let b = a;

    }
}

