
use game::gamestate::object::Object;
use interface::input::PlayerInput;
use std::ops::DerefMut;
use std::borrow::BorrowMut;

pub trait Update: Default {
    fn new() -> Self;
    fn update_objects(&self, objs: &mut Vec<Box<Object>>, input: PlayerInput);
    fn update_phase();
}

#[derive(Default)]
pub struct SnakeUpdater {

}
impl Update for SnakeUpdater {
    fn new() -> Self {
        SnakeUpdater {}
    }
    fn update_objects(& self, objs: &mut Vec<Box<Object>>, input: PlayerInput)  {
        for obj in objs {
            obj.deref_mut().handle_input(&input)
        }
    }

    fn update_phase() {

    }

}
