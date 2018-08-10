use super::RwLock;

pub enum Tile{
    Border(Option<char>),
    Active(Option<char>),
    Blocking(Option<char>),
    NonBlocking(Option<char>)
}

#[derive(Debug)]
pub struct Dimensions(pub u16,pub u16);

#[derive(Debug)]
pub struct Coordinates(pub u16,pub u16);

impl PartialEq for Coordinates{
    fn eq(&self, other: &Coordinates) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}


///T is either Vec\<Tile\> or RwLock\<Vec\<Tile\>\>
pub struct Board<T>  {
    _tiles: T,
    _dimensions: Dimensions
}

impl<T> Board<T> {
    pub fn new(tiles: T, dim: Dimensions) -> Board<T> {
        Board {_tiles: tiles, _dimensions: dim}
    }
    pub fn set_dimensions(&mut self, dim: Dimensions) {
        self._dimensions = dim;
    }

    pub fn dimensions(&self) -> &Dimensions {
        &self._dimensions
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


