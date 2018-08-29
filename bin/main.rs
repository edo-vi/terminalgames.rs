extern crate terminalgames;
extern crate rand;
extern crate pancurses;
extern crate simplelog;


use terminalgames::game::Game;
use terminalgames::game::board::Tile;
use terminalgames::game::board::Dimensions;
use std::time;
use std::thread;
use terminalgames::interface::input::PlayerInput;
use pancurses::initscr;
use pancurses::endwin;

use simplelog::*;
use std::fs::File;
use simplelog::CombinedLogger;
use terminalgames::game::gamestate::PacManStateManager;
use terminalgames::game::gamestate::PacManOptions;
use terminalgames::game::gamestate::updater::{PacManUpdater};
use terminalgames::game::gamestate::checker::{PacManChecker};

fn main() {

    // set up the logging
    CombinedLogger::init(
        vec![
            //WriteLogger::new(LevelFilter::Info, Config::default(), File::create("trmngames.log")).unwrap(),
            SimpleLogger::new(LevelFilter::Info, Config::default())
        ]
    ).unwrap();

    let a: u32 = 32;
    let keys: [char; 5] = ['w','a','s','d','e'];
    let mut game: Game<PacManStateManager<PacManOptions, PacManUpdater, PacManChecker>, PacManOptions,
        PacManUpdater, PacManChecker> = Game::new();
    game.new_board(vec![Tile::Empty(None); 30*22], Dimensions(30, 22));
    game.add_border();

    game.begin_rendering(a, keys.clone());
    game.begin_game_loop();
    let dur = time::Duration::from_millis(32);
    let mut string = String::new();
    loop {
        game.change_random_tile();
        thread::sleep(dur);
        match game.listen() {
            PlayerInput::Character(_c) => {
                if _c=='e' {break;}
                game.erase_board();
                game.add_border();
                string.push_str(&_c.to_string()[..]);
            },

            _ => ()
        }

    }

    initscr();
    endwin();
    println!("\n\n{}\n\n",string);

}



