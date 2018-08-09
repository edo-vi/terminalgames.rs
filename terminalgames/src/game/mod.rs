pub mod board;
use self::board::{Board, Tile};
use std::sync::{RwLock};

struct Game {
    board: Board<RwLock<Vec<Tile>>>,
}

