use game::gamestate::object::Object;

pub trait Update{
    fn update_objects(objs: Vec<Box<Object>>) -> Vec<Box<Object>>;
    fn periodic(periodic_objs: Vec<Box<Object>>) -> Vec<Box<Object>>;
    fn update_phase();
}