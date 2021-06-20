use std::sync::{Arc};
use dashmap::DashMap;
use crate::core::map::{WorldMap, Coordinate, HexCell};

pub struct GameState {
    pub map: Arc<DashMap<Coordinate, HexCell>>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            map: Arc::new(DashMap::new()),
        }
    }

    pub async fn init(&self) {
        WorldMap::grow(self.map.clone(), 100).await;
    }
}