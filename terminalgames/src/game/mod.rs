pub mod board;
use self::board::{Board, Tile};
use std::sync::{RwLock};

pub struct Game<T> {
    board: Board<T>
}

