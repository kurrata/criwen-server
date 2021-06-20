use log;
use simple_logger::SimpleLogger;
use clap::{App, SubCommand, ArgMatches, Arg};
use crate::server::GameServer;
use log::{Level, LevelFilter};

mod net;
mod server;
mod core;


#[tokio::main]
async fn main() {
    let mut app = App::new("Criwen")
        .args(&[
            Arg::with_name("v").short("v").multiple(true).help("Sets the level of verbosity\
                \n\t0 - Error\
                \n\t1 - Warn\
                \n\t2 - Info\
                \n\t3 - Debug\
                \n\t4 - Trace\
                "),
        ])
        .subcommand(SubCommand::with_name("start")
            .about("Start server")
        )
        .subcommand(SubCommand::with_name("stop")
            .about("Stop server")
        )
        .subcommand(SubCommand::with_name("restart")
            .about("Restart server")
        )
        .subcommand(SubCommand::with_name("ui")
            .about("Run administration UI")
        );

    init_logging(app.clone().get_matches().occurrences_of("v")).await;

    match app.clone().get_matches().subcommand() {
        ("start", Some(sub_app)) => { start_server().await }
        ("stop", Some(sub_app)) => {}
        ("restart", Some(sub_app)) => {}
        ("ui", Some(sub_app)) => {}
        (a, b) => {
            app.print_help().unwrap();
        }
    }
}

async fn init_logging(level: u64) {
    let mut logger = SimpleLogger::new();
    match level {
        0 => { logger.with_level(LevelFilter::Error) }
        1 => { logger.with_level(LevelFilter::Warn) }
        2 => { logger.with_level(LevelFilter::Info) }
        3 => { logger.with_level(LevelFilter::Debug) }
        _ => { logger.with_level(LevelFilter::Trace) }
    }.init().unwrap();
}

async fn start_server() {
    log::info!("Server starting");
    let server = GameServer::new();
    GameServer::init(server).await;
    log::info!("Server shut down");
}