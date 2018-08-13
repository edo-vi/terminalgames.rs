pub mod board;

use std::sync::{RwLock, LockResult, RwLockWriteGuard, RwLockReadGuard, Arc};
use game::board::{Board, Tile, Dimensions, BoardError, Area, LockedArea};
use std::thread;
use std::ops::Deref;
use interface::{Interface, renderer::{Renderer}};
use std::time::Duration;
use std::thread::JoinHandle;

pub struct Game {
    _board: Arc<RwLock<Board>>
}

impl Game {
    pub fn new() -> Self {
        Game {_board: Arc::new(RwLock::new(Default::default()))}
    }

    pub fn set_board(&mut self, tiles: Area, dimensions: Dimensions) {
        match Board::with_tiles(tiles, dimensions) {
            Ok(b) => self._board=Arc::new(RwLock::new(b)),
            Err(e) => match e {
                BoardError::WrongLen(mes) => panic!("Couldn't set the board for the game: {}", mes)
            }
        }
    }

    pub fn board(&self) -> &RwLock<Board> {
        self._board.deref()
    }

    pub fn begin_rendering(&self, interval: u32, valid_keys: [char;2]) -> JoinHandle<()> {
        let pointer=Arc::clone(&self._board);
        thread::spawn(move || {
            let mut interface = Interface::new(pointer);
            interface.new_renderer(interval, &valid_keys);
            interface.render_loop();
        })

    }

    fn _get_write_lock(&self) -> RwLockWriteGuard<Board> {
        // Attempt to get the lock over the board tiles
        let result: LockResult<RwLockWriteGuard<Board>> = self.board().write();

        match result {
            //We got the non-poisoned lock, so we return it alongside
            Ok(guard) => guard,
            Err(_) => panic!("The lock over the boar tiles was poisoned!")
        }

    }

    fn _get_read_lock(&self) -> RwLockReadGuard<Board> {
        // Attempt to get the lock over the board tiles
        let result: LockResult<RwLockReadGuard<Board>> = self.board().read();

        match result {
            //We got the non-poisoned lock, so we return it alongside
            Ok(guard) => guard,
            Err(_) => panic!("The lock over the boar tiles was poisoned!")
        }

    }
}