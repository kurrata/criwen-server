use log;
use simple_logger::SimpleLogger;
use clap::{App, SubCommand, ArgMatches};
use crate::server::GameServer;

mod net;
mod server;
mod core;


#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();
    let mut app = App::new("Criwen")
        .about("Criwen game server")
        .subcommand(SubCommand::with_name("start")
            .about("Start game server")
        )
        .subcommand(SubCommand::with_name("stop")
            .about("Stop game server")
        )
        .subcommand(SubCommand::with_name("restart")
            .about("Restart game server")
        )
        .subcommand(SubCommand::with_name("ui")
            .about("Run administration UI")
        );


    match app.clone().get_matches().subcommand().clone() {
        ("start", Some(sub_app)) => { start_server().await }
        ("stop", Some(sub_app)) => {}
        ("restart", Some(sub_app)) => {}
        ("ui", Some(sub_app)) => {}
        _ => {
            app.print_help().unwrap();
        }
    }
}

async fn start_server() {
    log::info!("Server starting");
    let server = GameServer::new();
    GameServer::init(server).await;
    log::info!("Server shut down");
}