pub mod gamestate;
pub mod object;

use interface::input::PlayerInput;
use std::collections::HashMap;
use uuid::Uuid;
use game::gamestate::gamestate::GameState;
use game::gamestate::object::ObjectCategory;
use game::gamestate::object::Object;
use game::gamestate::object::Point;

pub type CategoryMap<T> = HashMap<ObjectCategory, HashMap<Uuid, Object<T>>>;


pub struct GameOptions {
    dim_x: u32,
    dim_y: u32,
}

impl Default for GameOptions {
    fn default() -> Self {
        GameOptions {
            dim_x: 0,
            dim_y: 0
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

}

impl GameStateManager<Point> {
    pub fn new(_options: GameOptions) -> Self {
        GameStateManager {
            _phase: StatePhase::Start,
            _current: GameState::<Point>::with_objects(),
            _history : Vec::new(),
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


