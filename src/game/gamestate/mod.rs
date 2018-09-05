
use game::gamestate::object::ObjectFactory;

pub mod gamestate;
pub mod object;
pub mod updater;
pub mod checker;

use interface::input::PlayerInput;
use std::collections::HashMap;
use uuid::Uuid;
use game::gamestate::object::ObjectCategory;
use game::gamestate::object::Point;
use game::board::Dimensions;
use game::board::Tile;
use game::board::Coordinates;
use std::clone::Clone;
use std::ops::Deref;
use game::gamestate::object::Object;
use std::fmt::Debug;
use game::gamestate::updater::Update;
use game::gamestate::checker::Check;
use game::gamestate::updater::PacManUpdater;
use game::gamestate::checker::PacManChecker;
use game::gamestate::object::MainFactory;
use game::gamestate::object::WallFactory;

pub type Changes = Vec<(Coordinates, Tile)>;

pub trait GameOptions: Clone + Default {
    fn new(board_dimension: Dimensions) -> Self;
    fn dimensions(&self) -> &Dimensions;
    fn set_dimensions(&mut self, dim: Dimensions);
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
    fn set_dimensions(&mut self, dim: Dimensions) {
        self._dimensions = dim;
    }
}

impl Default for PacManOptions {
    fn default() -> Self {
        PacManOptions {
            _dimensions: Dimensions(0,0)
        }
    }
}
#[derive(PartialEq, Eq)]
pub enum StatePhase {
    Start,
    Normal,
    End
}
/// Game state manager: initializes the game state and updates it accordingly to the player input,
/// sets up the updater and checker objects, returns the state changes at each cycle.
pub trait StateManager<O: GameOptions, U: Update, C: Check>{
    ///Creates a new instance of the State Manager, with the necessary options to manage the game state.
    fn new(options: O, updater: U, checker: C) -> Self;
    fn set_options(&mut self, options: O);
    fn set_updater(&mut self, updater: U);
    fn set_checker(&mut self, checker: C);
    fn update_state(&mut self, input: PlayerInput) -> Option<Changes>;
}

pub struct PacManStateManager<O: GameOptions, U: Update, C: Check> {
    _phase: StatePhase,
    _current: Vec<Box<Object>>,
    _options: O,
    _updater: U,
    _checker: C
}

impl<O: GameOptions, U: Update, C: Check> StateManager<O, U, C> for PacManStateManager<O, U, C> {
    fn new(_options: O, _updater: U, _checker: C) -> Self {
        let mut _main = MainFactory::firsts(_options.dimensions());
        let mut _wall = WallFactory::firsts(_options.dimensions());

        _main.append(&mut _wall);
        let new_gsm= PacManStateManager {
            _phase: StatePhase::Start,
            _current: _main,
            _options,
            _updater,
            _checker
        };
        // this makes sense because when a new game_state_manager is instantiate the initial state is
        // set, and this state is to be considered as a change from the previous null state, when
        // the board is still to be rendered
        new_gsm
    }

    fn set_options(&mut self, options: O) {
        self._options = options;
    }
    fn set_updater(&mut self, updater: U) {
        self._updater = updater;
    }
    fn set_checker(&mut self, checker: C) {
        self._checker = checker;
    }

    fn update_state(&mut self, input: PlayerInput) -> Option<Changes> {
        if self._phase == StatePhase::Start {
            self._phase = StatePhase::Normal;
            Some(self.last_state_as_changes())
        } else {
            &self._updater.update_objects(&mut self._current, input);
            &self._checker.checks(&mut self._current);
            Self::complete_update(&mut self._current);
            Self::produce_changes(&mut self._current)
        }
    }



}

impl<O: GameOptions, U: Update, C: Check> PacManStateManager<O, U, C> {

    pub fn last_state_as_changes(&mut self) -> Changes {
        let mut changes: Changes = Vec::new();
        for obj in &self._current {
            for pos in obj.deref().current_position() {
                changes.push((pos.clone(),Tile::Active(None)));
            }

        }
        changes
    }
    ///Ends the process of updating all objects, by setting their current position as their next one,
    /// which will become none. This function must be called before ending the update_objects function,
    /// and after doing all the necessary checks on the objects;
    pub fn complete_update(objs: &mut Vec<Box<Object>>) {
        for obj in objs {
            obj.set_next_position_as_current();
        }
    }
    ///Produce a (Coordinates, Tile) vector based on the current position of all objects passed as arguments
    pub fn produce_changes(objs: &mut Vec<Box<Object>>) -> Option<Changes> {
        let mut vec = Vec::new();
        for obj in objs {
            for pos in obj.current_position() {
                vec.push((pos.clone(), obj.tile()))
            }
        }
        Some(vec)
    }

}



