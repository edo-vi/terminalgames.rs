#![feature(rust_2018_preview)]
extern crate core;
extern crate uuid;

pub mod game;

pub mod interface;

#[cfg(test)]
mod test {
    use super::game::board::{Board,Dimensions,Coordinates};

    #[test]
    fn test_conversion() {
        let board: Board = Board::new(Dimensions(4,3));

        assert_eq!(board.as_point(&Coordinates(0, 0)), 0);
        assert_eq!(board.as_point(&Coordinates(3, 4)), 19);
        assert_eq!(board.as_point(&Coordinates(2, 2)), 12);
        assert_eq!(board.as_point(&Coordinates(0, 4)), 4);
        assert_eq!(board.as_point(&Coordinates(2, 0)), 10);
        assert_eq!(board.as_coord(19), Coordinates(3, 4));
        assert_eq!(board.as_coord(0), Coordinates(0, 0));
        assert_eq!(board.as_coord(13), Coordinates(2, 3));
    }
}


