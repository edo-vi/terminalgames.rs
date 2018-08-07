
mod lib;
use lib::pac_lib::renderer::{Renderer, PlayerInput};

fn main() {
    println!("Inserisci qualcosa: ");

    let key: PlayerInput= Renderer::get_player_input();

}

