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

pub type CategoryMap = HashMap<ObjectCategory, HashMap<Uuid, Main>>;

pub trait CategoryMapNew {
    fn new() -> Self;
}

impl CategoryMapNew for CategoryMap {
    fn new() -> Self {
        let mut newhash: CategoryMap =HashMap::new();
        for cat in ObjectCategory::categories() {
            newhash.insert(cat,HashMap::new());
        }

        newhash
    }
}

#[derive(Clone)]
pub struct GameOptions {
    pub dimensions: Dimensions
}

impl Default for GameOptions {
    fn default() -> Self {
        GameOptions {
            dimensions: Dimensions(0,0)
        }
    }
}

enum StatePhase {
    Start,
    Movement,
    Action,
    Checks,
    End
}

pub trait Manage<T: Update>{
    fn update_objects(updater: T, objs: Vec<Box<Object>>, input: PlayerInput);
    fn get_changes(old: Vec<Box<Object>>, new: Vec<Box<Object>>) -> Changes;
}

pub struct GameStateManager {
    _phase: StatePhase,
    _current: GameState,
    _history: Vec<GameState>,
    _input: PlayerInput,
    _options: GameOptions,
    _changes: Option<Changes>
}

impl GameStateManager {

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

impl GameStateManager {
    pub fn new(_options: GameOptions) -> Self {
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
    //this requires a concrete type (Point or Coords)
    pub fn lasts_as_changes(&mut self) {
        let mut changes: Changes = Vec::new();
        for obj in self.last_state().all_objects() {
            for pos in obj.position() {
                changes.push((pos.clone(),Tile::Active(None)));
            }

        }
        self._changes=Some(changes);
    }

}



