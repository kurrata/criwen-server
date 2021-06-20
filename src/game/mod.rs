use std::sync::{Arc,  Mutex};
use dashmap::DashMap;
use crate::game::map::{Coordinate, HexCell};
use tokio::task::JoinHandle;

pub mod map;
pub mod game_state;
pub mod criwen;

pub struct Criwen {
    pub shut_down_in_progress: Arc<Mutex<bool>>,
    pub game_state: Arc<GameState>,
    pub threads: Arc<Mutex<Vec<JoinHandle<()>>>>
}

pub struct GameState {
    pub map: Arc<DashMap<Coordinate, HexCell>>,
}

