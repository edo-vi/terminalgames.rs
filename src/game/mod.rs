pub mod board;
pub mod gamestate;

extern crate pancurses;
extern crate rand;

use std::sync::{RwLock, LockResult, RwLockWriteGuard, RwLockReadGuard, Arc};
use game::board::{Board, Tile, Dimensions, BoardError, Area};
use std::ops::Deref;
use std::ops::DerefMut;
use interface::{Interface};
use std::thread;
use std::clone::Clone;
use self::rand::Rng;
use std::sync::mpsc::{channel, Receiver};
use std::thread::JoinHandle;
use interface::input::PlayerInput;
use game::gamestate::GameStateManager;
use game::gamestate::object::Point;
use game::gamestate::GameOptions;
use std::mem;
use game::gamestate::Changes;

pub struct Game {
    _board: Arc<RwLock<Board>>,
    _receiver: Option<Receiver<PlayerInput>>,
    _state_manager: Option<GameStateManager>,
    _game_options: Option<GameOptions>
}

impl Game {
    pub fn new() -> Self {
        Game {_board: Arc::new(RwLock::new(Default::default())), _receiver: Option::None,
            _state_manager: Option::None, _game_options: Option::None}
    }

    fn board(&self) -> &RwLock<Board> {
        self._board.deref()
    }

    pub fn new_board(&mut self, tiles: Area, dimensions: Dimensions) {
        let dim=dimensions.clone();
        match Board::with_tiles(tiles, dimensions) {
            Ok(b) => {
                self._board=Arc::new(RwLock::new(b));
                self._game_options = Some(GameOptions { dimensions: dim });
            },
            Err(e) => match e {
                BoardError::WrongLen(mes) => panic!("Couldn't set the board for the game: {}", mes)
            }
        }
    }

    pub fn state_manager(&self) -> &GameStateManager {
        match self._state_manager {
            Some(ref manager) => manager,
            None => panic!("No game state manager to unwrap")
        }
    }

    pub fn state_manager_mut(&mut self) -> &mut GameStateManager {
        match self._state_manager {
            Some(ref mut manager) => manager,
            None => panic!("No game state manager to unwrap")
        }
    }

    pub fn set_state_manager(&mut self, manager: GameStateManager) {
        match self._state_manager {
            None => self._state_manager = Some(manager),
            Some(ref mut old_manager) => {
                mem::replace(old_manager,manager);
            }
        }
    }

    pub fn options(&self) -> &GameOptions {
        match self._game_options {
            Some(ref options) => options,
            None => panic!("No game options to unwrap!")
        }
    }

    pub fn set_options(&mut self, options: GameOptions) {
        match self._game_options {
            None => self._game_options=Some(options),
            Some(ref mut old_options) => {
                mem::replace(old_options, options);
            }
        }
    }
    pub fn add_border(&mut self) {
        let mut guard: RwLockWriteGuard<Board> = self._get_write_lock();
        guard.deref_mut().set_border();
    }

    pub fn erase_board(&mut self) {
        //get the write lock over the board. It panics if it is unable to get it
        let mut guard: RwLockWriteGuard<Board> =self._get_write_lock();
        let Dimensions(x,y) = *(guard.deref().dimensions());
        guard.deref_mut().replace_tiles(vec![Tile::Empty(None); x as usize *y as usize]);
    }

    pub fn begin_rendering(&mut self, interval: u32, valid_keys: [char;5]) -> JoinHandle<()> {
        //pointer to the board, shared between interface and game
        let pointer=Arc::clone(&self._board);
        //setting up the channel: game is the receiver whereas interface is the sender
        let (sender,receiver) = channel();
        self._receiver = Some(receiver); //todo add setter

        thread::spawn(move || {
            let mut interface = Interface::new(pointer, sender);
            interface.new_renderer(interval);
            interface.new_input(&valid_keys);
            interface.render_loop();
        })
    }

    pub fn listen(&self) -> PlayerInput {
        match self._receiver {
            Some(ref receiver) => {
                match receiver.try_recv() {
                    Ok(value) => return value,
                    Err(_) => return PlayerInput::None, //todo check this
                }
            },
            None => panic!("No receiver to use!")
        };
    }

    pub fn map_state(&mut self) {
        let changes: &Option<Changes> = self.state_manager().changes();
        match changes {
            None => (),
            //If there are some changes, map it to the board
            Some(changes) => {
                let mut guard= self._get_write_lock();
                let board= guard.deref_mut();
                for ch in changes {
                    let index: usize = board.as_point(&ch.0) as usize;
                    board.set_tile(index, ch.1.clone());
                }
            }
        }
    }

    pub fn change_random_tile(&self) {
        let mut guard = self._get_write_lock();
        let board: &mut Board = guard.deref_mut();


        let mut rng = rand::thread_rng();
        let Dimensions(x,y) = *board.dimensions();

        board.set_tile(rng.gen::<usize>()%(x*y)as usize,Tile::HBorder(Some('*')));
        board.set_tile(rng.gen::<usize>()%(x*y)as usize,Tile::Empty(None));
    }

    fn _get_write_lock(&self) -> RwLockWriteGuard<Board> {
        // Attempt to get the lock over the board tiles
        let result: LockResult<RwLockWriteGuard<Board>> = self.board().write();

        match result {
            //We got the non-poisoned lock, so we return it alongside
            Ok(guard) => guard,
            Err(_) => panic!("The lock over the boar tiles was poisoned!")
        }

    }

    fn _get_read_lock(&self) -> RwLockReadGuard<Board> {
        // Attempt to get the lock over the board tiles
        let result: LockResult<RwLockReadGuard<Board>> = self.board().read();

        match result {
            //We got the non-poisoned lock, so we return it alongside
            Ok(guard) => guard,
            Err(_) => panic!("The lock over the boar tiles was poisoned!")
        }

    }
}

impl Game {
    pub fn begin_game_loop(&mut self) {
        //Initialize the game state manager
        let options: GameOptions = self.options().clone();
        self.set_state_manager(GameStateManager::new(options));

        //game logic loop
        loop {
            //get the player input
            let input_received=self.listen();

            if input_received==PlayerInput::Character('e') {
                break;
            }

            //do the update logic loop using the player input

            self.state_manager_mut().game_loop(input_received);

            self.map_state()

        }

    }
}