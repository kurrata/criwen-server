extern crate config;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use log;
use simple_logger::SimpleLogger;
use clap::{App, SubCommand, Arg};
use crate::server::GameServer;
use log::{LevelFilter};
use crate::core::settings::Settings;
use crate::client::ui;

mod net;
mod server;
mod core;
mod client;


#[tokio::main]
async fn main() {
    let config = Settings::new().await;
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
        ("start", _) => { start_server().await }
        ("stop", _) => {}
        ("restart", _) => {}
        ("ui", _) => { start_ui().await }
        _ => {
            app.print_help().unwrap();
        }
    }
}

async fn init_logging(level: u64) {
    let logger = SimpleLogger::new();
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

async fn start_ui() {
    log::info!("UI starting");
    ui().await;
    log::info!("UI shut down");
}