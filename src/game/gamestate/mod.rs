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
use game::gamestate::updater::PacManUpdater;
use game::gamestate::checker::PacManChecker;

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
/// Game state manager: initializes the game state and updates it accordingly to the player input,
/// sets up the updater and checker objects, returns the state changes at each cycle.
pub trait StateManager<O: GameOptions, U: Update, C: Check>{
    ///Creates a new instance of the State Manager, with the necessary options to manage the game state.
    fn new(options: O, updater: U, checker: C) -> Self;
    fn update_state(&mut self, input: PlayerInput) -> Option<Changes>;
}

pub struct PacManStateManager<U: Update, C: Check> {
    _phase: StatePhase,
    _current: Option<GameState>,
    _history: Vec<GameState>,
    _input: PlayerInput,
    _options: PacManOptions,
    _changes: Option<Changes>,
    _updater: U,
    _checker: C
}

impl<U: Update ,C: Check> StateManager<PacManOptions, U, C> for PacManStateManager<U,C> {
    fn new(_options: PacManOptions, _updater: U, _checker: C) -> Self {
        let current = GameState::with_objects();
        let mut history = Vec::new();
        history.push(current);

        let mut new_gsm= PacManStateManager {
            _phase: StatePhase::Start,
            _current: None,
            _history : history,
            _input: PlayerInput::None,
            _options,
            _changes: None,
            _updater,
            _checker
        };
        // this makes sense because when a new game_state_manager is instantiate the initial state is
        // set, and this state is to be considered as a change from the previous null state, when
        // the board is still to be rendered
        new_gsm.lasts_as_changes();
        new_gsm
    }

    fn update_state(&mut self, input: PlayerInput) -> Option<Changes> {
        Some(self.lasts_as_changes())
    }

}
impl PacManStateManager<PacManUpdater, PacManChecker> {

    pub fn lasts_as_changes(&mut self) -> Changes {
        let mut changes: Changes = Vec::new();
        for obj in self.last_state().all_objects() {
            for pos in obj.position() {
                changes.push((pos.clone(),Tile::Active(None)));
            }

        }
        changes
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



