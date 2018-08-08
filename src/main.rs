
mod lib;
use lib::pac_lib::renderer::{Renderer, PlayerInput};

fn main() {
    println!("Inserisci qualcosa: ");
    let a: u32 = 20000;
    let render: Renderer = Renderer::create(a,&['w','a','s','d', '2', 'e']);
    println!("{}",a);
}

