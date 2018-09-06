#![feature(specialization)]
#![feature(duration_as_u128)]
#![feature(type_ascription)]

//logging dependencies
#[macro_use]
extern crate log;
extern crate fern;

extern crate core;
extern crate uuid;

pub mod game;

pub mod interface;

#[cfg(test)]
mod test {
    use super::game::board::{Board,Dimensions,Coordinates};

    #[test]
    fn test_conversion() {
        let board: Board = Board::new(Dimensions(6,5));

        assert_eq!(board.as_point(&Coordinates(0, 0)), 0);
        assert_eq!(board.as_point(&Coordinates(3, 1)), 9);
        assert_eq!(board.as_point(&Coordinates(2, 2)), 14);
        assert_eq!(board.as_point(&Coordinates(0, 4)), 24);
        assert_eq!(board.as_point(&Coordinates(2, 0)), 2);
        assert_eq!(board.as_coord(19), Coordinates(1, 3));
        assert_eq!(board.as_coord(0), Coordinates(0, 0));
        assert_eq!(board.as_coord(13), Coordinates(1, 2));
    }
}


