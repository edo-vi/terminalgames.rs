extern crate terminalgames;
extern crate rand;
extern crate pancurses;
#[macro_use] extern crate log;
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
use terminalgames::game::gamestate::StateManager;
use terminalgames::game::gamestate::updater::Update;
use terminalgames::game::gamestate::checker::Check;
use terminalgames::game::gamestate::GameOptions;
use terminalgames::game::gamestate::PacManOptions;
use terminalgames::game::gamestate::updater::{PacManUpdater};
use terminalgames::game::gamestate::checker::{PacManChecker};
use terminalgames::game::board::Board;
use terminalgames::game::gamestate::PacManStateManager;

fn main() {

    // set up the logging
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default()).unwrap(),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();

    let keys: [char; 5] = ['w','a','s','d','e'];
    let dim = Dimensions(32, 22);
    let board = Board::new(dim.clone());
    let statemanager = PacManStateManager::new(PacManOptions::new(dim), PacManUpdater::default(), PacManChecker::default());
    let mut game = Game::new(board, statemanager);
    game.add_border();

    game.begin_rendering(32, keys.clone());
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



