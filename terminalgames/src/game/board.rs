use super::RwLock;
use std::mem;
///A single tile
#[derive(Clone)]
pub enum Tile{
    Border(Option<char>),
    New(Option<char>),
    Active(Option<char>),
    Blocking(Option<char>),
    NonBlocking(Option<char>)
}

#[derive(Debug)]
pub struct Dimensions(pub u16,pub u16);

impl PartialEq for Dimensions{
    fn eq(&self, other: &Dimensions) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl Eq for Dimensions {}

#[derive(Debug)]
pub struct Coordinates(pub u16,pub u16);

impl PartialEq for Coordinates{
    fn eq(&self, other: &Coordinates) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl Eq for Coordinates {}

///T is either Vec\<Tile\> or RwLock\<Vec\<Tile\>\>
pub struct Board<T>  {
    _tiles: T,
    _dimensions: Dimensions
}

impl<T> Board<T> {

    pub fn dimensions(&self) -> &Dimensions {
        &self._dimensions
    }
    pub fn set_dimensions(&mut self, dim: Dimensions) {
        self._dimensions = dim;
    }

    ///Returns the set of coordinates of a point position.
    pub fn as_coord(&self, point: u16) -> Coordinates {
        Coordinates (point/(self._dimensions.0+1), point%(self._dimensions.0+1))
    }

    ///Returns the point position of a set of coordinates. Accepts only an immutable borrow to the
    /// coordinates to non take it ownership.
    pub fn as_point(&self, coord: &Coordinates) -> u16 {
        coord.0*(self._dimensions.0+1)+coord.1
    }


}

impl Board<RwLock<Vec<Tile>>> {

    pub fn new(dim: Dimensions) -> Board<RwLock<Vec<Tile>>> {
        let tiles = RwLock::new(vec![Tile::New(None);dim.0 as usize * dim.1 as usize]);
        Board {_tiles: tiles, _dimensions: dim}
    }
    pub fn set_tiles(&mut self, tiles: Vec<Tile>) {
        mem::replace(&mut self._tiles, RwLock::new(tiles));
    }
}


