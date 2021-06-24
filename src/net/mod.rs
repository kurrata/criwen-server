pub mod command;

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