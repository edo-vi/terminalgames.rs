extern crate pancurses;


pub mod renderer;
pub mod input;

use std::sync::{Arc, RwLock};
use self::renderer::{Renderer};
use self::input::{Input, PlayerInput};
use self::pancurses::{initscr, Window, Input as PancursesInput};
use game::board::{Board};
use std::sync::{RwLockReadGuard, LockResult};
use std::ops::Deref;
use std::thread;
use core::time;
use std::sync::mpsc::Sender;

pub struct Interface {
    _renderer: Option<Renderer>,
    _input: Option<Input>,
    _window: Window,
    _board: Arc<RwLock<Board>>
}


impl Interface {
    pub fn new(board: Arc<RwLock<Board>>) -> Self {
        let win = initscr();
        Interface {_renderer: Option::None, _input: Option::None, _window: win, _board: board}
    }
    ///Creates a new Renderer. Accepts [n]! arguments: 'interval',
    /// which indicates the number of milliseconds that must pass before rendering the screen again,
    /// and 'valid_keys', immutable borrow of a char array that contains the keyboard characters we
    /// consider valid (note that the arrow keys are always valid). It also initializes the target window.
    pub fn new_renderer(&mut self, interval: u32) {
        match self._renderer {
            None => self._renderer=Some(Renderer::new(interval)),
            Some(ref mut r) => {
                r.set_interval(interval);
            }
        }
    }
    pub fn new_input(&mut self, valid_keys: &[char], sender: Sender<PlayerInput>) {
        match self._input {
            None => self._input=Some(Input::new(valid_keys, sender)),
            Some(ref mut i) => {
                i.set_keys(valid_keys);
                i.set_sender(sender);
            }
        }
    }

    pub fn renderer(&self) -> &Renderer {
        match self._renderer {
            Some(ref renderer) => renderer,
            None => panic!("No renderer")
        }
    }

    fn board(&self) -> &RwLock<Board> {
        //Transforms the weak pointer in an Arc pointer, and if returns None because it is dropped,
        //it will panic
        self._board.deref()
    }

    pub fn render_loop(&self) {
        let dur = time::Duration::from_millis(30);
        loop {
            { //gets the read lock
                let guard: RwLockReadGuard<Board> = self._get_read_lock();
                //guard.deref returns an immutable borrow to the inner value guarded
                self.renderer().render_board(&self._window, guard.deref());
            } //guard is dropped here

            match self._input {
                Some(ref input) => input.get_player_input(&self._window),
                None => panic!("No Input object to get player input")
            };
            thread::sleep(dur);
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