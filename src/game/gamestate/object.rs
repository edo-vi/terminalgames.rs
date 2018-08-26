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
    fn set_category(&mut self, category: ObjectCategory);
    fn id(&self) -> &Uuid;
    fn set_id(&mut self, id: Uuid);
    fn receptiveness(&self) -> bool;
    fn set_receptiveness(&mut self, recept: bool);
    fn position(&self) -> &Vec<Coordinates>;
}

#[derive(Clone, Debug)]
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

    fn set_category(&mut self, category: ObjectCategory) {
        self._category = category;
    }
    fn id(&self) -> &Uuid {
        &self._id
    }
    fn set_id(&mut self, id: Uuid) {
        self._id=id;
    }
    fn receptiveness(&self) -> bool {
        self._receptive
    }
    fn set_receptiveness(&mut self, receptiveness: bool) {
        self._receptive = receptiveness;
    }
    fn position(&self) -> &Vec<Coordinates> {&self._position}

}

pub struct ObjectFactory {
}

impl ObjectFactory {
    ///Creates the first objects to be placed on the board
    pub fn firsts() -> Vec<Main> {
        let mut vec = Vec::new();
        vec.push(
            Main {
                    _id: Uuid::new_v4(),
                    _category: ObjectCategory::Main,
                    _receptive: true,
                    _position: vec!(Coordinates(5,5)),
                }
            );

        vec
    }
}