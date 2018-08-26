use game::gamestate::object::Object;

pub trait Check{
    fn check_collisions(objs: Vec<Box<Object>>) -> Vec<Box<Object>>;
    fn objects() -> Vec<Box<Object>>;
}