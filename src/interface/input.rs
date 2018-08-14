use super::pancurses::{Input as PancursesInput, noecho};

use super::pancurses::Window;

#[derive(PartialEq)]
pub enum PlayerInput {
    Arrow(PancursesInput),
    Character(char),
    Invalid
}

pub struct Input {
    _valid_keys: Vec<char>
}

impl Input {
    pub fn new(valid_keys: &[char]) -> Self {
        Input {
            _valid_keys: Input::vectorize(valid_keys)
        }
    }

    pub fn set_keys(&mut self, valid_keys: &[char]) {
        self._valid_keys = Input::vectorize(valid_keys);
    }

    pub fn get_player_input(&self, window: &Window) -> PlayerInput {
        window.keypad(true);
        window.nodelay(true);
        noecho();

        let key = window.getch();
        window.refresh();

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
}