use std::collections::HashMap;
use game::gamestate::CategoryMap;

pub struct GameState<T> {
    objects: CategoryMap<T>,
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
