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
        Self::check_collision(objs);
        Self::check_powerup(objs);
    }
}

impl PacManChecker {
    pub fn check_collision(objs: &mut Vec<Box<Object>>) {

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
    //I know it's coded shittly, I just needed to get around the double borrow of objs in mut snakes
    // and the iter().enumerate
    pub fn check_powerup(objs: &mut Vec<Box<Object>>) {
        let powerup_pos: Vec<Coordinates> = objs.iter().filter(|a| *(a.deref().category())==ObjectCategory::PowerUp)
            .flat_map(|a| {
                a.current_position().clone()
            }).collect();

        let mut is_found = false;

        {
            let mut snakes: Vec<&mut Box<Object>> = objs.iter_mut().filter(|a| {
                *(a.deref().category())==ObjectCategory::Main && a.deref().next_position() != None
            }).collect();

            //The object has already updated its next_position containing the new tile at the end:
            // we remove it only if it hasn't it a powerup
            for s in snakes {
                if !powerup_pos.contains(&s.next_position().unwrap()[0]) {
                    let mut pos = s.next_position().unwrap().clone();
                    let len= pos.len()-1;
                    pos.remove(len);
                    s.set_next_position(Some(&pos));
                } else {
                    is_found=true;
                }
            }

        }
        if is_found==true {
            //removes the powerup
            let mut index = 0;
            for (i,o) in objs.iter().enumerate() {
                if *o.category()==ObjectCategory::PowerUp {
                    index = i;
                }
            }
            objs.remove(index);
        }

    }



}