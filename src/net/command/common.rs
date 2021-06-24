use crate::net::Command;

#[derive(Serialize, Deserialize)]
pub struct Ping {
    command: u16,
}

impl Ping {
    pub fn new() -> Ping {
        Ping { command: Command::Ping.value() }
    }
}