
extern crate pancurses;

pub mod renderer {

    use super::pancurses::{initscr, endwin, Window, Input as PancursesInput};
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
        ///Associated function used to initialize a Renderer. Accepts [n]! arguments: interval,
        /// which indicates the number of milliseconds that must pass before rendering the screen again,
        /// and valid_keys, immutable borrow of a char array that contains the keyboard characters we
        /// consider valid (note that the arrow keys are always valid).
        pub fn create(interval: u32, valid_keys: &[char]) -> Self {
            let mut vec: Vec<char> = Vec::new();
            for &a in valid_keys {
                vec.push(a);
            }
            // shadows mutable binding to immutable
            let vec = vec;

            let _win = initscr();

            Renderer {
                _interval: interval,
                _valid_keys: vec,
                _window: _win
            }
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
            let dur = time::Duration::from_millis(100);
            let _subwin = self._window.subwin(10,15,5,10);
            let _suw : Window;
            self._window.setscrreg(1,20);
            match _subwin {
                Err(e) => {
                    endwin();
                    panic!("Errore nel creare subwindow: {}", e)
                },
                Ok(t) => {_suw = t;}
            }
            for i in 1..1000 {

                self._window.printw("HELLO");
                thread::sleep(dur);
                self._window.refresh();
                _suw.border('|','|','_', ' ', ' ', ' ', ' ', ' ');
                _suw.touchline(0,20);
                _suw.refresh();
            }
            endwin();
        }
    }
}



