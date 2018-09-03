use std::mem;


pub type Area = Vec<Tile>;

///A single tile
#[derive(Clone, Debug)]
pub enum Tile{
    HBorder(Option<char>),
    VBorder(Option<char>),
    Empty(Option<char>),
    Active(Option<char>),
    Blocking(Option<char>),
    NonBlocking(Option<char>),
    Teleport(Option<char>)
}

impl Tile {
    fn tiles() -> Vec<Tile> {//todo remove hardcoded
        let mut new_vec=Vec::new();
        new_vec.push(Tile::HBorder(None));
        new_vec.push(Tile::VBorder(None));
        new_vec.push(Tile::Empty(None));
        new_vec.push(Tile::Active(None));
        new_vec.push(Tile::Blocking(None));
        new_vec.push(Tile::NonBlocking(None));
        new_vec.push(Tile::Teleport(None));

        new_vec
    }
}
#[derive(Debug, Clone)]
pub struct Dimensions(pub u16,pub u16);

impl PartialEq for Dimensions{
    fn eq(&self, other: &Dimensions) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl Eq for Dimensions {}

#[derive(Debug, Clone)]
pub struct Coordinates(pub u16,pub u16);

impl PartialEq for Coordinates{
    fn eq(&self, other: &Coordinates) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl Eq for Coordinates {}

pub enum BoardError{
    WrongLen(String)
}
///T is either Vec\<Tile\> or RwLock\<Vec\<Tile\>\>

pub struct Board  {
    _tiles: Area,
    _dimensions: Dimensions
}

impl Board {

    pub fn new(dim: Dimensions) -> Board {
        let tiles = vec![Tile::Empty(None); dim.0 as usize * dim.1 as usize];
        Board {_tiles: tiles, _dimensions: dim}
    }

    pub fn with_tiles(tiles: Area, dim: Dimensions) -> Result<Board,BoardError> {
        if tiles.len()!=dim.0 as usize *dim.1 as usize {
            Err(BoardError::WrongLen("Wrong dimensions, tiles must be equal to dim_x * dim_y ".to_string()))
        } else {
            Ok(Board {_tiles: tiles, _dimensions: dim})
        }
    }

    ///Replaces all the current tiles with those passed as argument
    pub fn replace_tiles(&mut self, tiles: Area) {
        mem::replace(&mut self._tiles, tiles);
    }

    pub fn clean(&mut self) {
        let Dimensions(x,y) =  self._dimensions.clone();
        self._tiles = vec![Tile::Empty(None); x as usize * y as usize];
    }

    pub fn clean_with_border(&mut self) {
        self.clean();
        self.set_border();
    }
    pub fn set_border(&mut self) {

        let Dimensions(x,y): Dimensions = *self.dimensions();

        for (i,v) in self._tiles.iter_mut().enumerate() {
        if i/(x as usize)==0 /* first line */
            || i/(x as usize)==y as usize-1 /* last line */
            {
                *v=Tile::HBorder(Some('*'))
            }
        else if i%(x as usize)==0 /* first tile */
            || i%(x as usize)==x as usize-1 /* last tile */
            {
                *v=Tile::VBorder(Some('*'))
            }
        }
    }
    pub fn get_tile(&self, i: usize) -> &Tile {
        &self._tiles[i]
    }
    pub fn set_tile(&mut self, i: usize, tile: Tile) {
        self._tiles[i] = tile;
    }
    pub fn dimensions(&self) -> &Dimensions {
        &self._dimensions
    }
    pub fn set_dimensions(&mut self, dim: Dimensions) {
        self._dimensions = dim;
    }

    ///Returns the set of coordinates of a point position.
    pub fn as_coord(&self, point: u16) -> Coordinates {
        let dim_x=self.dimensions().0;
        Coordinates (point%(dim_x), point/(dim_x-1))
    }

    ///Returns the point position of a set of coordinates. Accepts only an immutable borrow to the
    /// coordinates to non take it ownership.
    pub fn as_point(&self, coord: &Coordinates) -> u16 {
        let dim_x=self.dimensions().0;
        coord.1*(dim_x)+coord.0
    }

}

impl Default for Board {
    fn default() -> Board {
        Board {
            _tiles: Vec::new(),
            _dimensions: Dimensions(0,0)
        }
    }
}

