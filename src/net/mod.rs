pub enum Command {
    Ping,
}

impl Command {
    fn value(&self) -> u16 {
        match *self {
            //Private
            //Common
            Command::Ping => 10000,
            //Public
        }
    }
}

pub struct Payload {
    command: u16,
    data: String,
}

impl Payload {
    pub fn new(command: u16, data: String) -> Payload {
        Payload { command, data }
    }
}