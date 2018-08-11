pub mod board;
use self::board::{Board, Tile};
use std::sync::{RwLock, LockResult, RwLockWriteGuard};

pub struct Game<T> {
    board: Board<T>
}

impl Game<RwLock<Vec<Tile>>> {
    pub fn new() -> Self {
        Game {board: Default::default()}
    }
    pub fn set_board()
}