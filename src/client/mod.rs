use std::io::prelude::*;
use std::net::TcpStream;
use bytes::{BytesMut, Bytes};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6379")?;
    let mut buf = BytesMut::with_capacity(1024);
    buf = BytesMut::from("qqqwqqqertyuiopqwertyuiop");
    stream.write(&buf)?;
    // stream.read(&mut [0; 128])?;
    Ok(())
}