use game::gamestate::object::Object;
use game::gamestate::object::ObjectCategory;
use std::ops::Deref;
use game::gamestate::object::Snake;
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

        let wall_pos: Vec<Coordinates> = objs.iter().filter(|a| *(a.deref().category())==ObjectCategory::Wall)
            .flat_map(|a| {
                a.current_position().clone()
            }).collect();

        let mut mains: Vec<&mut Box<Object>> = objs.iter_mut().filter(|a| {
            *(a.deref().category())==ObjectCategory::Main && a.deref().next_position() != None
        }).collect();

        for m in mains {
            for p in m.next_position().unwrap().clone() {
                if wall_pos.contains(&p) {
                    m.reset_next_position()
                }
            }

        }
    }

}