use game::gamestate::object::Object;
use interface::input::PlayerInput;
use std::ops::Deref;
use std::borrow::BorrowMut;

pub trait Update: Default {
    fn new() -> Self;
    fn update_objects(&self, objs: Vec<Box<Object>>, input: PlayerInput) -> Vec<Box<Object>>;
    fn periodic(&self, periodic_objs: Vec<Box<Object>>) -> Vec<Box<Object>>;
    fn update_phase();
}

#[derive(Default)]
pub struct PacManUpdater {

}
impl Update for PacManUpdater {
    fn new() -> Self {
        PacManUpdater {}
    }
    fn update_objects(&self, objs: Vec<Box<Object>>, input: PlayerInput) -> Vec<Box<Object>> {
        objs.into_iter().filter(|a| a.deref().movable()).map(|mut a| {
            a.handle_input(&input);
            a
        }).collect()
    }

    fn periodic(&self, periodic_objs: Vec<Box<Object>>) -> Vec<Box<Object>> {
        Vec::new()
    }
    fn update_phase() {

    }

}