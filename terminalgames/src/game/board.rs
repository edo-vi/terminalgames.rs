enum Tile{
    Border(char),
    Active(char),
    Blocking(char),
    NonBlocking(char)
}
struct Dimensions(u8,u8);

pub struct Board {
    tiles: Vec<Tile>,
    dimensions: Dimensions
}