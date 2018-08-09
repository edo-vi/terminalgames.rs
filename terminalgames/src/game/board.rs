use super::RwLock;

pub enum Tile{
    Border(Option<char>),
    Active(Option<char>),
    Blocking(Option<char>),
    NonBlocking(Option<char>)
}

struct Dimensions(u16,u16);
pub struct Coordinates(u16,u16);

///T is either Vec\<Tile\> or RwLock\<Vec\<Tile\>\>
pub struct Board<T> {
    tiles: T,
    dimensions: Dimensions
}

impl<T> Board<T> {
    pub fn convert_to_plane(&self, pos: u16) -> Coordinates {
        Coordinates (pos/self.dimensions.0,pos%self.dimensions.0)
    }

    pub fn convert_to_line(&self, pos: Coordinates) -> u16 {
        pos.0*(self.dimensions.0+1)+pos.1
    }
}


