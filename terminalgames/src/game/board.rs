use std::sync::{Mutex};

enum Tile{
    Border(Option<char>),
    Active(Option<char>),
    Blocking(Option<char>),
    NonBlocking(Option<char>)
}

struct Dimensions(u8,u8);

pub struct Board {
    tiles: Vec<Tile>,
    dimensions: Dimensions
}
pub struct ShareableBoard {
    tiles: Mutex<Vec<Tile>>,
    dimensions: Dimensions
}
