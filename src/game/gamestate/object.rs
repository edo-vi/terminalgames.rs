use interface::input::PlayerInput;
use uuid::Uuid;
use game::board::Coordinates;

pub type Point = Coordinates;
pub type Coords = Vec<Coordinates>;

#[derive(PartialEq, Eq, Hash, Clone)]
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

pub trait Active {
    fn handle_input(&mut self, input: &PlayerInput);
    fn category(&self) -> &ObjectCategory;
    fn set_category(&mut self, category: ObjectCategory);
    fn id(&self) -> &Uuid;
    fn set_id(&mut self, id: Uuid);
}

#[derive(Clone)]
pub struct Object<T> {
    _id : Uuid,
    _category: ObjectCategory,
    _position: T
}

impl<T> Active for Object<T> {
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

}

pub struct ObjectFactory<T> {
    _type: T
}

impl ObjectFactory<Point> {
    ///Creates the first objects to be placed on the board
    pub fn firsts() -> Vec<Object<Coordinates>> {
        let mut vec = Vec::new();
        vec.push(
            Object {
                    _id: Uuid::new_v4(),
                    _category: ObjectCategory::Main,
                    _position: Coordinates(5,5)
                }
            );

        vec
    }
}