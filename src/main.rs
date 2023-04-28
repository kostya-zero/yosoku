use clap::Command;
use config::{Manager, Config};

use crate::{config::{Units, ForecastView}, api::API};

mod config;
mod api;

fn cli() -> Command {
    Command::new("yosoku")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Konstantin Zhigaylo")
        .arg_required_else_help(true)
        .subcommands([
                     Command::new("now")
                     .about("Get forecast for this moment."),

                     Command::new("today")
                     .about("Get forecast for today"),

                     Command::new("tomorrow")
                     .about("Get forecast for today and tomorrow"),
                     
                     Command::new("next")
                     .about("Get Forecast for next 3 days.")
        ])
}

fn build_options(conf: &Config) -> String {
    let mut options_str: String = "".to_string();
    let mode = match &conf.forecast_view {
        Some(ForecastView::detailed) => "",
        Some(ForecastView::quite) => "q",
        Some(ForecastView::super_quite) => "Q",
        _ => ""
    };

    let metric = match &conf.units {
        Some(Units::us) => "u",
        Some(Units::metric) => "M",
        _ => "M"
    };

    options_str = options_str + mode + metric + "&lang=" + &conf.language.clone().expect("Failed to get language from config.").to_string();
    return options_str;
}

fn main() {
    let args = cli().get_matches();
    match args.subcommand() {
        Some(("now", _sub)) => {
            let conf = Manager::read_config();
            // println!("{:?}", conf); 
            let mut uri: String = "https://wttr.in/".to_string();
            uri = uri + &conf.location.clone().expect("Failed to get location from config.") + "?0AF";
            let req_opts: String = build_options(&conf);
            uri = uri + &req_opts;

            let result = API::send(&uri);
            println!("{}", result);
        },
        Some(("today", _sub)) => {
            let conf = Manager::read_config();
            // println!("{:?}", conf); 
            let mut uri: String = "https://wttr.in/".to_string();
            uri = uri + &conf.location.clone().expect("Failed to get location from config.") + "?1AF";
            let req_opts: String = build_options(&conf);
            uri = uri + &req_opts;

            let result = API::send(&uri);
            println!("{}", result);
        },
        Some(("tomorrow", _sub)) => {
            let conf = Manager::read_config();
            // println!("{:?}", conf); 
            let mut uri: String = "https://wttr.in/".to_string();

            uri = uri + &conf.location.clone().expect("Failed to get location from config.") + "?2FA";
            let req_opts: String = build_options(&conf);
            uri = uri + &req_opts;

            let result = API::send(&uri);
            println!("{}", result);
        },
        Some(("next", _sub)) => {
            let conf = Manager::read_config();
            // println!("{:?}", conf); 
            let mut uri: String = "https://wttr.in/".to_string();

            uri = uri + &conf.location.clone().expect("Failed to get location from config.") + "?AF";
            let req_opts: String = build_options(&conf);
            uri = uri + &req_opts;

            let result = API::send(&uri);
            println!("{}", result);
        },

        _ => println!("Unknown command!")
    }
}
