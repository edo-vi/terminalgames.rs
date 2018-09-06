use game::gamestate::object::Object;
use game::gamestate::object::ObjectCategory;
use std::ops::Deref;
use game::gamestate::object::Snake;
use game::board::Coordinates;
use game::gamestate::object::PowerUpFactory;
use game::gamestate::StatePhase;
use std::process;
use super::super::pancurses::{initscr, endwin};

pub trait Check: Default{
    fn new() -> Self;
    fn checks(&self, objs: &mut Vec<Box<Object>>, phase: &mut StatePhase);
}

#[derive(Default)]
pub struct SnakeChecker {}


impl Check for SnakeChecker {
    fn new() -> Self {
        SnakeChecker {}
    }
    fn checks(&self, objs: &mut Vec<Box<Object>>, phase: &mut StatePhase) {
        Self::check_collision(objs);
        Self::check_powerup(objs, phase);
    }
}

impl SnakeChecker {
    pub fn check_collision(objs: &mut Vec<Box<Object>>) {

        let wall_pos: Vec<Coordinates> = objs.iter().filter(|a| *(a.deref().category())==ObjectCategory::Wall)
            .flat_map(|a| {
                a.current_position().clone()
            }).collect();
        //with no head
        let main_pos: Vec<Coordinates> = objs.iter().filter(|a| *(a.deref().category())==ObjectCategory::Main)
            .flat_map(|a| {
                let mut posit = a.current_position().clone();
                posit.remove(0);
                posit
            }).collect();

        let mut mains: Vec<&mut Box<Object>> = objs.iter_mut().filter(|a| {
            *(a.deref().category())==ObjectCategory::Main && a.deref().next_position() != None
        }).collect();

        for m in mains {

            if wall_pos.contains(&m.next_position().unwrap().clone()[0])
                || main_pos.contains(&m.next_position().unwrap().clone()[0]){
                initscr();
                endwin();
                println!("\n          ////////////////////////////\n          ///     Hai perso! 👎    ///\n          ////////////////////////////\n");
                println!("Snake 🐍 by Edoardo Zorzi, 06/09/2018");
                process::exit(1);
            }

        }
    }
    //I know it's coded shittly, I just needed to get around the double borrow of objs in mut snakes
    // and the iter().enumerate
    pub fn check_powerup(objs: &mut Vec<Box<Object>>, phase: &mut StatePhase) {
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
            //change phase to create
            match phase {
                _ => *phase = StatePhase::Create
            }
        }

    }


}