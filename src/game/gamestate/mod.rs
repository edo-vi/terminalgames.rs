

pub mod gamestate;
pub mod object;
pub mod updater;
pub mod checker;

use std::time::{SystemTime, UNIX_EPOCH};

use game::gamestate::object::ObjectFactory;
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
use game::gamestate::updater::SnakeUpdater;
use game::gamestate::checker::SnakeChecker;
use game::gamestate::object::MainFactory;
use game::gamestate::object::WallFactory;
use game::gamestate::object::PowerUpFactory;
use std::time::Instant;
use game::board::Euclidean;
use game::board::Mappable;

pub type Changes = Vec<(Coordinates, Tile)>;

pub trait GameOptions: Clone + Default {
    fn new(board_dimension: Dimensions) -> Self;
    fn dimensions(&self) -> &Dimensions;
    fn set_dimensions(&mut self, dim: Dimensions);
}

#[derive(Clone)]
pub struct SnakeOptions {
    _dimensions: Dimensions,
    _last_input: PlayerInput,
    _free_positions: Option<Vec<Coordinates>>
}

impl GameOptions for SnakeOptions {
    fn new(board_dimension: Dimensions) -> Self {
        SnakeOptions {
            _dimensions: board_dimension,
            _last_input: PlayerInput::Character('d'),
            _free_positions: None
        }
    }
    fn dimensions(&self) -> &Dimensions {
        &self._dimensions
    }
    fn set_dimensions(&mut self, dim: Dimensions) {
        self._dimensions = dim;
    }
}

impl Default for SnakeOptions {
    fn default() -> Self {
        SnakeOptions {
            _dimensions: Dimensions(0,0),
            _last_input: PlayerInput::Character('d'),
            _free_positions: None
        }
    }
}

impl SnakeOptions {
    pub fn last_input(&self) -> &PlayerInput {
        &self._last_input
    }
    pub fn set_last_input(&mut self, input: PlayerInput) {
        self._last_input=input;
    }
    pub fn free_positions(&self) -> &Option<Vec<Coordinates>> {
        &self._free_positions
    }
    pub fn set_free_positions(&mut self, free_pos: Option<Vec<Coordinates>>) {
        self._free_positions=free_pos;
    }
}
#[derive(PartialEq, Eq)]
pub enum StatePhase {
    Start,
    Normal,
    Create,
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

pub struct SnakeStateManager<O: GameOptions, U: Update, C: Check> {
    _phase: StatePhase,
    _current: Vec<Box<Object>>,
    _timer: Instant,
    _options: O,
    _updater: U,
    _checker: C
}

//Specialized implementation with SnakeOptions
impl<U: Update, C: Check> StateManager<SnakeOptions, U, C> for SnakeStateManager<SnakeOptions, U, C> {
    fn new(mut _options: SnakeOptions, _updater: U, _checker: C) -> Self {
        let mut _main: Vec<Box<Object>> = MainFactory::firsts(_options.clone());
        let mut _wall: Vec<Box<Object>> = WallFactory::firsts(_options.clone());

        _main.append(&mut _wall);

        //get all the occupied position to randomly chose one (not taken) to be the powerup position

        let mut _powerups = PowerUpFactory::new_on_free(_options.clone(), &mut _main);

        _main.append(&mut _powerups);
        let new_gsm= SnakeStateManager {
            _phase: StatePhase::Start,
            _current: _main,
            _timer: Instant::now(),
            _options,
            _updater,
            _checker
        };
        // this makes sense because when a new game_state_manager is instantiate the initial state is
        // set, and this state is to be considered as a change from the previous null state, when
        // the board is still to be rendered
        new_gsm
    }

    fn set_options(&mut self, options: SnakeOptions) {
        self._options = options;
    }
    fn set_updater(&mut self, updater: U) {
        self._updater = updater;
    }
    fn set_checker(&mut self, checker: C) {
        self._checker = checker;
    }

    fn update_state(&mut self, input: PlayerInput) -> Option<Changes> {
        let new_instant = Instant::now();
        //if it is time to move the snake
        if new_instant.duration_since(self._timer).as_millis() >= 110 {
            let input = self._options.last_input().clone();
            &self._updater.update_objects(&mut self._current, input);
            &self._checker.checks(&mut self._current, &mut self._phase);
            self._timer=new_instant;
        }
        //else if it is the first update
        else if self._phase == StatePhase::Start {
            self._phase = StatePhase::Normal;
        }
        //any other case
        else if self._phase == StatePhase::Normal {
            &self._updater.update_objects(&mut self._current, input.clone());
            &self._checker.checks(&mut self._current, &mut self._phase);
            //set last input as the last input the player has given, if it is a character
            match input {
                PlayerInput::Character('a') | PlayerInput::Character('s') |
                PlayerInput::Character('d') | PlayerInput::Character('w') => {
                    self._options.set_last_input(input);
                    //reset timer
                    self._timer = Instant::now();
                },
                _ => ()
            }
        }
        else if self._phase == StatePhase::Create {
            self._phase = StatePhase::Normal;
            let mut new = PowerUpFactory::new_on_free(self._options.clone(), &mut self._current);
            self._current.append(&mut new);

            //continue normally
            &self._updater.update_objects(&mut self._current, input.clone());
            &self._checker.checks(&mut self._current, &mut self._phase);
            //set last input as the last input the player has given, if it is a character
            match input {
                PlayerInput::Character('a') | PlayerInput::Character('s') |
                PlayerInput::Character('d') | PlayerInput::Character('w') => {
                    self._options.set_last_input(input);
                    //reset timer
                    self._timer = Instant::now();
                },
                _ => ()
            }
        } else {
        }



        //In any case, complete the update and then produce the changes
        Self::complete_update(&mut self._current);
        Self::produce_changes(&mut self._current)

    }
}


impl<O: GameOptions, U: Update, C: Check> SnakeStateManager<O, U, C> {

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
