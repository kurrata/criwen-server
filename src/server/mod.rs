use std::sync::{Arc, Mutex};
use crate::server::game_state::GameState;
use tokio::task::JoinHandle;
use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;

mod game_state;

pub struct GameServer {
    pub shut_down_in_progress: Arc<Mutex<bool>>,
    pub game_state: Arc<GameState>,
    pub threads: Arc<Mutex<Vec<JoinHandle<()>>>>,
}

impl GameServer {
    pub fn new() -> Arc<GameServer> {
        Arc::new(GameServer {
            game_state: Arc::new(GameState::new()),
            shut_down_in_progress: Arc::new(Mutex::new(false)),
            threads: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub async fn init(game: Arc<GameServer>) {
        game.game_state.init().await;
        GameServer::start_local_socket(game.clone()).await;

        for thread in &mut *game.threads.lock().unwrap() {
            thread.await.unwrap();
        }
    }

    async fn start_local_socket(game: Arc<GameServer>) {
        let lock = game.shut_down_in_progress.clone();
        let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
        game.threads.clone().lock().unwrap().push(tokio::spawn(async move {
            log::debug!("Start: Local socket");
            while *lock.lock().unwrap() == false {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut buffer = Vec::new();
                socket.read_to_end(&mut buffer).await.unwrap();
                log::debug!("{}", String::from_utf8(buffer).unwrap());
            }
            log::debug!("Done: Local socket");
        }));
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    async fn start_public_socket(game: Arc<GameServer>) {}
}