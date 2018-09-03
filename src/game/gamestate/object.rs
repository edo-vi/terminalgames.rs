
#[macro_use] use super::super::super::log;

use interface::input::PlayerInput;
use uuid::Uuid;
use game::board::Coordinates;
use std::collections::HashMap;
use game::board::Tile;
use game::board::Dimensions;
use game::board::Mappable;
use game::board::Euclidean;

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
    fn set_current_position(&mut self, pos: &Vec<Coordinates>);
    fn next_position(&self) -> Option<&Vec<Coordinates>>;
    fn set_next_position(&mut self, pos: Option<&Vec<Coordinates>>);
    fn tile(&self) -> Tile;
}

#[derive(Debug)]
pub struct Main {
    _id : Uuid,
    _category: ObjectCategory,
    _tile: Tile,
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

    fn set_current_position(&mut self, pos: &Vec<Coordinates>) {
        self._position = pos.clone()
    }

    fn next_position(&self) -> Option<&Vec<Coordinates>> {
        match self._next_position {
            None => None,
            Some(ref coords) => Some(coords)
        }
    }

    fn set_next_position(&mut self, pos: Option<&Vec<Coordinates>>) {
        match pos {
            None => self._next_position = None,
            Some(pos) => self._next_position = Some(pos.clone())
        }
    }

    fn tile(&self) -> Tile {
        self._tile.clone()
    }
}

pub trait ObjectFactory {
    fn firsts(dim: &Dimensions) -> Vec<Box<Object>>;
}

pub struct MainFactory {}

impl ObjectFactory for MainFactory {
    ///Creates the first objects to be placed on the board
    fn firsts(dim: &Dimensions) -> Vec<Box<Object>> {
        let vec = vec!(
            Box::new(
                Main {
                        _id: Uuid::new_v4(),
                        _category: ObjectCategory::Main,
                        _movable: true,
                        _tile: Tile::Active(None),
                        _position: vec!(Coordinates(5,5)),
                        _next_position: None
                    }
                ) as Box<Object>) ;

        vec
    }
}

pub struct Wall {
    _id : Uuid,
    _category: ObjectCategory,
    _tile: Tile,
    _movable: bool,
    _position: Vec<Coordinates>,
    _next_position: Option<Vec<Coordinates>>
}

impl Object for Wall {
    fn handle_input(&mut self, input: &PlayerInput) {

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

    fn set_current_position(&mut self, pos: &Vec<Coordinates>) {
        self._position = pos.clone()
    }

    fn next_position(&self) -> Option<&Vec<Coordinates>> {
        match self._next_position {
            None => None,
            Some(ref coords) => Some(coords)
        }
    }

    fn set_next_position(&mut self, pos: Option<&Vec<Coordinates>>) {
        match pos {
            None => self._next_position = None,
            Some(pos) => self._next_position = Some(pos.clone())
        }
    }

    fn tile(&self) -> Tile {
        self._tile.clone()
    }
}
pub struct WallFactory {}

impl ObjectFactory for WallFactory {
    fn firsts(dim: &Dimensions) -> Vec<Box<Object>> {
        let coordinates: Vec<Coordinates> = (0..dim.x()*dim.y()).map(|a| {
            info!("{:?} <=> {:?}", a, dim.as_coord(a));
            dim.as_coord(a)
        })
            .filter(|a| {
                a.x()==0 || a.x()==(dim.x()-1) || a.y()==0 || a.y()==(dim.y())-1
            }).collect();
        let vec = vec!(
            Box::new(
                Wall {
                    _id: Uuid::new_v4(),
                    _category: ObjectCategory::NonActive,
                    _movable: false,
                    _tile: Tile::VBorder(None),
                    _position: coordinates,
                    _next_position: None
                }
            ) as Box<Object>
        );

        vec
    }
}