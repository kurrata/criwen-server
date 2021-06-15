mod hex_cell;
mod coordinate;
mod world_map;

pub struct HexCell {
}

#[derive(Hash)]
#[derive(Eq)]
#[derive(PartialEq)]
pub struct Coordinate {
    pub x: i16,
    pub y: i16,
}

pub struct WorldMap {}