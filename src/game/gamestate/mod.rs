pub mod gamestate;
pub mod object;

use interface::input::PlayerInput;
use std::collections::HashMap;
use uuid::Uuid;
use game::gamestate::gamestate::GameState;
use game::gamestate::object::ObjectCategory;
use game::gamestate::object::Object;
use game::gamestate::object::Point;
use game::board::Dimensions;
use game::board::Tile;
use game::board::Coordinates;
use std::clone::Clone;

pub type Changes = (Coordinates, Tile);

pub type CategoryMap<T> = HashMap<ObjectCategory, HashMap<Uuid, Object<T>>>;

pub trait CategoryMapNew {
    fn new() -> Self;
}

impl<T> CategoryMapNew for CategoryMap<T> {
    fn new() -> Self {
        let mut newhash: CategoryMap<T> =HashMap::new();
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
    Action
}

pub struct GameStateManager<T> {
    _phase: StatePhase,
    _current: GameState<T>,
    _history: Vec<GameState<T>>,
    _input: PlayerInput,
    _options: GameOptions
}

impl<T> GameStateManager<T> {

    fn input(&self) -> &PlayerInput {
        &self._input
    }

    pub fn set_input(&mut self, input: PlayerInput) {
        self._input = input;
    }

    fn save_state(&mut self, state: GameState<T>) {
        self._history.push(state);
    }

    fn last_state(&self) -> Option<&GameState<T>> {
        self._history.last()
    }

    pub fn game_loop(&mut self, input: PlayerInput) {
        //save input
        self.set_input(input);


    }


}

impl GameStateManager<Point> {
    pub fn new(_options: GameOptions) -> Self {
        let current = GameState::<Point>::with_objects();
        let mut history = Vec::new();
        history.push(current);
        GameStateManager {
            _phase: StatePhase::Start,
            _current: GameState::new(),
            _history : history,
            _input: PlayerInput::None,
            _options,

        }
    }

}
/*impl<T> Default for GameStateManager<T> {
    fn default() -> Self {
        GameStateManager {
            _phase: StatePhase::Init,
            _current: None,
            _history: Vec::new(),
            _input: PlayerInput::None,
            _options: Default::default()
        }
    }
}*/


