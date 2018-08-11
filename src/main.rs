

extern crate terminalgames;

use terminalgames::interface::{Interface};
use terminalgames::interface::renderer::Renderer;
use terminalgames::interface::input::Input;
use terminalgames::game::board::*;
fn main() {
    let a: u32 = 200;
    let keys: [char; 6] = ['w','a','s','d', '2', 'e'];
    let input: Input;
    let mut interface: Interface = Interface::new();
    interface.new_renderer(a, &keys);
    let mut board = Board::new(Dimensions (6,5));
    board.swap_tiles(vec![Tile::NonBlocking(Some('e')); 6*5]);
    board.set_border();
    let t = 5;
}

