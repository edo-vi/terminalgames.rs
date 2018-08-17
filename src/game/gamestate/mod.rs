pub mod gamestate;
pub mod active;

use interface::input::PlayerInput;
use std::collections::HashMap;
use uuid::Uuid;
use game::gamestate::gamestate::GameState;
use game::gamestate::active::ActiveCategory;
use game::gamestate::active::ActiveObject;

pub type CategoryMap<T> = HashMap<ActiveCategory, HashMap<Uuid, ActiveObject<T>>>;

enum StatePhase {
    Init,
    Movement,
    Action
}

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

pub struct GameStateManager<T> {
    _state: StatePhase,
    _current: Option<GameState<T>>,
    _history: Vec<GameState<T>>,
    _input: PlayerInput,
    _options: GameOptions
}

impl<T> GameStateManager<T> {
    pub fn new(_options: GameOptions) -> Self {
        GameStateManager {
            _options,
            ..Default::default()

        }
    }
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

impl<T> Default for GameStateManager<T> {
    fn default() -> Self {
        GameStateManager {
            _state: StatePhase::Init,
            _current: None,
            _history: Vec::new(),
            _input: PlayerInput::None,
            _options: Default::default()
        }
    }
}


