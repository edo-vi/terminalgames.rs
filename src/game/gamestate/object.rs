use super::super::super::simplelog;
#[macro_use] use super::super::super::log;

use interface::input::PlayerInput;
use uuid::Uuid;
use game::board::Coordinates;
use std::collections::HashMap;

pub type Point = Coordinates;
pub type Coords = Vec<Coordinates>;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum ObjectCategory {
    Main,
    Enemy,
    NonActive,
    Finished
}

impl ObjectCategory {
    //todo remove hardcoded
    pub fn categories() -> Vec<ObjectCategory> {
        let mut vec = Vec::new();
        vec.push(ObjectCategory::Main);
        vec.push(ObjectCategory::Enemy);
        vec.push(ObjectCategory::NonActive);
        vec.push(ObjectCategory::Finished);

        vec
    }
}

pub trait Object {
    fn handle_input(&mut self, input: &PlayerInput);
    fn category(&self) -> &ObjectCategory;
    fn id(&self) -> &Uuid;
    fn movable(&self) -> bool;
    fn current_position(&self) -> &Vec<Coordinates>;
    fn next_position(&self) -> Option<&Vec<Coordinates>>;
}

#[derive(Debug)]
pub struct Main {
    _id : Uuid,
    _category: ObjectCategory,
    _movable: bool,
    _position: Vec<Coordinates>,
    _next_position: Option<Vec<Coordinates>>
}

impl Object for Main {
    fn handle_input(&mut self, input: &PlayerInput) {
        match input {
            PlayerInput::Character('a') => {
                let mut vec = Vec::new();
                for v in self.current_position() {
                    vec.push(v.clone());
                }
                for pos in &mut vec {
                    pos.0 = pos.0-1
                }
                self._next_position = Some(vec);
            },
            PlayerInput::Character('d') => {
                let mut vec = Vec::new();
                for v in self.current_position() {
                    vec.push(v.clone());
                }
                for pos in &mut vec {
                    pos.0 = pos.0+1
                }
                self._next_position = Some(vec);
            },
            PlayerInput::Character('w') => {
                let mut vec = Vec::new();
                for v in self.current_position() {
                    vec.push(v.clone());
                }
                for pos in &mut vec {
                    pos.1 = pos.1-1
                }
                self._next_position = Some(vec);
            },
            PlayerInput::Character('s') => {
                let mut vec = Vec::new();
                for v in self.current_position() {
                    vec.push(v.clone());
                }
                for pos in &mut vec {
                    pos.1 = pos.1+1
                }
                self._next_position = Some(vec);
            },
            _ => ()
        }
        info!("{:?}", self._position);
        info!("{:?}", self._next_position);
    }

    fn category(&self) -> &ObjectCategory {
        &self._category
    }

    fn id(&self) -> &Uuid {
        &self._id
    }

    fn movable(&self) -> bool {
        self._movable
    }

    fn current_position(&self) -> &Vec<Coordinates> {&self._position}

    fn next_position(&self) -> Option<&Vec<Coordinates>> {
        match self._next_position {
            None => None,
            Some(ref coords) => Some(coords)
        }
    }
}

pub trait ObjectFactory {
    fn firsts() -> Vec<Box<Object>>;
}

pub struct MainFactory {}

impl ObjectFactory for MainFactory {
    ///Creates the first objects to be placed on the board
    fn firsts() -> Vec<Box<Object>> {
        let mut vec = Vec::new();
        vec.push(
            Box::new(
                Main {
                        _id: Uuid::new_v4(),
                        _category: ObjectCategory::Main,
                        _movable: true,
                        _position: vec!(Coordinates(5,5)),
                        _next_position: None
                    }
                ) as Box<Object>) ;
        vec
    }
}