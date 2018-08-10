pub mod game;

pub mod interface;

#[cfg(test)]
mod test {
    use super::game::board::{Board,Dimensions,Coordinates};

    #[test]
    fn test_conversion() {
        let board: Board<u8> = Board::new(8,Dimensions(4,3));
        assert_eq!(board.convert_to_line(Coordinates(0,0)),0);
        assert_eq!(board.convert_to_line(Coordinates(3,4)),19);
        assert_eq!(board.convert_to_line(Coordinates(2,2)),12);
        assert_eq!(board.convert_to_line(Coordinates(0,4)),4);
        assert_eq!(board.convert_to_line(Coordinates(2,0)),10);
        assert_eq!(board.convert_to_plane(19),Coordinates(3,4));
        assert_eq!(board.convert_to_plane(0),Coordinates(0,0));
        assert_eq!(board.convert_to_plane(13),Coordinates(2,3));
    }
}


