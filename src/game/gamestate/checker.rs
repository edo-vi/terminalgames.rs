use game::gamestate::object::Object;

pub trait Check{
    fn new() -> Self;
    fn check_collisions(objs: Vec<Box<Object>>) -> Vec<Box<Object>>;
    fn objects() -> Vec<Box<Object>>;
}

pub struct PacManChecker {}

impl Check for PacManChecker {
    fn new() -> Self {
        PacManChecker {}
    }
    fn check_collisions(objs: Vec<Box<Object>>) -> Vec<Box<Object>> {
        Vec::new()
    }
    fn objects() -> Vec<Box<Object>> {
        Vec::new()
    }
}