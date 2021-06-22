extern crate config;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use log;
use simple_logger::SimpleLogger;
use clap::{App, SubCommand, ArgMatches, Arg};
use crate::server::GameServer;
use log::{Level, LevelFilter};
use crate::core::settings::Settings;

mod net;
mod server;
mod core;


#[tokio::main]
async fn main() {
    let config = Settings::new();
    let mut app = App::new("Criwen")
        .args(&[
            Arg::with_name("v").short("v").multiple(true).help("Sets the level of verbosity\
                \n\t0 - Default\
                \n\t1 - Error\
                \n\t2 - Warn\
                \n\t3 - Info\
                \n\t4 - Debug\
                \n\t5 - Trace\
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

    if app.clone().get_matches().occurrences_of("v") == 0 {
        init_logging(config.get::<u64>("log_level").unwrap()).await;
    } else {
        init_logging(app.clone().get_matches().occurrences_of("v")).await;
    }

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
        0 => { logger.with_level(LevelFilter::Off) }
        1 => { logger.with_level(LevelFilter::Error) }
        2 => { logger.with_level(LevelFilter::Warn) }
        3 => { logger.with_level(LevelFilter::Info) }
        4 => { logger.with_level(LevelFilter::Debug) }
        _ => { logger.with_level(LevelFilter::Trace) }
    }.init().unwrap();
}

async fn start_server() {
    log::info!("Server starting");
    let server = GameServer::new();
    GameServer::init(server).await;
    log::info!("Server shut down");
}