use std::io::prelude::*;
use std::net::TcpStream;
use crate::net::command::common::Ping;


pub async fn ui() {
    let mut stream = TcpStream::connect("127.0.0.1:6379").unwrap();
    let command = Ping::new();
    stream.write(serde_json::ser::to_string(&command).unwrap().as_bytes()).unwrap();
}