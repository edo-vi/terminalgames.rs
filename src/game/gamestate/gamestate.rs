use std::collections::HashMap;
use game::gamestate::CategoryMap;
use game::gamestate::object::{ObjectFactory, ObjectCategory, Object};
use game::gamestate::object::Point;
use game::gamestate::object::Active;


pub struct GameState<T> {
    _objects: CategoryMap<T>,
    _end: bool,
}

impl<T> GameState<T> {
    pub fn new() -> GameState<T> {
        Default::default()
    }
    pub fn all_objects(&self) -> Vec<&Object<T>> {
        let mut objects=Vec::new();
        //iterate over each hashmap of different categories
        for maps in self._objects.values() {
            for object in maps.values() {
                objects.push(object);
            }
        }

        objects
    }
}

impl GameState<Point> {
    pub fn with_objects() -> GameState<Point> {
        let mut first_objects: Vec<Object<Point>> = ObjectFactory::<Point>::firsts();
        //Create the hashmap from the vector
        let mut hashmap: CategoryMap<Point> = HashMap::new();

        for category in ObjectCategory::categories() {
            let mut new_hash = HashMap::new();

            let mut i=0;

            while i<first_objects.len() {
                if *first_objects[i].category()==category {
                    let object = first_objects.remove(i);
                    new_hash.insert(*object.id(), object);
                } else { i+=1; }
            }
            hashmap.insert(category,new_hash);
        }
        GameState {
            _objects: hashmap,
            _end: false
        }
    }
}

impl<T> Default for GameState<T> {
    fn default() -> Self {
        GameState {
            _objects: CategoryMap::new(),
            _end: false
        }
    }
}
