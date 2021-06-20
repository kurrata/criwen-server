use crate::game::{Criwen, GameState};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;

impl Criwen {
    pub fn new() -> Arc<Criwen> {
        Arc::new(Criwen {
            game_state: Arc::new(GameState::new()),
            shut_down_in_progress: Arc::new(Mutex::new(false)),
            threads: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub async fn init(game: Arc<Criwen>) {
        game.game_state.init().await;
        Criwen::start_local_socket(game.clone()).await;

        for thread in &mut *game.threads.lock().unwrap() {
            thread.await.unwrap();
        }
    }

    async fn start_local_socket(game: Arc<Criwen>) {
        let lock = game.shut_down_in_progress.clone();
        let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
        game.threads.clone().lock().unwrap().push(tokio::spawn(async move {
            log::debug!("Start: Local socket");
            while *lock.lock().unwrap() == false {
                let (mut socket, _) = listener.accept().await.unwrap();
            }
            log::debug!("Done: Local socket");
        }));
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    async fn start_public_socket(game: Arc<Criwen>) {}
}