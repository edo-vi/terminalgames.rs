extern crate pancurses;

pub mod renderer;
pub mod input;

use std::sync::{Weak, RwLock};
use self::renderer::{Renderer};
use self::input::{Input};
use self::pancurses::{initscr, Window};
use game::board::{Tile, Area, LockedArea};

pub struct Interface {
    _renderer: Option<Renderer>,
    _input: Option<Input>,
    _window: Window,
    _board: Weak<LockedArea>
}


impl Interface {
    pub fn new(board: Weak<LockedArea>) -> Self {
        let win = initscr();
        Interface {_renderer: Option::None, _input: Option::None, _window: win, _board: board}
    }
    ///Creates a new Renderer. Accepts [n]! arguments: 'interval',
    /// which indicates the number of milliseconds that must pass before rendering the screen again,
    /// and 'valid_keys', immutable borrow of a char array that contains the keyboard characters we
    /// consider valid (note that the arrow keys are always valid). It also initializes the target window.
    pub fn new_renderer(&mut self, interval: u32, valid_keys: &[char]) {
        match self._renderer {
            None => self._renderer=Some(Renderer::new(interval, valid_keys)),
            Some(ref mut r) => {
                r.set_interval(interval);
                r.set_keys(valid_keys);
            }
        }

    }
    pub fn test_renderer(&self) {
        match self._renderer {
            None => println!("No Renderer found"),
            Some(ref c) => c.render_border(&self._window)
        }
    }


}