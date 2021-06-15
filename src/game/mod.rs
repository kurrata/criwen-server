use std::sync::Arc;
use dashmap::DashMap;
use crate::game::map::{Coordinate, HexCell};

pub mod map;
pub mod game_state;

pub struct GameState {
    pub map: Arc<DashMap<Coordinate, HexCell>>,
}

