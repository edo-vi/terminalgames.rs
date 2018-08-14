pub mod board;
extern crate pancurses;
extern crate rand;

use std::sync::{RwLock, LockResult, RwLockWriteGuard, RwLockReadGuard, Arc};
use game::board::{Board, Tile, Dimensions, BoardError, Area};
use std::ops::Deref;
use std::ops::DerefMut;
use interface::{Interface};
use std::thread;
use std::time;
use self::rand::Rng;
use std::sync::mpsc::{channel, Receiver};
use std::thread::JoinHandle;
use self::pancurses::{Input as PancursesInput};
use interface::input::PlayerInput;

pub struct Game {
    _board: Arc<RwLock<Board>>,
    _receiver: Option<Receiver<PlayerInput>>
}

impl Game {
    pub fn new() -> Self {
        Game {_board: Arc::new(RwLock::new(Default::default())), _receiver: Option::None}
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

    pub fn begin_rendering(&mut self, interval: u32, valid_keys: [char;2]) -> JoinHandle<()> {
        let pointer=Arc::clone(&self._board);
        let (sender,receiver) = channel();
        self._receiver = Some(receiver);

        thread::spawn(move || {
            let mut interface = Interface::new(pointer);
            interface.new_renderer(interval);
            interface.new_input(&valid_keys, sender);
            interface.render_loop();
        })

    }

    pub fn start_listening(&self) {
        match self._receiver {
            Some(ref receiver) => receiver.try_recv(),
            None => panic!("!!")
        };
    }
    pub fn change_random_tile(&self) {
        let mut guard = self._get_write_lock();
        let board: &mut Board = guard.deref_mut();


        let mut rng = rand::thread_rng();
        let Dimensions(x,y) = *board.dimensions();

        board.set_tile(rng.gen::<usize>()%(x*y)as usize,Tile::Border(None));
        board.set_tile(rng.gen::<usize>()%(x*y)as usize,Tile::New(None));
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