use game::gamestate::object::Object;
use interface::input::PlayerInput;
use std::ops::Deref;
use std::borrow::BorrowMut;
pub trait Update{
    fn new() -> Self;
    fn update_objects(objs: Vec<Box<Object>>, input: PlayerInput) -> Vec<Box<Object>>;
    fn periodic(periodic_objs: Vec<Box<Object>>) -> Vec<Box<Object>>;
    fn update_phase();
}

pub struct PacManUpdater {

}

impl Update for PacManUpdater {
    fn new() -> Self {
        PacManUpdater {}
    }
    fn update_objects(objs: Vec<Box<Object>>, input: PlayerInput) -> Vec<Box<Object>> {
        objs.iter().filter(|a| a.deref().movable()).borrow_mut().map(|mut a| a.handle_input(&input)).collect()
    }

    fn periodic(periodic_objs: Vec<Box<Object>>) -> Vec<Box<Object>> {
        Vec::new()
    }
    fn update_phase() {
        Vec::new()
    }

}