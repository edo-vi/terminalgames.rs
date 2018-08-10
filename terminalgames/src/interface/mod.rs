extern crate pancurses;

pub mod renderer;
pub mod input;

use std::rc::Rc;
use std::mem::replace;
use self::renderer::{Renderer};
use self::input::{Input};
use self::pancurses::{initscr, Window};

pub struct Interface  {
    pub renderer: Renderer,
    pub input: Input,
    _window: Rc<Window>,
}

impl Interface {
    pub fn new(renderer: Renderer, input: Input) -> Self {
        let win = Rc::new(initscr());
        let mut renderer = renderer;
        renderer.set_window(Rc::clone(&win));

        Interface {renderer, input, _window: win}
    }
    ///Creates a new Renderer. Accepts [n]! arguments: 'interval',
    /// which indicates the number of milliseconds that must pass before rendering the screen again,
    /// and 'valid_keys', immutable borrow of a char array that contains the keyboard characters we
    /// consider valid (note that the arrow keys are always valid). It also initializes the target window.
    pub fn new_renderer(&mut self, interval: u32, valid_keys: &[char]) {
        replace(&mut self.renderer,Renderer::new(interval, &valid_keys, Rc::clone(&self._window)));
    }

}