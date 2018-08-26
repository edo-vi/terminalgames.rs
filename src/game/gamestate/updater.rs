use game::gamestate::object::Object;

pub trait Update{
    fn update(objs: Vec<Box<Object>>) -> Vec<Box<Object>>;
    fn periodic(objs: Vec<Box<Object>>) -> Vec<Box<Object>>;
}