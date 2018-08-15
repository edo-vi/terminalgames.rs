use interface::input::PlayerInput;

enum StatePhase {
    Init,
    Movement,
    Action,
    Map
}

pub struct GameState {

}

pub struct GameStateManager {
    _state: StatePhase,
    _history: Vec<GameState>,
    _input: PlayerInput
}

impl GameStateManager {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn input(&self) -> &PlayerInput {
        &self._input
    }

    pub fn set_input(&mut self, input: PlayerInput) {
        self._input = input;
    }

    pub fn push_state(&mut self, state: GameState) {
        self._history.push(state);
    }
}

impl Default for GameStateManager {
    fn default() -> Self {
        GameStateManager {
            _state: StatePhase::Init,
            _history: Vec::new(),
            _input: PlayerInput::None
        }
    }
}

