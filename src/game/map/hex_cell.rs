use crate::game::map::{HexCell};

impl HexCell {
    pub fn new() -> HexCell {
        HexCell {}
    }

    // pub fn neighbors(&self) -> [Coordinate; 6] {
    //     let neighbors: [Coordinate; 6] = [
    //         Coordinate::new(self.position.x + 1, self.position.y), //east
    //         Coordinate::new(self.position.x + 1, self.position.y - 1), //north east
    //         Coordinate::new(self.position.x, self.position.y - 1), //north west
    //         Coordinate::new(self.position.x - 1, self.position.y), //west
    //         Coordinate::new(self.position.x - 1, self.position.y + 1), //south west
    //         Coordinate::new(self.position.x, self.position.y + 1), //south east
    //     ];
    //     neighbors
    // }
}