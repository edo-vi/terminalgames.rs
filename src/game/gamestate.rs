use interface::input::PlayerInput;
use std::collections::HashMap;
use game::board::Coordinates;
use uuid::Uuid;

enum StatePhase {
    Init,
    Movement,
    Action,
    Map
}

pub struct GameStateManager<T> {
    _state: StatePhase,
    _current: GameState<T>,
    _history: Vec<GameState<T>>,
    _input: PlayerInput
}

impl<T> GameStateManager<T> {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn input(&self) -> &PlayerInput {
        &self._input
    }

    pub fn set_input(&mut self, input: PlayerInput) {
        self._input = input;
    }

    fn push_state(&mut self, state: GameState<T>) {
        self._history.push(state);
    }
}

impl<T> Default for GameStateManager<T> {
    fn default() -> Self {
        GameStateManager {
            _state: StatePhase::Init,
            _current: Default::default(),
            _history: Vec::new(),
            _input: PlayerInput::None
        }
    }
}


pub struct GameState<T> {
    objects: HashMap<Uuid, ActiveObject<T>>,
    end: bool,
}

impl<T> Default for GameState<T> {
    fn default() -> Self {
        GameState {
            objects: HashMap::new(),
            end: false
        }
    }
}


struct ActiveObject<T> {
    _id : Uuid,
    _position: Option<T>
}

impl<T> Default for ActiveObject<T> {
    fn default() -> Self {
        ActiveObject {
            _id: Uuid::new_v4(),
            _position: None
        }
    }
}

