use game::gamestate::object::Object;
use game::gamestate::object::ObjectCategory;
use std::ops::Deref;
use game::gamestate::object::Main;
use game::board::Coordinates;

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
        self.check_collision(objs);
    }
}

impl PacManChecker {
    pub fn check_collision(&self, objs: &mut Vec<Box<Object>>) {
        let main_pos: Vec<Coordinates> = objs.iter().filter(|a| *(a.deref().category())==ObjectCategory::Main)
            .flat_map(|a| {
                match a.next_position() {
                    None => Vec::new(),
                    Some(pos) => pos.clone()
                }
            }).collect();
        let wall_pos: Vec<Coordinates> = objs.iter().filter(|a| *(a.deref().category())==ObjectCategory::Wall)
            .flat_map(|a| {
                info!("{:?}", a.current_position());
                a.current_position().clone()
            }).collect();

        let mut mains: Vec<&mut Box<Object>> = objs.iter_mut().filter(|a| *(a.deref().category())==ObjectCategory::Main).collect();

        let mut main=&mut mains[0];

        for mainpos in main_pos {
            if wall_pos.contains(&mainpos) {
                main.reset_next_position();
            }
        }
    }

}