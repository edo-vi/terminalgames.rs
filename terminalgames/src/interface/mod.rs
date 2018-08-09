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
    ///Creates a new
    pub fn new_renderer(interval: u32, valid_keys: &[char]) -> Renderer {
        Renderer::new(interval, &valid_keys, initscr())
    }

}