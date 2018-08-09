

extern crate terminalgames;

use terminalgames::interface::renderer::{Renderer};
fn main() {
    let a: u32 = 20000;
    let keys: [char; 6] = ['w','a','s','d', '2', 'e'];
    let render: Renderer = Renderer::new(a, &keys);
    render.render_border();

}

