use interface::input::PlayerInput;
use uuid::Uuid;

#[derive(PartialEq, Eq, Hash)]
pub enum ActiveCategory {
    Main,
    Enemy
}

pub trait Active {
    fn handle_input(&mut self, input: &PlayerInput);
    fn category(&self) -> ActiveCategory;
    fn set_category(&mut self, category: ActiveCategory);
    fn id(&self) -> &Uuid;
    fn set_id(&mut self, id: Uuid);
}

pub struct ActiveObject<T> {
    _id : Uuid,
    _category: ActiveCategory,
    _position: T
}