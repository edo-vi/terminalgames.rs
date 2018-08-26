use game::gamestate::object::Object;

pub trait Update{
    fn update(objs:  Vec<Box<Object>>) -> Vec<Box<Object>>;
    fn periodic(periodic_objs: Vec<Box<Object>>) -> Vec<Box<Object>>;
}