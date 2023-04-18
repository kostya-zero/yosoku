use toml::{self, from_str};
use serde::{Serialize, Deserialize};
use home::home_dir;
use std::{fs::read_to_string, path::Path};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Units {
    us,
    metric
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum ForecastView {
    detailed,
    minimalist,
    super_minimalist
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    pub location: Option<String>,
    pub units: Option<Units>,
    pub forecast_view: Option<ForecastView>,
    pub language: Option<String>
}

impl Default for Config {
    fn default() -> Self {
        Config { 
            location: Some("".to_string()), 
            units: Some(Units::metric), 
            forecast_view: Some(ForecastView::detailed), 
            language: Some("en".to_string()) 
        }
    }
}

pub struct Manager;
impl Manager {
    pub fn get_config_path() -> String {
        home_dir().expect("Error").display().to_string() + "/.config/yosoku.toml"
    }

    pub fn read_config() -> Config {
        let path = Self::get_config_path();
        if !Path::new(&path).exists() {
            let cfg_struct: Config = Config { ..Default::default() };
            return cfg_struct;
        }
        let content: String = read_to_string(Self::get_config_path()).expect("Error");
        let cfg_struct: Config = from_str(&content).expect("Error");
        return cfg_struct;
    }
}
