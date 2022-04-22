use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
    temp: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Current {
    temp_c: f32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ApiResponse {
    WeatherApi { current: Current },
    OpenWeather { main: Main },
}

impl ApiResponse {
    pub fn temp(&self) -> f32 {
        match self {
            Self::WeatherApi { current } => current.temp_c,
            Self::OpenWeather { main } => main.temp,
        }
    }
}
