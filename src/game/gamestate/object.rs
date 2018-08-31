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
    fn position(&self) -> &Vec<Coordinates>;
}

#[derive(Debug)]
pub struct Main {
    _id : Uuid,
    _category: ObjectCategory,
    _receptive: bool,
    _position: Vec<Coordinates>,
    //_actions: HashMap<PlayerInput, Box<FnMut(&mut Object<T>) -> ()>>
}

impl Object for Main {
    fn handle_input(&mut self, input: &PlayerInput) {}//todo

    fn category(&self) -> &ObjectCategory {
        &self._category
    }

    fn id(&self) -> &Uuid {
        &self._id
    }

    fn movable(&self) -> bool {
        self._receptive
    }
    fn position(&self) -> &Vec<Coordinates> {&self._position}

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
            Box::from(
                (Main {
                        _id: Uuid::new_v4(),
                        _category: ObjectCategory::Main,
                        _receptive: true,
                        _position: vec!(Coordinates(5,5)),
                    }) as Object
                )) ;
        vec
    }
}