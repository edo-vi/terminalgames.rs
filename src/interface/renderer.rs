
use super::pancurses::{endwin, Window, half_delay, flushinp, Input as PancursesInput};
use super::Board;
use std::{thread, time};
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

    ///Random tests of pancurses library
    pub fn render_border(&self, window: &Window) {
        let dur = time::Duration::from_millis(self._interval as u64);
        let _subwin = window.subwin(10,10,5,10);

        let _suw : Window;
        let _input_window: Window = window.subwin(3,15,15,2).unwrap();

        match _subwin {
            Err(e) => {
                endwin();
                panic!("Errore nel creare subwindow: {}", e)
            },
            Ok(t) => {_suw = t;}
        }
        for _ in 1..1000 {

            window.printw("HELLO");
            window.refresh();

            _input_window.refresh();

            half_delay(1);
            let key = window.getch();

            window.refresh();
            match key {
                Some(c) => {
                    match c {
                        PancursesInput::Character(c) => {
                            window.erase();
                            _suw.border('*','*','*','*',' ',' ',' ',' ');
                            //_suw.touchline(0,20);
                            _suw.refresh();
                            window.printw(c.to_string().repeat(20));
                            window.refresh();
                            thread::sleep(dur);

                            flushinp();
                        }
                        _ => ()
                    }
                }
                _ => ()
            }
            _suw.border('*','*','*','*',' ',' ',' ',' ');
            //_suw.touchline(0,20);
            _suw.refresh();
            window.refresh();
            thread::sleep(dur);
        }
        endwin();
    }

    pub fn render_board(&self, window: &Window, board: &Board) {
        let Dimensions(x,y): Dimensions = *board.dimensions();
        window.erase();
        for i in 0..y  {
            for j in 0..x {
                match *board.get_tile(j as usize +i as usize *x as usize) {
                    Tile::Empty(_c) => window.addch(' '),
                    Tile::Border(_c) => window.addch('*'),
                    _ => window.addch(' ')
                };
            }
            window.mv(i as i32 +1,0);

        }
        window.refresh();

        //endwin(); //todo change, only for debugging
    }
}