use crate::game::map::Coordinate;

impl Coordinate {
    pub fn new(x: i16, y: i16) -> Coordinate {
        Coordinate { x, y}
    }
}