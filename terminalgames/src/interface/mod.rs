extern crate pancurses;

pub mod renderer;
pub mod input;

use self::renderer::{Renderer};
use self::input::{Input};
use self::pancurses::{initscr, Window};

pub struct Interface{
    pub renderer: Renderer,
    pub input: Input
}

impl Interface {
    ///Creates a new Renderer. Accepts [n]! arguments: 'interval',
    /// which indicates the number of milliseconds that must pass before rendering the screen again,
    /// and 'valid_keys', immutable borrow of a char array that contains the keyboard characters we
    /// consider valid (note that the arrow keys are always valid). It also initializes the target window.
    pub fn new_renderer(interval: u32, valid_keys: &[char]) -> Renderer {
        Renderer::new(interval, &valid_keys, initscr())
    }

}