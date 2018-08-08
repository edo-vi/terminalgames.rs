
extern crate pancurses;

pub mod renderer {

    use super::pancurses::{initscr, endwin, Input as PancursesInput};

    pub enum PlayerInput {
        Arrow(PancursesInput),
        Character(char),
        Invalid
    }

    pub struct Renderer {
        _interval: u32,
        _valid_keys: Vec<char>,
    }


    impl Renderer {
        pub fn create(interval: u32, valid_keys: &[char]) -> Self {
            let vec: Vec<char> = valid_keys.iter().map(|a| {*a}).collect();
            Renderer {
                _interval: interval,
                _valid_keys: vec
            }
        }
        pub fn get_player_input(&self) -> PlayerInput {
            let window = initscr();
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
            // valid keyboard inputs
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
    }
}



