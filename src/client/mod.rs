use std::io::prelude::*;
use std::net::TcpStream;
use crate::net::command::common::Ping;
use crate::core::settings::Settings;


pub async fn ui() {
    let address = [
        "127.0.0.1".to_string(),
        Settings::new().await.get::<u64>("server.local_port").unwrap().to_string()
    ].join(":");
    let mut stream = TcpStream::connect(address).unwrap();
    let command = Ping::new();
    stream.write(serde_json::ser::to_string(&command).unwrap().as_bytes()).unwrap();
}