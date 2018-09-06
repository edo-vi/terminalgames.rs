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
use terminalgames::game::gamestate::SnakeOptions;
use terminalgames::game::gamestate::updater::{SnakeUpdater};
use terminalgames::game::gamestate::checker::{SnakeChecker};
use terminalgames::game::board::Board;
use terminalgames::game::gamestate::SnakeStateManager;

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
        .level(log::LevelFilter::Error)
        .chain(fern::log_file("lib.log")?)
        .apply()?;
    Ok(())
}

fn main() {

    // set up the logging
    setup_logger();

    let keys: [char; 5] = ['w','a','s','d','e'];
    let dim = Dimensions(30, 20);
    let board = Board::new(dim.clone());
    let statemanager = SnakeStateManager::new(SnakeOptions::new(dim), SnakeUpdater::default(), SnakeChecker::default());
    let mut game = Game::new(board, statemanager);
    game.add_border();

    game.begin_rendering(32, keys.clone());
    game.begin_game_loop();

    initscr();
    endwin();
    println!("\n Snake 🐍 by Edoardo Zorzi, 06/09/2018\n");

}



