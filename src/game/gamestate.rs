use interface::input::PlayerInput;
use std::collections::HashMap;
use game::board::Coordinates;
use uuid::Uuid;

pub type CategoryMap = HashMap<ActiveCategory, HashMap<Uuid, ActiveObject<T>>>;

enum StatePhase {
    Init,
    Movement,
    Action,
    Map
}

pub struct GameStateManager<T> {
    _state: StatePhase,
    _current: Option<GameState<T>>,
    _history: Vec<GameState<T>>,
    _input: PlayerInput
}

impl<T> GameStateManager<T> {
    pub fn new() -> Self {
        Default::default()
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
            _current: Default::default(),
            _history: Vec::new(),
            _input: PlayerInput::None
        }
    }
}

pub struct GameState<T> {
    objects: CategoryMap,
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

enum ActiveCategory {
    Main,
    Enemy
}

pub trait Active {
    fn handle_input(&mut self, input: &PlayerInput);
    fn category(&self) -> ActiveCategory;
    fn set_category(&mut self);
    fn id(&self);
}

struct ActiveObject<T> {
    _id : Uuid,
    _category: ActiveCategory,
    _position: T
}

