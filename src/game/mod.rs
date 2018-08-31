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
use game::gamestate::PacManStateManager;
use game::gamestate::object::Point;
use game::gamestate::GameOptions;
use std::mem;
use game::gamestate::Changes;
use game::gamestate::StateManager;
use game::gamestate::updater::Update;
use game::gamestate::checker::Check;
use std::marker::PhantomData;

pub struct Game<S: StateManager<O, U, C>, O: GameOptions, U: Update, C: Check> {
    _board: Arc<RwLock<Board>>,
    _receiver: Option<Receiver<PlayerInput>>,
    _state_manager: S,
    _phantom1: PhantomData<U>,
    _phantom2: PhantomData<C>,
    _phantom3: PhantomData<O>
}

impl<S, O, U, C> Game<S, O, U, C>
where S: StateManager<O, U, C>, O: GameOptions, U: Update, C: Check
{
    pub fn new(_board: Board, _state_manager: S) -> Self {
        Game {
            _board: Arc::new(RwLock::new(_board)),
            _receiver: Option::None,
            _state_manager,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
            _phantom3: PhantomData
        }
    }

    fn board(&self) -> &RwLock<Board> {
        self._board.deref()
    }

    pub fn new_board(&mut self, tiles: Area, dimensions: Dimensions) {
        let dim=dimensions.clone();
        match Board::with_tiles(tiles, dimensions) {
            Ok(b) => {
                self._board=Arc::new(RwLock::new(b));
                self.state_manager_mut().set_options(O::new(dim));
            },
            Err(e) => match e {
                BoardError::WrongLen(mes) => panic!("Couldn't set the board for the game: {}", mes)
            }
        }
    }

    pub fn state_manager(&self) -> &S {
        &self._state_manager
    }

    pub fn state_manager_mut(&mut self) -> &mut S {
        &mut self._state_manager
    }

    pub fn set_state_manager(&mut self, manager: S) {
        mem::replace(self.state_manager_mut(),manager);
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

    pub fn begin_game_loop(&mut self) {
        //game logic loop
        loop {
            //get the player input
            let input_received=self.listen();

            if input_received==PlayerInput::Character('e') {
                break;
            }

            //do the update logic loop using the player input

            let changes: Option<Changes>;
            {
                changes=self.state_manager_mut().update_state(input_received);
            }
            self.map_state(changes);
        }

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

    pub fn map_state(&mut self, changes: Option<Changes>) {
        match changes {
            None => (),
            //If there are some changes, map it to the board
            Some(changes) => {
                let mut guard= self._get_write_lock();
                let board= guard.deref_mut();
                for ch in changes {
                    let index: usize = board.as_point(&ch.0) as usize;
                    board.set_tile(index, ch.1);
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

