use game::gamestate::object::Object;

pub trait Update{
    fn update_objects(objs: Vec<Box<Object>>) -> Vec<Box<Object>>;
    fn periodic(periodic_objs: Vec<Box<Object>>) -> Vec<Box<Object>>;
    fn update_phase();
}

pub struct PacManUpdater {

}

impl Update for PacManUpdater {
    fn update_objects(objs: Vec<Box<Object>>) -> Vec<Box<Object>> {
        objs.iter().filter(|a: Box<Object>| {
            a.downcast();
        })
    }

    fn periodic(periodic_objs: Vec<Box<Object>>) -> Vec<Box<Object>> {
        Vec::new()
    }
    fn update_phase() {
        Vec::new()
    }

}