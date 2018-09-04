extern crate terminalgames;
extern crate rand;
extern crate pancurses;


#[macro_use] extern crate log;
extern crate fern;

extern crate chrono;

use terminalgames::game::Game;
use terminalgames::game::board::Tile;
use terminalgames::game::board::Dimensions;
use std::time;
use std::thread;
use terminalgames::interface::input::PlayerInput;
use pancurses::initscr;
use pancurses::endwin;


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

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file("trmngames.log")?)
        .apply()?;
    Ok(())
}

fn main() {

    // set up the logging
    setup_logger();

    let keys: [char; 5] = ['w','a','s','d','e'];
    let dim = Dimensions(32, 22);
    let board = Board::new(dim.clone());
    let statemanager = PacManStateManager::new(PacManOptions::new(dim), PacManUpdater::default(), PacManChecker::default());
    let mut game = Game::new(board, statemanager);
    game.add_border();

    game.begin_rendering(32, keys.clone());
    game.begin_game_loop();

    initscr();
    endwin();

}



