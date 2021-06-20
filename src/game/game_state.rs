use crate::game::GameState;
use std::sync::{Arc};
use dashmap::DashMap;
use crate::game::map::WorldMap;

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