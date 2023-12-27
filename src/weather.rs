use reqwest::Error;
use std::env;
//import models.rs
pub use crate::models::Weather;

impl Weather {
    pub fn get_weather(&self) -> String {
        format!(
            "City: {}\nTemp: {}\nFeels Like: {}\nDescription: {}\nWind Speed: {}\nWind Direction: {}\nHumidity: {}\nPressure: {}\nVisibility: {}",
            self.name.as_ref().unwrap_or(&"N/A".to_string()),
            self.main.temp.unwrap_or(0.0),
            self.main.feels_like.unwrap_or(0.0),
            self.weather.get(0).and_then(|w| w.description.as_ref()).unwrap_or(&"N/A".to_string()),
            self.wind.speed.unwrap_or(0.0),
            self.wind.deg.unwrap_or(0),
            self.main.humidity.unwrap_or(0),
            self.main.pressure.unwrap_or(0),
            self.visibility.unwrap_or(0)
        )
    }

    pub async fn fetch_weather(city: &str) -> Result<Weather, Error> {
        let api_key = env::var("OPENWEATHERMAP_API_KEY").expect("OPENWEATHERMAP_API_KEY must be set");
        let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&units=metric&appid={}", city, api_key);
        let response = reqwest::get(&url).await?.json::<Weather>().await?;
        Ok(response)
    }
}