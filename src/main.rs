use clap::Command;
use config::Manager;

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

fn main() {
    let args = cli().get_matches();
    match args.subcommand() {
        Some(("now", _sub)) => {
            let conf = Manager::read_config();
            // println!("{:?}", conf); 
            let mut uri: String = "https://wttr.in/".to_string();

            uri = uri + &conf.location.expect("Failed to get location from config.") + "?0AF";

            let mode = match &conf.forecast_view {
                Some(ForecastView::detailed) => "",
                Some(ForecastView::minimalist) => "q",
                Some(ForecastView::super_minimalist) => "Q",
                _ => ""
            };
            
            let metric = match &conf.units {
                Some(Units::us) => "u",
                Some(Units::metric) => "M",
                _ => "M"
            };

            uri = uri + mode + metric;
            uri = uri + "&lang=" + &conf.language.expect("Failed to get language from config.").to_string();

            let result = API::send(&uri);
            println!("{}", result);
        },
        Some(("today", _sub)) => {
            let conf = Manager::read_config();
            // println!("{:?}", conf); 
            let mut uri: String = "https://wttr.in/".to_string();

            uri = uri + &conf.location.expect("Failed to get location from config.") + "?1FA";

            let mode = match &conf.forecast_view {
                Some(ForecastView::detailed) => "",
                Some(ForecastView::minimalist) => "q",
                Some(ForecastView::super_minimalist) => "Q",
                _ => ""
            };
            
            let metric = match &conf.units {
                Some(Units::us) => "u",
                Some(Units::metric) => "M",
                _ => "M"
            };

            uri = uri + mode + metric;
            uri = uri + "&lang=" + &conf.language.expect("Failed to get language from config.");

            let result = API::send(&uri);
            println!("{}", result);
        },
        Some(("tomorrow", _sub)) => {
            let conf = Manager::read_config();
            // println!("{:?}", conf); 
            let mut uri: String = "https://wttr.in/".to_string();

            uri = uri + &conf.location.expect("Failed to get location from config.") + "?2FA";

            let mode = match &conf.forecast_view {
                Some(ForecastView::detailed) => "",
                Some(ForecastView::minimalist) => "q",
                Some(ForecastView::super_minimalist) => "Q",
                _ => ""
            };
            
            let metric = match &conf.units {
                Some(Units::us) => "u",
                Some(Units::metric) => "M",
                _ => "M"
            };

            uri = uri + mode + metric;
            uri = uri + "&lang=" + &conf.language.expect("Failed to get language from config.");

            let result = API::send(&uri);
            println!("{}", result);
        },
        Some(("next", _sub)) => {
            let conf = Manager::read_config();
            // println!("{:?}", conf); 
            let mut uri: String = "https://wttr.in/".to_string();

            uri = uri + &conf.location.expect("Failed to get location from config.") + "?AF";

            let mode = match &conf.forecast_view {
                Some(ForecastView::detailed) => "",
                Some(ForecastView::minimalist) => "q",
                Some(ForecastView::super_minimalist) => "Q",
                _ => ""
            };
            
            let metric = match &conf.units {
                Some(Units::us) => "u",
                Some(Units::metric) => "M",
                _ => "M"
            };

            uri = uri + mode + metric;
            uri = uri + "&lang=" + &conf.language.expect("Failed to get language from config.");

            let result = API::send(&uri);
            println!("{}", result);
        },

        _ => println!("Unknown command!")
    }
}
