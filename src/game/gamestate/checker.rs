use game::gamestate::object::Object;

pub trait Check: Default{
    fn new() -> Self;
    fn checks(&self, objs: &mut Vec<Box<Object>>);
}

#[derive(Default)]
pub struct PacManChecker {}


impl Check for PacManChecker {
    fn new() -> Self {
        PacManChecker {}
    }
    fn checks(&self, objs: &mut Vec<Box<Object>>) {

    }
}