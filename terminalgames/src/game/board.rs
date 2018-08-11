use std::ops::DerefMut;
use super::{RwLock, RwLockWriteGuard, LockResult};
use std::mem;

pub type Area = Vec<Tile>;
pub type LockedArea = RwLock<Area>;

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

pub enum BoardError{
    WrongLen(String)
}
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
        let dim_x=self.dimensions().0;
        Coordinates (point/(dim_x+1), point%(dim_x+1))
    }

    ///Returns the point position of a set of coordinates. Accepts only an immutable borrow to the
    /// coordinates to non take it ownership.
    pub fn as_point(&self, coord: &Coordinates) -> u16 {
        let dim_x=self.dimensions().0;
        coord.0*(dim_x+1)+coord.1
    }
}

impl Board<LockedArea> {

    pub fn new(dim: Dimensions) -> Board<LockedArea> {
        let tiles = RwLock::new(vec![Tile::New(None);(dim.0 as usize * dim.1 as usize)]);
        Board {_tiles: tiles, _dimensions: dim}
    }

    pub fn with_tiles(tiles: Area, dim: Dimensions) -> Result<Board<LockedArea>,BoardError> {
        if tiles.len()!=(dim.0 as usize *dim.1 as usize) {
            Err(BoardError::WrongLen("Wrong dimensions, tiles must be equal to dim_x * dim_y ".to_string()))
        } else {
            Ok(Board {_tiles: RwLock::new(tiles), _dimensions: dim})
        }
    }

    fn _get_guard(&self) -> RwLockWriteGuard<Area> {
        // Attempt to get the lock over the board tiles
        let result: LockResult<RwLockWriteGuard<Area>> = self._tiles.write();

        match result {
            //We got the non-poisoned lock, so we return it alongside
            Ok(guard) => guard,
            Err(_) => panic!("The lock over the boar tiles was poisoned!")
        }

    }
    ///Replaces all the current tiles with those passed as argument
    pub fn replace_tiles(&self, tiles: Area) {
        //We use deref_mut to get &mut T with RwLockWriteGuard<T>; if the lock is poisoned, this
        //call will panic
        mem::replace(self._get_guard().deref_mut(), tiles);
    }

    pub fn set_border(&mut self) {
        //Extends guard lifetime until this function returns
        let mut guard = self._get_guard();

        let dim: &Dimensions = self.dimensions();

        let mut mutable_to_tile: &mut Area = guard.deref_mut();
        for (i, v) in mutable_to_tile.into_iter().enumerate() {
            if i/((*dim).0 as usize)==0 /* first line */
            || i/((*dim).0 as usize)==dim.1 as usize-1 /* last line */
            || i%((*dim).0 as usize)==0 /* first tile */
            || i%((*dim).0 as usize)==dim.1 as usize /* last tile */
                    {
                *v=Tile::Border(Some('*')) //todo
            }
        }
    }
}

impl Default for Board<LockedArea> {
    fn default() -> Board<LockedArea> {
        Board {
            _tiles: RwLock::new(Vec::new()),
            _dimensions: Dimensions(0,0)
        }
    }
}

