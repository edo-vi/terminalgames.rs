

#[macro_use] use super::super::super::log;

extern crate rand;
use self::rand::Rng;

use interface::input::PlayerInput;
use uuid::Uuid;
use game::board::Coordinates;
use std::collections::HashMap;
use game::board::Tile;
use game::board::Dimensions;
use game::board::Mappable;
use game::board::Euclidean;
use game::gamestate::GameOptions;
use game::gamestate::SnakeOptions;
use std::ops::Deref;

pub type Point = Coordinates;
pub type Coords = Vec<Coordinates>;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum ObjectCategory {
    Main,
    Enemy,
    NonActive,
    Wall,
    Finished,
    PowerUp
}

impl ObjectCategory {
    //todo remove hardcoded
    pub fn categories() -> Vec<ObjectCategory> {
        let mut vec = Vec::new();
        vec.push(ObjectCategory::Main);
        vec.push(ObjectCategory::Enemy);
        vec.push(ObjectCategory::NonActive);
        vec.push(ObjectCategory::Wall);
        vec.push(ObjectCategory::Finished);
        vec.push(ObjectCategory::PowerUp);

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
    fn reset_next_position(&mut self);
    fn set_next_position_as_current(&mut self);
    fn tile(&self) -> Tile;
}

#[derive(Debug)]
pub struct Snake {
    _id : Uuid,
    _category: ObjectCategory,
    _tile: Tile,
    _movable: bool,
    _position: Vec<Coordinates>,
    _next_position: Option<Vec<Coordinates>>,
    _last: Coordinates
}

impl Object for Snake {
    fn handle_input(&mut self, input: &PlayerInput) {
        match input {
            PlayerInput::Character('a') => {
                let mut vec = Vec::new();
                for v in self.current_position() {
                    vec.push(v.clone());
                }
                let first = vec[0].clone();
                vec.insert(1, first);

                vec[0].0 -= 1;
                self._next_position = Some(vec);
            },
            PlayerInput::Character('d') => {
                let mut vec = Vec::new();
                for v in self.current_position() {
                    vec.push(v.clone());
                }

                let first = vec[0].clone();
                vec.insert(1, first);

                vec[0].0 += 1;

                self._next_position = Some(vec);
            },
            PlayerInput::Character('w') => {
                let mut vec = Vec::new();
                for v in self.current_position() {
                    vec.push(v.clone());
                }

                let first = vec[0].clone();
                vec.insert(1, first);

                vec[0].1 -= 1;
                self._next_position = Some(vec);
            },
            PlayerInput::Character('s') => {
                let mut vec = Vec::new();
                for v in self.current_position() {
                    vec.push(v.clone());
                }

                let first = vec[0].clone();
                vec.insert(1, first);

                vec[0].1 += 1;
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

    fn reset_next_position(&mut self) {
        self._next_position = None;
    }

    fn set_next_position_as_current (&mut self) {
        match &self._next_position {
            None => (), // do nothing, as we can't and shouldn't set current position as None
            Some(pos) => {
                self._position=pos.clone();
            }
        }
        self._next_position=None;
    }
    fn tile(&self) -> Tile {
        self._tile.clone()
    }


}

pub trait ObjectFactory<O: GameOptions> {
    fn firsts(_options: O) -> Vec<Box<Object>>;
}

pub struct MainFactory {}

impl<O: GameOptions> ObjectFactory<O> for MainFactory {
    ///Creates the first objects to be placed on the board
    fn firsts(_options: O) -> Vec<Box<Object>> {
        let vec = vec!(
            Box::new(
                Snake {
                        _id: Uuid::new_v4(),
                        _category: ObjectCategory::Main,
                        _movable: true,
                        _tile: Tile::Active(Some('#')),
                        _position: vec!(Coordinates(6,5), Coordinates(5,5), Coordinates(4,5)),
                        _next_position: None,
                        _last: Coordinates(4,5)
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

    fn reset_next_position(&mut self) {
        self._next_position = None;
    }

    fn set_next_position_as_current (&mut self) {
        match &self._next_position {
            None => (), // do nothing, as we can't and shouldn't set current position as None
            Some(pos) => {
                self._position=pos.clone();
            }
        }
        self._next_position=None;
    }

    fn tile(&self) -> Tile {
        self._tile.clone()
    }
}
pub struct WallFactory {}

impl<O: GameOptions> ObjectFactory<O> for WallFactory {
    fn firsts(_options: O) -> Vec<Box<Object>> {
        let dim = _options.dimensions();
        let coordinates: Vec<Coordinates> = (0.. dim.x()*dim.y()).map(|a| {
            dim.as_coord(a)
        })
            .filter(|a: &Coordinates| {
                a.x()==0 || a.x()==(dim.x()-1) || a.y()==0 || a.y()==(dim.y())-1
            }).collect();

        let vec = vec!(
            Box::new(
                Wall {
                    _id: Uuid::new_v4(),
                    _category: ObjectCategory::Wall,
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

pub struct PowerUpFactory{}

default impl<O: GameOptions> ObjectFactory<O> for PowerUpFactory {
    fn firsts(_options: O) -> Vec<Box<Object>> {
        let vec = vec!(
            Box::new(
                Wall {
                    _id: Uuid::new_v4(),
                    _category: ObjectCategory::PowerUp,
                    _movable: false,
                    _tile: Tile::Active(Some('\'')),
                    _position: vec!(Coordinates(9,9)),
                    _next_position: None
                }
            ) as Box<Object>
        );

        vec
    }
}

//specialized implementation of ObjectFactory for PowerUpFactory
impl ObjectFactory<SnakeOptions> for PowerUpFactory {
    fn firsts(_options: SnakeOptions) -> Vec<Box<Object>> {
        let coord: Coordinates;
        match _options.free_positions() {
            None => coord = Coordinates(9,9),
            Some(ref free) => {
                coord = rand::thread_rng().choose(free).unwrap().clone();
            }
        }
        vec!(
            Box::new(
                Wall {
                    _id: Uuid::new_v4(),
                    _category: ObjectCategory::PowerUp,
                    _movable: false,
                    _tile: Tile::Active(Some('o')),
                    _position: vec!(coord),
                    _next_position: None
                }
            ) as Box<Object>
        )
    }
}

impl PowerUpFactory {
    pub fn new_on_free(_options: SnakeOptions, objs: &mut Vec<Box<Object>>) -> Vec<Box<Object>> {
        let occupied_pos: Vec<Coordinates> = objs.iter().filter(|a| *(a.deref().category())!=ObjectCategory::PowerUp)
            .flat_map(|a| {
                a.current_position().clone()
            }).collect();

        let mut free_pos: Vec<Coordinates> = Vec::new();

        //get free positions
        for pos in (0.._options.dimensions().x()*_options.dimensions().y()).map(|a: u16| {
            let b: Coordinates = _options.dimensions().as_coord(a);
            b
        }) {
            if !occupied_pos.contains(&pos) {
                free_pos.push(pos.clone())
            }
        }

        vec!(
            Box::new(
                Wall {
                    _id: Uuid::new_v4(),
                    _category: ObjectCategory::PowerUp,
                    _movable: false,
                    _tile: Tile::Active(Some('o')),
                    _position: vec!(rand::thread_rng().choose(&free_pos).unwrap().clone()),
                    _next_position: None
                }
            ) as Box<Object>
        )
    }
}
