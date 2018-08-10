extern crate pancurses;

pub mod renderer;
pub mod input;

use std::rc::Rc;
use std::mem::replace as mem_replace;
use self::renderer::{Renderer};
use self::input::{Input};
use self::pancurses::{initscr, Window};

pub struct Interface {
    _renderer: Option<Renderer>,
    _input: Option<Input>,
    _window: Rc<Window>,
}


impl Interface {
    pub fn new() -> Self {
        let win = Rc::new(initscr());
        Interface {_renderer: Option::None, _input: Option::None, _window: win}
    }
    ///Creates a new Renderer. Accepts [n]! arguments: 'interval',
    /// which indicates the number of milliseconds that must pass before rendering the screen again,
    /// and 'valid_keys', immutable borrow of a char array that contains the keyboard characters we
    /// consider valid (note that the arrow keys are always valid). It also initializes the target window.
    pub fn new_renderer(&mut self, interval: u32, valid_keys: &[char]) {
        match self._renderer {
            None => self._renderer=Some(Renderer::new(interval, valid_keys, Rc::clone(&self._window))),
            Some(ref mut R) => {
                R.set_interval(interval);
                R.set_valid_keys(valid_keys);
            }
        }

    }
    pub fn test_renderer(&self) {
        match self._renderer {
            None => println!("No Renderer found"),
            Some(ref C) => C.render_border()
        }
    }

}