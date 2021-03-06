

use super::pancurses::{Window, ACS_BSBS};
use super::Board;
use game::board::Dimensions;
use game::board::Tile;

pub struct Renderer {
    _interval: u32,
}


impl Renderer {
    pub (in interface) fn new(interval: u32) -> Self {
        Renderer {
            _interval: interval,
        }
    }

    pub fn interval(&self) -> u32 {
        self._interval
    }

    pub fn set_interval(&mut self, interval: u32) {
        self._interval=interval;
    }

    pub fn render_board(&self, window: &Window, board: &Board) {
        let Dimensions(x,y): Dimensions = *board.dimensions();
        window.erase();

        for j in 0..y  {
            window.mv(j as i32, 0);
            for i in 0..x {

                match *board.get_tile(i as usize +j as usize *x as usize) {
                    Tile::Empty(v) => {
                        match v{
                            None => window.addch(' '),
                            Some(c) => window.addch(c)
                        }
                    },
                    Tile::VBorder(v) => {
                        match v{
                            None => window.addch('*'),
                            Some(c) => window.addch(c)
                        }
                    },
                    Tile::HBorder(v) => {
                        match v{
                            None => window.addch('*'),
                            Some(c) => window.addch(c)
                        }
                    },
                    Tile::Active(v) => {
                        match v{
                            None => window.addch('^'),
                            Some(c) => window.addch(c)
                        }
                    },
                    _ => window.addch(' ')
                };
            }
             //have no idea why it works, I changed it randomly

        }
        window.refresh();

    }
}
