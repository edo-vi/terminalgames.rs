
use super::pancurses::{endwin, Window, half_delay, flushinp, Input as PancursesInput};
use std::{thread, time};

pub enum PlayerInput {
    Arrow(PancursesInput),
    Character(char),
    Invalid
}

pub struct Renderer {
    _interval: u32,
    _valid_keys: Vec<char>,
}

//TODO change number of arguments
impl Renderer {
    pub (in interface) fn new(interval: u32, valid_keys: &[char]) -> Self {
        Renderer {
            _interval: interval,
            _valid_keys: Renderer::vectorize(valid_keys),
        }
    }

    pub fn interval(&self) -> u32 {
        self._interval
    }

    pub fn set_interval(&mut self, interval: u32) {
        self._interval=interval;
    }

    pub fn set_keys(&mut self, valid_keys: &[char]) {
        self._valid_keys = Renderer::vectorize(valid_keys);
    }

    pub fn get_player_input(&self, window: &Window) -> PlayerInput {
        window.keypad(true);
        window.refresh();
        half_delay(3);
        let key = window.getch();


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

    fn vectorize(keys: &[char]) -> Vec<char> {
        keys.iter().map(|a| *a).collect()
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
    ///Random tests of pancurses library
    pub fn render_border(&self, window: &Window) {
        let dur = time::Duration::from_millis(self._interval as u64);
        let _subwin = window.subwin(10,10,5,10);
        window.setscrreg(1,14);
        window.scrollok(true);
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
}