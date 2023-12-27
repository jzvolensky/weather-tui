
use std::env;

//import models.rs
pub use crate::models::Weather;

impl Weather {
    pub fn get_weather(&self) -> Vec<(String, String)> {
        let mut weather_data = Vec::new();
        weather_data.push(("City".to_string(), self.name.clone().unwrap_or("N/A".to_string())));
        weather_data.push(("Temp".to_string(), format!("{} °C", self.main.temp.unwrap_or(0.0))));
        weather_data.push(("Feels Like".to_string(), format!("{} °C", self.main.feels_like.unwrap_or(0.0))));
        weather_data.push(("Description".to_string(), self.weather.get(0).and_then(|w| w.description.clone()).unwrap_or("N/A".to_string())));
        weather_data.push(("Wind Speed".to_string(), format!("{} m/s", self.wind.speed.unwrap_or(0.0))));
        weather_data.push(("Wind Direction".to_string(), format!("{} degrees", self.wind.deg.unwrap_or(0))));
        weather_data.push(("Humidity".to_string(), format!("{}%", self.main.humidity.unwrap_or(0))));
        weather_data.push(("Pressure".to_string(), format!("{} hPa", self.main.pressure.unwrap_or(0))));
        weather_data.push(("Visibility".to_string(), format!("{} m", self.visibility.unwrap_or(0))));
        weather_data
    }

    pub async fn fetch_weather(city: &str) -> Result<Weather, Box<dyn std::error::Error>> {
        let api_key = env::var("OPENWEATHERMAP_API_KEY")?;
        let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city, api_key);
        let resp = reqwest::get(&url).await?;
    
        if !resp.status().is_success() {
            return Err(format!("Failed to fetch weather data for city: {}", city).into());
        }
    
        let weather: Weather = resp.json().await?;
        Ok(weather)
    }
}