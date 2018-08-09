

extern crate terminalgames;

use terminalgames::interface::{Interface};
use terminalgames::interface::renderer::*;

fn main() {
    let a: u32 = 20000;
    let keys: [char; 6] = ['w','a','s','d', '2', 'e'];
    let render: Renderer = Interface::new_renderer(a,&keys);
    render.render_border();
}

