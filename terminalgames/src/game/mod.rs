pub mod board;
use self::board::{Board, Tile};
use std::sync::{RwLock, LockResult, RwLockWriteGuard, Arc};
use game::board::{Dimensions, BoardError, Area, LockedArea};
use std::thread;
use std::borrow::{BorrowMut, Borrow};
use interface::renderer::Renderer;
use interface::Interface;

pub struct Game<T> {
    _board: Arc<Board<T>>
}

impl Game<LockedArea> {
    pub fn new() -> Self {
        Game {_board: Arc::new(Default::default())}
    }
    pub fn set_board(&mut self, tiles: Area, dimensions: Dimensions) {
        match Board::with_tiles(tiles, dimensions) {
            Ok(b) => self._board=Arc::new(b),
            Err(e) => match e {
                BoardError::WrongLen(mes) => panic!("Couldn't set the board for the game: {}", mes)
            }
        }
    }
    pub fn ref_board(&mut self) -> &Board<LockedArea> {
        self._board.borrow()
    }
    pub fn mut_ref_board(&mut self) -> &mut Board<LockedArea> {
        //todo check this, the problem is that it will fail if we have already shared the weak
        //pointer to the other thread
        Arc::get_mut(&mut self._board).unwrap()
    }

    pub fn begin_rendering(&self, interval: u32, valid_keys: [char;2]) {

        let weak_ptr=Arc::downgrade(&self._board.clone());
        thread::spawn(move || {
            let mut interface = Interface::new(weak_ptr);
            interface.new_renderer(interval, &valid_keys);
            interface.test_renderer();
        });
    }
}