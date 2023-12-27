use serde::Deserialize;

#[derive(Deserialize)]
pub struct Coord {
    pub lon: Option<f32>,
    pub lat: Option<f32>,
}

#[derive(Deserialize)]
pub struct WeatherDescription {
    pub id: Option<u32>,
    pub main: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
}

#[derive(Deserialize)]
pub struct Main {
    pub temp: Option<f32>,
    pub feels_like: Option<f32>,
    pub temp_min: Option<f32>,
    pub temp_max: Option<f32>,
    pub pressure: Option<u32>,
    pub humidity: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct Wind {
    pub speed: Option<f32>,
    pub deg: Option<u32>,
    pub gust: Option<f32>,
}

#[derive(Deserialize)]
pub struct Clouds {
    pub all: Option<u32>,
}

#[derive(Deserialize)]
pub struct Sys {
    #[serde(rename = "type")]
    pub type_: Option<u32>,
    pub id: Option<u32>,
    pub country: Option<String>,
    pub sunrise: Option<u64>,
    pub sunset: Option<u64>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Weather {
    pub coord: Coord,
    pub weather: Vec<WeatherDescription>,
    pub base: Option<String>,
    pub main: Main,
    pub visibility: Option<u32>,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: Option<u64>,
    pub sys: Sys,
    pub timezone: Option<i32>,
    pub id: Option<u32>,
    pub name: Option<String>,
    pub cod: Option<u32>,
}