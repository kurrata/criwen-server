use crate::game::GameState;
use std::sync::Arc;
use dashmap::DashMap;

impl GameState {
    pub fn new() -> GameState {
        GameState {
            map: Arc::new(DashMap::new()),
        }
    }
}