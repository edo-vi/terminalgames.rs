
use super::pancurses::{Window};
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

        for i in 0..y  {
            for j in 0..x {
                match *board.get_tile(j as usize +i as usize *x as usize) {
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
                    _ => window.addch(' ')
                };
            }
            window.mv(i as i32 +1,0);

        }
        window.refresh();

    }
}