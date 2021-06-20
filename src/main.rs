use log;
use simple_logger::SimpleLogger;
use crate::game::{Criwen};

mod game;
mod net;


#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();
    log::info!("Server starting");
    let criwen = Criwen::new();
    Criwen::init(criwen).await;
    log::info!("Server shut down");
}