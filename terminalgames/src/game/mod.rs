pub mod board;
use self::board::{Board, Tile};
use std::sync::{RwLock, LockResult, RwLockWriteGuard};
use game::board::Area;
use game::board::Dimensions;
use game::board::BoardError;

pub struct Game<T> {
    _board: Board<T>
}

impl Game<RwLock<Area>> {
    pub fn new() -> Self {
        Game {_board: Default::default()}
    }
    pub fn set_board(&mut self, tiles: Area, dimensions: Dimensions) {
        match Board::with_tiles(tiles, dimensions) {
            Ok(b) => self._board=b,
            Err(e) => match e {
                BoardError::WrongLen(mes) => panic!("Couldn't set the board for the game: {}", mes)
            }
        }
    }
}