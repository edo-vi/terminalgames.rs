
extern crate pancurses;

pub mod player {

}

pub mod renderer {

    use super::pancurses::{initscr, endwin, Input as PanInput};

    pub enum PlayerInput {
        Arrow(PanInput),
        Valid(char),
        Invalid
    }

    pub struct Renderer {
        _interval: u32
    }

    impl Renderer {
        pub fn get_player_input() -> PlayerInput {
            let window = initscr();
            window.keypad(true);
            window.refresh();
            let key = window.getch();
            endwin();

            match key {
                None => PlayerInput::Invalid,
                Some(v) => {
                    match v {
                        PanInput::KeyLeft|PanInput::KeyRight|PanInput::KeyDown|PanInput::KeyUp => PlayerInput::Arrow(v),
                        PanInput::Character(c) => {
                            // valid playerinputs
                            match c {
                                'w'|'a'|'s'|'d' => PlayerInput::Valid(c),
                                _ => PlayerInput::Invalid
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
        fn _is_key_valid(key: PanInput) -> bool {
            if key==PanInput::Character('w') || key==PanInput::Character('a')
                || key==PanInput::Character('s') || key==PanInput::Character('d') {
                true
            } else {
                false
            }
        }
    }
}



