
mod lib;
use lib::pac_lib::renderer::{Renderer, PlayerInput};

fn main() {
    let a: u32 = 20000;
    let keys: [char; 6] = ['w','a','s','d', '2', 'e'];
    let render: Renderer = Renderer::create(a,&keys);
    render.render_border();
}

