use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;
use log::*;
use simple_logger::SimpleLogger;
use dashmap::DashMap;
use crate::game::GameState;
use crate::game::map::{HexCell, Coordinate, WorldMap};

mod game;







#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();
    log::info!("Server starting");
    let my_game = GameState::new();
    WorldMap::grow(my_game.map, 1000).await;
    log::info!("Server shut down");
}