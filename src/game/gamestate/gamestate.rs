/* use std::collections::HashMap;

use game::gamestate::object::{ObjectFactory, ObjectCategory, Main, MainFactory};
use game::gamestate::object::Point;
use game::gamestate::object::Object;
use uuid::Uuid;

pub type CategoryMap<U> = HashMap<ObjectCategory, HashMap<Uuid, U>>;

pub trait CategoryMapNew {
    fn new() -> Self;
}

impl<U> CategoryMapNew for CategoryMap<U> {
    fn new() -> Self {
        let mut newhash: CategoryMap<U> =HashMap::new();
        for cat in ObjectCategory::categories() {
            newhash.insert(cat,HashMap::new());
        }

        newhash
    }
}

#[derive(Debug)]
pub struct GameState<U> {
    _objects: CategoryMap<U>,
    _end: bool,
}

impl<U> GameState<U> {
    pub fn new() -> GameState<U> {
        Default::default()
    }

    pub fn with_objects() -> GameState<U> {
        let mut first_objects: Vec<Box<Object>> = MainFactory::firsts();
        //Create the hashmap from the vector
        let mut hashmap: CategoryMap<U> = HashMap::new();

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

    pub fn all_objects(&self) -> Vec<&Main> {

        let mut objects=Vec::new();
        //iterate over each hashmap of different categories
        for maps in self._objects.values() {
            for object in maps.values() {
                objects.push(object);
            }
        }

        objects
    }
    pub fn all_movable(&self) -> Vec<&Main> {
        let mut objects=Vec::new();
        //iterate over each hashmap of different categories
        for maps in self._objects.values() {
            for object in maps.values() {
                if object.movable()==true {
                    objects.push(object);
                }
            }
        }

        objects
    }
}

impl<U> Default for GameState<U> {
    fn default() -> Self {
        GameState {
            _objects: CategoryMap::new(),
            _end: false
        }
    }
}
*/