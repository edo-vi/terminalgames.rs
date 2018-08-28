use super::super::log;
use super::super::simplelog;

pub mod gamestate;
pub mod object;
pub mod updater;
pub mod checker;

use interface::input::PlayerInput;
use std::collections::HashMap;
use uuid::Uuid;
use game::gamestate::gamestate::GameState;
use game::gamestate::object::ObjectCategory;
use game::gamestate::object::Main;
use game::gamestate::object::Point;
use game::board::Dimensions;
use game::board::Tile;
use game::board::Coordinates;
use std::clone::Clone;
use game::gamestate::object::Object;
use std::fmt::Debug;
use game::gamestate::updater::Update;
use game::gamestate::checker::Check;

pub type Changes = Vec<(Coordinates, Tile)>;




pub trait GameOptions: Clone {
    fn new(board_dimension: Dimensions) -> Self;
    fn dimensions(&self) -> &Dimensions;
}

#[derive(Clone)]
pub struct PacManOptions {
    _dimensions: Dimensions
}

impl GameOptions for PacManOptions {
    fn new(board_dimension: Dimensions) -> Self {
        PacManOptions {
            _dimensions: board_dimension
        }
    }
    fn dimensions(&self) -> &Dimensions {
        &self._dimensions
    }
}
impl Default for PacManOptions {
    fn default() -> Self {
        PacManOptions {
            _dimensions: Dimensions(0,0)
        }
    }
}

pub enum StatePhase {
    Start,
    Movement,
    Action,
    Checks,
    End
}
/* (updater: &T, objs: &Vec<Box<Object>>,
                    input: PlayerInput, phase: StatePhase) -> Vec<Box<Object> */
/// Game state manager: initializes the game state and updates it accordingly to the player input,
/// sets up the updater and checker objects, returns the state changes at each cycle.
pub trait StateManager<O: GameOptions>{
    ///Creates a new instance of the State Manager, with the necessary options to manage the game state.
    fn new(options: O) -> Self;
    fn update_state(&mut self, input: PlayerInput) -> Option<Changes>;
}

pub struct GameStateManager {
    _phase: StatePhase,
    _current: GameState,
    _history: Vec<GameState>,
    _input: PlayerInput,
    _options: PacManOptions,
    _changes: Option<Changes>
}

impl StateManager<PacManOptions> for GameStateManager {
    fn new(_options: PacManOptions) -> Self {
        let current = GameState::with_objects();
        let mut history = Vec::new();
        history.push(current);

        let mut new_gsm= GameStateManager {
            _phase: StatePhase::Start,
            _current: GameState::new(),
            _history : history,
            _input: PlayerInput::None,
            _options,
            _changes: None
        };
        // this makes sense because when a new game_state_manager is instantiate the initial state is
        // set, and this state is to be considered as a change from the previous null state, when
        // the board is still to be rendered
        new_gsm.lasts_as_changes();
        new_gsm
    }

    fn update_state(&mut self, input: PlayerInput) -> Option<Changes> {
        Option::None
    }

}
impl GameStateManager {

    pub fn lasts_as_changes(&mut self) {
        let mut changes: Changes = Vec::new();
        for obj in self.last_state().all_objects() {
            for pos in obj.position() {
                changes.push((pos.clone(),Tile::Active(None)));
            }

        }
        self._changes=Some(changes);
    }

    fn input(&self) -> &PlayerInput {
        &self._input
    }

    pub fn set_input(&mut self, input: PlayerInput) {
        self._input = input;
    }

    fn save_state(&mut self, state: GameState) {
        self._history.push(state);
    }

    fn last_state(&self) -> &GameState {
        match self._history.last() {
            Some(ref state) => state,
            None => panic!("History is empty, no last state to extract!")
        }
    }
    pub fn changes(&self) -> &Option<Changes> {
        &self._changes
    }
    pub fn game_loop(&mut self, input: PlayerInput) {

        //save input
        self.set_input(input);


    }

}



