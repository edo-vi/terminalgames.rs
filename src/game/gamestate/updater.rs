use game::gamestate::object::Object;
use interface::input::PlayerInput;
use std::ops::DerefMut;
use std::borrow::BorrowMut;

pub trait Update: Default {
    fn new() -> Self;
    fn update_objects(&self, objs: &mut Vec<Box<Object>>, input: PlayerInput);
    fn periodic(&self, objs: &mut Vec<Box<Object>>);
    fn update_phase();
}

#[derive(Default)]
pub struct PacManUpdater {

}
impl Update for PacManUpdater {
    fn new() -> Self {
        PacManUpdater {}
    }
    fn update_objects(& self, objs: &mut Vec<Box<Object>>, input: PlayerInput)  {
        for obj in objs {
            obj.deref_mut().handle_input(&input)
        }
    }

    fn periodic(&self, periodic_objs: &mut Vec<Box<Object>>) {

    }
    fn update_phase() {

    }

}