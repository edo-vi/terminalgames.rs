
use super::pancurses::{initscr, endwin, Window, half_delay, echo, flushinp, Input as PancursesInput};
use std::{thread, time};

pub enum PlayerInput {
    Arrow(PancursesInput),
    Character(char),
    Invalid
}

pub struct Renderer {
    _interval: u32,
    _valid_keys: Vec<char>,
    _window: Window
}

//TODO change number of arguments
impl Renderer {
    ///Create a new Renderer. Accepts [n]! arguments: interval,
    /// which indicates the number of milliseconds that must pass before rendering the screen again,
    /// and valid_keys, immutable borrow of a char array that contains the keyboard characters we
    /// consider valid (note that the arrow keys are always valid).
    pub (in interface) fn new(interval: u32, valid_keys: &[char], window: Window) -> Self {
        let mut vec: Vec<char> = Vec::new();
        for &a in valid_keys {
            vec.push(a);
        }
        // shadows mutable binding to immutable
        let vec = vec;

        Renderer {
            _interval: interval,
            _valid_keys: vec,
            _window: window
        }
    }

    pub fn interval(&self) -> u32 {
        self._interval
    }

    pub fn get_player_input(&self) -> PlayerInput {
        let window = &self._window;
        window.keypad(true);
        window.refresh();
        let key = window.getch();
        endwin();

        match key {
            None => PlayerInput::Invalid,
            Some(v) => {
                match v {
                    PancursesInput::KeyLeft|PancursesInput::KeyRight|PancursesInput::KeyDown|PancursesInput::KeyUp => PlayerInput::Arrow(v),
                    PancursesInput::Character(c) => {
                        match self._is_key_valid(c) {
                            true => PlayerInput::Character(c),
                            false => PlayerInput::Invalid
                        }
                    }
                    _ => PlayerInput::Invalid
                }
            }
        }
    }

    pub fn set_interval(&mut self, interval: u32) {
        self._interval=interval;
    }

    pub fn destroy(self) {
        println!("Destroying Renderer");
    }

    fn _is_key_valid(&self, key: char) -> bool {
        if self._valid_keys.contains(&key) {
            true
        } else {
            false
        }
    }

    pub fn populate_valid_keys(&mut self, keys: &[char]) {
        for a in keys.iter() {
            self._valid_keys.push(*a);
        }
    }
    ///test per vedere se funziona in cargo doc
    pub fn render_border(&self) {
        let dur = time::Duration::from_millis(200);
        let dur2 = time::Duration::from_millis(1000);
        let _subwin = self._window.subwin(10,10,5,10);
        self._window.setscrreg(1,14);
        self._window.scrollok(true);
        let _suw : Window;
        let _input_window: Window = self._window.subwin(3,15,15,2).unwrap();

        match _subwin {
            Err(e) => {
                endwin();
                panic!("Errore nel creare subwindow: {}", e)
            },
            Ok(t) => {_suw = t;}
        }
        for i in 1..1000 {

            self._window.printw("HELLO");
            self._window.refresh();

            _input_window.refresh();

            half_delay(5);
            let key = self._window.getch();

            self._window.refresh();
            match key {
                Some(c) => {
                    match c {
                        PancursesInput::Character(c) => {
                            self._window.erase();
                            _suw.border('*','*','*','*',' ',' ',' ',' ');
                            //_suw.touchline(0,20);
                            _suw.refresh();
                            self._window.printw("AAAQQQQQQQQQQQQQQA");
                            self._window.refresh();
                            thread::sleep(dur2);

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
            self._window.refresh();
            thread::sleep(dur);
        }
        endwin();
    }
}