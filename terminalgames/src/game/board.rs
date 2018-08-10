use super::RwLock;

pub enum Tile{
    Border(Option<char>),
    Active(Option<char>),
    Blocking(Option<char>),
    NonBlocking(Option<char>)
}

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
    pub fn convert_to_plane(&self, pos: u16) -> Coordinates {
        Coordinates (pos/(self._dimensions.0+1), pos%(self._dimensions.0+1))
    }

    pub fn convert_to_line(&self, pos: Coordinates) -> u16 {
        pos.0*(self._dimensions.0+1)+pos.1
    }
}


