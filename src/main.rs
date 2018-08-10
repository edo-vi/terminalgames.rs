

extern crate terminalgames;

use terminalgames::interface::{Interface};
use terminalgames::interface::renderer::Renderer;
use terminalgames::interface::input::Input;

fn main() {
    let a: u32 = 200;
    let keys: [char; 6] = ['w','a','s','d', '2', 'e'];
    let input: Input;
    let mut interface: Interface = Interface::new();
    interface.new_renderer(a, &keys);
    interface.test_renderer();
}

