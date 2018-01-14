#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
extern crate binance;
extern crate chrono;
#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};
use binance::api::Binance as BinanceClient;
use binance::general::General as BinanceGeneral;
use chrono::NaiveDateTime;

fn main() {
    let matches = App::new("Binance-cli")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("ping")
                .about("Responds \"pong\" on successful response from Binance ping API."),
        )
        .subcommand(
            SubCommand::with_name("servertime")
                .alias("time")
                .about("Attempts to retrieve server time (Unix Timestamp) from Binance API. EX: 1515958015582")
                .arg(
                    Arg::with_name("human")
                        .short("-H")
                        .long("--human")
                        .alias("datetime")
                        .help("Displays server time in datetime format. EX: 2018-01-14 11:24:26"),
                ),
        )
        .get_matches();
    match matches.subcommand() {
        ("ping", Some(_)) => {
            let binance_general: BinanceGeneral = BinanceClient::new(None, None);
            let result = binance_general.ping();
            match result {
                Ok(response) => println!("{:?}", response),
                Err(e) => println!("Error: {}", e), // Error: https://www.binance.com/api/v1/ping: No such host is known. (os error 11001)
            }
        }
        ("servertime", Some(servertime_matches)) => {
            let binance_general: BinanceGeneral = BinanceClient::new(None, None);
            let result = binance_general.get_server_time();
            if let Ok(response) = result {
                let mut time_string = response.server_time.to_string();
                if servertime_matches.is_present("human") {
                    time_string =
                        NaiveDateTime::from_timestamp(((response.server_time / 1000) as i64), 0)
                            .to_string();
                }
                println!("Server Time: {}", time_string);
            } else if let Err(e) = result {
                println!("Error: {}", e);
            }
        }
        _ => unreachable!(),
    }
}
